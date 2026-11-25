> 안내: 저장소 루트의 일본어 CLAUDE.md가 원문입니다. 이 문서는 AI가 쉬운 말로 번역하고, 다시 AI가 원어민 수준으로 자연스러운지 검수했습니다. 사람 원어민의 확인은 아직 받지 않았습니다.

# pl2pl 개발 메모
- **역할**: 언어·프레임워크·운영체제 사이에서 코드를 바꿔 쓰는 도구(Rust로 제작, `src/`). 원본 언어 목록은 `docs/LANGUAGES.md`, 조합은 `docs/COMBINATIONS.md`. 설계는 소유자가 `docs/DESIGN.md`에 조금씩 씁니다. **마음대로 사양을 정하지 마세요.**
- **방향**: 널리 알려진 언어 사이의 "다대다" 변환을 목표로 합니다. 한꺼번에 만들지 않고 고른 것부터 합니다. 웹사이트에는 설명과 다운로드만 있고, 변환은 PC 앱에서 합니다.
- **규칙**: 바꿀 수 없는 부분은 `TODO`로 남기고 보고합니다(몰래 지우지 않기). 신중하게(의존성이나 부작용이 걱정되면 바꾸지 않기). 실제로 돌려 본 뒤에 "끝났다"고 말하기. 새 REST는 만들지 않기. 답변은 일본어로만.
- **공통 모양(IR)**: `src/ir.rs`(타입, 함수 본문, 모델, 미들웨어, 템플릿)와 쓰는 쪽인 `src/ir_emit.rs`. 입력 쪽(PHP, Python, Java)과 출력 쪽(JSON, OpenAPI, FastAPI)을 나눠서 새 조합을 쉽게 늘립니다. `pl2pl php`도 같은 쓰기 부분을 거칩니다. 출력이 바뀌면 안 되는 수정은 수정 전후 출력의 diff가 비어 있는 것으로 확인합니다. 라우트를 못 찾은 파일은 보고합니다. 출력은 `py_compile` 등으로 실제로 확인합니다.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. 종단 간 테스트: `tests/run-php.sh`(실제 FastAPI, SQLite, Jinja2). 단위 테스트: `cargo test --release`. WSL에서는 `wsl -d Ubuntu -- bash <file>`.
- **자동 따라가기**: `pl2pl watch`(`src/watch.rs`, `registry/ecosystem.toml`). 확인 명령이 비어 있는 항목은 "미확인"으로만 표시하고 스스로 올리지 않습니다. 알림은 version-control의 `--notify-file`로 보냅니다.
- **배포**: 소개 페이지는 `webpage/index.html`. VPS에서는 `/root/easy-web.tokyo/pl2pl`(`git pull`로 갱신, 정적이라 재시작 불필요), `pl2pl-web.service` 포트 8142, `domains.toml`에 추가해 두었습니다.
- **함정**: 일본어가 든 파일은 Python `-X utf8`로 쓰기. 셸 스크립트는 LF. Git Bash는 `/mnt/...` 경로를 바꿔 버립니다. 따옴표나 `\`가 많은 글은 heredoc 대신 Write / Edit 도구로.
- **COBOL_to / Java_to**: `converters/`에 합쳤습니다. 원래 저장소를 지우는 것은 되돌릴 수 없으니 마지막 확인을 받은 뒤에.
- **번역 시스템**(사람의 언어)은 변환기와 분리합니다. 변환기는 LLM에 기대지 않습니다.
- **문서 언어**: 일본어 파일이 원문입니다. 영어 파일은 쉬운 말로 다시 썼습니다. `docs/i18n/`의 나머지 8개 언어는 같은 내용을 AI가 번역하고 AI가 자연스러움을 검수한 것입니다(사람 원어민 검수 전). 지난 경과는 `docs/HISTORY-ja.md`.

## v0.2.1의 새 소식 (2026-11-20)
- `pl2pl web --backend`에서 `rpoem` / `rporea`를 고르면 안내가 나옵니다(일본어와 영어): RPoem은 Tauri 호환, WunderGraph Cosmo 호환, tokio, WASM을 이미 포함하므로 따로 고르지 마세요. HTML5와 CSS3는 쓰지만 최소한으로, JavaScript·TypeScript·Node.js·REST는 설계상 쓰지 않습니다. `--also`로 추가한 것이 이미 포함돼 있으면 거절됩니다(종료 코드 2). 포함돼 있지만 아직 완성도가 낮은 것은 주의 문구와 함께 고를 수 있습니다.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]`는 탐침 시험을 실행해 그 완성도를 판단합니다(대장: `registry/maturity.toml`). aruaru-llm의 AI 의견만으로는 판정이 바뀌지 않습니다.
- RPoem / RPoRea 이외의 조합에서 `--also nodejs`(또는 `next.js`)는 안내와 함께 받아들여집니다. 'Node.js를 쓰지 않는다'는 방침을 강요하지 않지만 권장하지도 않습니다. 고른 언어에 맞는 대안(RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js, 그리고 우리가 확인하지 못한 다른 것들)을 보여 줍니다.
- 함께 개발하는 자매 저장소: Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js, pl2pl. 하나를 바꾸면 같은 날 다른 저장소도 확인합니다.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]`와 `webpage/info.html`이 RPoRea / RPoem의 설계 사상을 장단점과 함께 11개 언어로 설명합니다.
- 원래의 `COBOL_to` / `Java_to`는 남겨 둡니다(삭제하지 않음). `converters/`에 합친 사본은 출력이 완전히 같았습니다.

## 새로운 내용 (2026-11-22 ~ 2026-11-23)
- COBOL(`converters/cobol_to`): `SORT`(`USING` / `GIVING`, `INPUT` / `OUTPUT PROCEDURE`, `RELEASE`, `RETURN`), `MERGE`, `SEARCH` / `SEARCH ALL`, `INSPECT ... LEADING` / `CHARACTERS`를 추가했습니다. `COMP-3`와 `COMP` 사용 형식도 읽을 수 있습니다.
- 내장 SQL: `EXEC SQL`(커서, `FETCH`, `SELECT ... INTO`), `WHENEVER`, 지시 변수(`INDICATOR` 키워드 형식 포함), `SQLSTATE`, `EXECUTE IMMEDIATE`, `PREPARE` / `EXECUTE ... USING`을 지원합니다. 생성된 Python은 실제로 실행해 확인했고, Rust 쪽은 가짜 핸들러로만 확인했습니다.
- `EXEC CICS`(최소 지원): `SEND TEXT`는 글자를 표시하고, `RETURN` / `ABEND`는 프로그램을 끝냅니다. 다른 명령은 `TODO`로 남깁니다. BMS 화면 맵은 재현하지 않습니다.
- Java(`converters/java_to`): 메서드 참조, `Comparator`, `Collectors`, 익명 클래스(모든 메서드를 유지)를 지원합니다. Rust 쪽에서는 List 연산(`Vec`, `push`, 인덱스 접근, `for`)이 되고, 생성된 Rust는 spring, jaxrs, servlet, spark 모두 `cargo check`를 통과했습니다.
- Spring PetClinic REST(OpenAPI에서 출발하는 프로젝트)로 시험한 결과: 찾은 경로가 1개에서 37개로 늘었고, 생성된 Python의 구문 오류는 0개이며, 파생 쿼리(`Or`, `Like`, 비교, `OrderBy` 등)가 메모리 저장소에서 실행되고, 앱을 import할 수 있으며(경로 41개), `this.field` 이름을 맞춰서 호출이 컨트롤러 → 서비스 → 저장소까지 진행됩니다.
- 아직 남은 일: 저장소와 MapStruct 매퍼 생성(호출이 `TODO`에서 멈춤), 실제 데이터베이스, 생성된 FastAPI의 실제 실행, CICS 맵과 파일, Map이나 Stream이 들어간 Rust 본문. pl2pl 테스트: 55개 통과.

## 2026-11-25 현황·남은 일

현황: 오래된 프로그램(COBOL, C, C++, Fortran, Java 등)을 선택한 언어와 프레임워크로 다시 써 주는 도구입니다. 최근 Java_to의 PetClinic 지원(ResponseEntity, Mapper, 상위 데이터 클래스)을 가져오고 다국어 문서를 동기화했습니다. 남은 일: 실제 Mac과 iPhone에서의 확인(여기에는 해당 기기가 없습니다). 번역: AI 번역이며 원어민의 확인은 아직 받지 않았습니다.

## 2026-11-25 추가: 변환기 동기화
`converters/cobol_to`에 COBOL_to의 EXEC CICS LINK/XCTL → CALL(`7d67bbf`)을, `converters/java_to`에 Java_to의 삼항 연산자(Rust 함수 본문, `e9ac300`)를 가져왔습니다. 시험: Java_to 39개, COBOL_to 25개, pl2pl 55개, 모두 통과했습니다. 남은 일: Mac·iPhone 실제 기기, 생성한 FastAPI를 실제 DB로 띄워 보는 확인.

## 2026-11-25 추가: 구현이 여러 개일 때의 선택 방식을 시험으로 고정
저장소 인터페이스에 구현이 여러 개 있을 때, 메모리에서 동작하는 것(이름에 Jdbc·Jpa가 들어가지 않는 것)을 우선하고 없으면 맨 앞의 것을 고르는 처리는 이미 구현돼 있었습니다(이전의 "미지원"이라는 설명은 낡은 내용입니다). 단위 테스트 `multiple_impls_prefer_non_db_one`으로 고정했습니다(Java_to 40개 통과). 남은 일: MapStruct의 복잡한 매퍼, 실제 DB로 띄워 보는 확인.

## 2026-11-25 추가: MapStruct의 @Mapping 지원
매퍼 메서드에 붙인 `@Mapping(target=..., source="a.b")`(중첩된 원본 속성), `ignore=true`, `constant="..."`를 생성하는 Python의 `MapperBase._to(cls, src, rename, ignore, const)`로 넘기게 했습니다. 단위 테스트 `mapstruct_mapping_annotations`(Java_to 41개 통과)와 실행 시 동작 확인(`owner.name` → `fullName`, ignore, 상수)을 했습니다. 남은 일: `@Mappings({...})`의 중첩 표기, `expression=`, `@MappingTarget`, 실제 DB로 띄워 보는 확인.

<!-- directive-20261125-1130 -->
## 2026-11-25 지시(11:30까지 확인 불필요)
2026-11-25 11:30까지, 남은 일은 확인 없이 완성될 때까지 자동으로 진행합니다. 수요가 적거나 필요성이 낮다고 판단되더라도, 또는 처음부터 크게 다시 만들어야 하더라도 다른 방식으로 대신하지 않고 정면으로 다시 만듭니다. 동시에 개발하는 대상: COBOL_to, Java_to, MFastAPI, 2MFastAPI(다른 언어·프레임워크에서 MFastAPI 호환으로. Node.js와 TypeScript는 예외적으로 고를 수 있고, 고르지 않는 선택도 가능), RMojo, RPoRea(RPoem과 Reaxt의 통합), 2RPoRea(다른 언어·프레임워크에서 RPoRea로), Reaxt(React+Next.js+Nuxt.js+Nest.js의 통합). 하나를 바꾸면 다른 것도 확인하고, README, CLAUDE, PORTING, POLICY를 일본어와 영어로 같은 날 갱신합니다.

<!-- directive-20261125-mojo -->
## 2026-11-25 추가: MFastAPI는 Mojo에서 쓸 수 있어야 함
MFastAPI는 **Mojo에서 사용할 수 있는 사양**을 유지합니다(Python 구문, Rust 구현, Mojo 지원. Windows에서는 RMojo의 `compat/wsl-bridge`를 거쳐 WSL의 Mojo를 사용). 2MFastAPI의 변환 결과도 Mojo에서 쓸 수 있어야 합니다. 현재 상태: `mojo/`는 참조 구현이며 단계 B·C의 서버는 실험적입니다. Mojo에서 호출해 보는 실제 환경 확인은, 확인한 범위만 "확인됨"이라고 씁니다. 11:30까지 확인 없이 자동으로 진행합니다.

## Versionless API 방침 (2026-11-25)
이 저장소의 API는 **Versionless**입니다(URL이나 패키지 이름에 `v1` 등을 붙이지 않고, 호환성을 깨지 않으면서 더하기만 해서 발전시킵니다). 필드는 더하기만 하고, 더하는 필드는 생략할 수 있게 하며, 모르는 필드는 무시합니다. 이름을 바꿀 때는 새 이름을 더하고 옛 이름을 폐기 예정으로 둡니다(기한은 기본 6개월 이상, 사용이 0인 것을 확인한 뒤 삭제). enum은 모르는 값도 허용하고, 보안상 긴급 수정은 예외입니다. 기존 `.v1`은 호출하는 쪽이 깨지지 않도록 당분간 남겨 두고, 버전 없는 별칭을 더해 단계적으로 정리합니다. 자세한 내용과 출처는 [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md) 끝부분을 보세요. 적용은 앞으로 차례로 합니다.

## API 방침: REST 전면 폐지, gRPC / Connect로
이 저장소에서는 **REST API를 새로 만들지 않습니다**. 브라우저나 `curl`에서 부르는 API는 **Connect의 단순 호출**(`POST /<패키지>.<Service>/<Method>`, `content-type: application/json`)로 하고, 서비스 사이는 **gRPC / gRPC-web**으로 합니다. GraphQL은 읽기 전용입니다. 기존 REST는 손댈 때 옮기고, 옮기고 나면 옛 경로는 지웁니다. JSON은 RS-JSON(`from_str_secure` / `parse_secure`)으로 읽습니다. 예외는 상대방의 규격이 형태를 정하는 것(ACME, OAuth/OIDC 콜백, 제3자 Webhook, 헬스 체크, 정적 파일, OTLP)뿐입니다. 고민될 때는 Connect를 쓰세요. 원본은 [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md)입니다.
