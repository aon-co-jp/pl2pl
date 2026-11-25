> 안내: 저장소 루트의 일본어 PORTING.md가 원문입니다. 이 문서는 AI가 쉬운 말로 번역하고, 다시 AI가 원어민 수준으로 자연스러운지 검수했습니다. 사람 원어민의 확인은 아직 받지 않았습니다.

# pl2pl 인수인계
(지난 자세한 기록은 `docs/HISTORY-ja.md`에 있습니다. 일본어입니다.)

## 지금 상태(2026-11-19)
- v0.1.0 공개. Windows·Linux·Mac 설치 프로그램과 CI 테스트가 있습니다.
- PHP → FastAPI는 실제 FastAPI로 종단 간 테스트를 마쳤습니다: 라우트, Eloquent, 쿼리 빌더, Blade, 미들웨어, 관계, `@can` 계열.
- `pl2pl web`이 이제 PHP도 알아봅니다.
- `pl2pl watch`가 새 버전을 찾고, 테스트하고, 고치고, 안 되면 알려 줍니다. GitHub에서도 매일 밤과 아침에 최신 버전으로 테스트합니다.
- `pl2pl ir`(시험 단계)이 PHP, Python, Java(Spring, JAX-RS, Servlet, Spark)의 라우트를 공통 모양으로 만들어 JSON, OpenAPI, FastAPI 뼈대를 냅니다. IR은 타입이 있는 인자, 함수 본문, 모델, 미들웨어, 템플릿도 담고, `pl2pl php`는 이제 `src/ir_emit.rs`의 공통 출력기를 거쳐 씁니다(출력은 이전과 같음, 통합 시험 49건 통과).
- 소개·다운로드 페이지 공개: <https://easy-web.tokyo/pl2pl/>(VPS, 정적). easy-web.tokyo 첫 화면에도 카드가 있습니다.
- COBOL_to와 Java_to는 `converters/`에 합쳤습니다.

## 아직 남은 일(중요한 순)
1. **확인**: 실제 이메일 도착(`version-control --test-notify`. 이 PC에는 SMTP 설정이 없음), version-control `--loop`로 하는 자동 수정, 최신 fastapi로 테스트(GitHub의 매일 밤 실행이 확인해 줌).
2. IR에 자료형과 함수 내용을 담기. 지금의 FastAPI 출력(`pl2pl php`)을 IR을 거치게 하기. 라우트 찾기에서 놓치는 것이 있으면 tree-sitter를 검토.
3. 진짜 새 버전으로 `watch`를 시험함: fastapi 0.165.0(starlette 1.10)에서 `tests/run-php.sh`가 49개 통과해서 registry의 시험 완료 버전을 0.165로 올렸습니다. 남은 것: 새 laravel 13 / django 6.1 / spring-boot 4.1은 통합 시험이 실제 프레임워크를 쓰지 않아서 '미확인'으로 남습니다(실물을 쓰는 시험을 만들 때까지는 보고만 합니다).
4. 컴퓨터에서 매일 밤·아침 자동 실행 등록(아직 안 함: 이 PC에서는 확인 명령이 돌지 않아 오경보가 남. 예시는 `scripts/pl2pl-watch.cmd` 맨 위).
5. (완료) easy-web.tokyo 첫 화면에 pl2pl 카드를 넣어 실서비스에 반영했습니다(open-easy-web `95cb1e7`, 이전 바이너리는 `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP(완료, 한계 있음): Blade `@php`가 대입과 `++` `--` `+=` `-=` `*=` `/=` `.=`를 `{% set %}`으로 바꿉니다. `$attributes`는 출력, `->merge`(문자열 값 또는 변환 가능한 식), `->class([...])`(문자열 목록), `->only`, `->except`가 동작합니다. `morphToMany`(중간 테이블의 타입 열)와 클래스로 만든 Blade 컴포넌트(`app/View/Components/*.php`: 생성자 인자가 기본값과 `_props`가 되고, `render()` 안의 `view('x.y')`도 따라감)도 지원합니다(통합 시험 55건, 단위 시험 41건). 남은 것: 컴포넌트 클래스의 공개 메서드(템플릿의 `{{ $method() }}`는 TODO로 남고 보고서에 개수를 냄), 키가 있는(조건부) `->class()`, if/foreach나 함수 호출이 있는 `@php`(TODO로 남음).
7. aruaru-llm 연동(완료): 2026-11-19에 aruaru-llm v0.2.5를 가져와 실제 `pl2pl llm setup`을 실행했습니다. 자동 선택인 3B는 가중치가 여러 조각(샤드)으로 나뉘어 있어 실패했으므로, 이제 설치는 한 단계 작은 모델로 물러납니다(모델을 직접 지정한 경우는 제외). `GenerateQwen`은 qwen2.5-1.5b-instruct(약 2.9GB, CPU)로 글을 만들어 냅니다. **`pl2pl php --explain`**(선택 사항, `--explain-max N`, 기본 3)은 보고서의 각 TODO 항목 아래에 AI 메모를 붙입니다. 실측: CPU에서 항목당 약 2분이고, **품질이 낮아 틀릴 때도 있어**(2건 중 1건이 빗나감) 기본값은 끔입니다. 다시 시작한 aruaru-llm은 모델이 선택되지 않은 상태(400)이므로, 먼저 설치된 모델을 고릅니다. **더 큰 모델(약 12GB)**: aruaru-llm 카탈로그에서 가장 큰 것은 3B(약 6.2GB)이고, 3B와 7B는 여러 샤드 파일로 배포되어 지금의 설치 프로그램으로는 받을 수 없습니다. 먼저 샤드 지원(aruaru-llm 쪽)과 CPU 속도(1.5B에서도 이미 이만큼 느림)가 필요합니다. GPU에서의 Kokkos와 open-cuda / open-directx 위의 커널은 아직 미확인입니다. **GPU(2026-11-19 확인)**: GeForce GT 730(2GB, sm_35, 드라이버 475 = CUDA 11.4급)에서 관리자 권한 없이 micromamba(NVIDIA의 conda 채널)로 CUDA 11.8 nvcc를 설치하고, 간단한 CUDA 커널과 `pl2pl gpu --target open-cuda`의 출력(`vec_loop12.cu`)을 실제 GPU에서 돌려 CPU와 0건의 불일치를 확인했습니다(`tests/gpu-check.ps1`). 함정: MSVC 14.44는 `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH`와 `-allow-unsupported-compiler`가 필요하고, PyPI에는 Windows용 nvcc가 없으며, CUDA 12는 sm_35를 대상으로 할 수 없습니다. **아직 미확인**: Kokkos CUDA 백엔드, 그리고 open-cuda(Rust 구현)와 open-directx에서의 실행.
8. 원래의 COBOL_to와 Java_to 삭제(**되돌릴 수 없으니 마지막 확인을 받은 뒤에**).
9. `docs/DESIGN.md`는 소유자가 씁니다. 2026-11-19에 `docs/LANGUAGES.md`를 다시 읽었고 현재 상태와 어긋난 곳은 없었습니다(Java와 COBOL에는 변환기가 있음). 아직 지식에 바탕을 둔 목록이며, 새로 웹 검색으로 확인한 것은 아닙니다.

## 테스트
`bash tests/run-all.sh`(WSL) / `bash tests/run-php.sh`(PHP 종단 간) / `cargo test --release`

## 다시 시작할 때 쓰는 메시지
"F:\pl2pl 재개. PORTING.md의 '아직 남은 일'을 위에서부터 진행. 바꿀 수 없는 부분은 몰래 지우지 말고 TODO로 남기기. 실제로 돌려 확인한 뒤에 끝났다고 보고. 답변은 일본어로만. 단계마다 커밋하고 push, VPS도 갱신."

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

## 2026-11-25 추가: c2rust의 배열과 break/continue 지원
`pl2pl c2rust`가 고정 길이 배열(`int a[8] = {...};`, `double w[4];`. 모자란 초깃값은 0으로 채움), 인덱스 읽기·쓰기(`a[j + 1]` → `a[(j + 1) as usize]`), `break;`, `continue;`, `while (1)` → `loop`를 지원합니다. 버블 정렬과 `while (1)`이 들어간 예제를 WSL의 gcc와 rustc로 돌려 출력이 완전히 일치했습니다(`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). 단위 테스트 56개 통과. 아직 안 되는 것: 포인터, 구조체, 2차원 배열, 배열을 함수에 넘기기, 정수와 소수의 암묵적 형변환(`k * 1.5`는 `(double)k`라고 써야 함).

<!-- directive-20261125-1130 -->
## 2026-11-25 지시(11:30까지 확인 불필요)
2026-11-25 11:30까지, 남은 일은 확인 없이 완성될 때까지 자동으로 진행합니다. 수요가 적거나 필요성이 낮다고 판단되더라도, 또는 처음부터 크게 다시 만들어야 하더라도 다른 방식으로 대신하지 않고 정면으로 다시 만듭니다. 동시에 개발하는 대상: COBOL_to, Java_to, MFastAPI, 2MFastAPI(다른 언어·프레임워크에서 MFastAPI 호환으로. Node.js와 TypeScript는 예외적으로 고를 수 있고, 고르지 않는 선택도 가능), RMojo, RPoRea(RPoem과 Reaxt의 통합), 2RPoRea(다른 언어·프레임워크에서 RPoRea로), Reaxt(React+Next.js+Nuxt.js+Nest.js의 통합). 하나를 바꾸면 다른 것도 확인하고, README, CLAUDE, PORTING, POLICY를 일본어와 영어로 같은 날 갱신합니다.

<!-- directive-20261125-mojo -->
## 2026-11-25 추가: MFastAPI는 Mojo에서 쓸 수 있어야 함
MFastAPI는 **Mojo에서 사용할 수 있는 사양**을 유지합니다(Python 구문, Rust 구현, Mojo 지원. Windows에서는 RMojo의 `compat/wsl-bridge`를 거쳐 WSL의 Mojo를 사용). 2MFastAPI의 변환 결과도 Mojo에서 쓸 수 있어야 합니다. 현재 상태: `mojo/`는 참조 구현이며 단계 B·C의 서버는 실험적입니다. Mojo에서 호출해 보는 실제 환경 확인은, 확인한 범위만 "확인됨"이라고 씁니다. 11:30까지 확인 없이 자동으로 진행합니다.
