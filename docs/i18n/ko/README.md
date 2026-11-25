> 안내: 저장소 루트의 일본어 README.md가 원문입니다. 이 문서는 AI가 쉬운 말로 번역하고, 다시 AI가 원어민 수준으로 자연스러운지 검수했습니다. 사람 원어민의 확인은 아직 받지 않았습니다.

# pl2pl

## pl2pl이 뭐예요?
프로그램은 여러 가지 "언어"(프로그래밍 언어)로 쓰여 있습니다.
**pl2pl은 어떤 언어로 쓴 프로그램을 다른 언어로 바꿔 쓰는 도구입니다.**
예를 들어 PHP(Laravel)로 만든 사이트를 Python(FastAPI) 사이트로 바꿀 수 있습니다.
Windows, Linux, Mac에서 쓸 수 있습니다(v0.1.0, 아직 만드는 중입니다).

- 쓰려면 내 컴퓨터에 설치합니다. 웹사이트에는 설명과 다운로드만 있습니다 → <https://easy-web.tokyo/pl2pl/>
- 바꿀 수 없는 부분은 몰래 지우지 않습니다. `TODO`(나중에 할 일)로 남기고 이유를 보고서에 적습니다.
- 목표는 널리 알려진 언어끼리 가능한 한 많은 조합으로 바꿔 쓸 수 있게 하는 것입니다.

## 지금 할 수 있는 일
| 명령 | 하는 일 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony) → FastAPI. 자세한 내용은 [PHP.md](../../PHP.md)(일본어) |
| `pl2pl web` | 폴더 안을 살펴 Java, COBOL, PHP, Flask/Django를 알아보고 알맞은 도구에 맡김 |
| `pl2pl ir` | (시험 단계) PHP, Python, Java의 "경로(라우트)"를 같은 모양으로 만들어 JSON, OpenAPI, FastAPI 뼈대로 출력합니다. PHP 변환도 타입, 함수 본문, 모델, 미들웨어, 템플릿을 이 모양에 담은 뒤에 씁니다 |
| `pl2pl watch` | 언어와 도구의 새 버전을 찾아 스스로 따라감 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | C/C++ 반복문을 더 빠르게 돌아가게 함(gpu는 시험 단계) |
| `pl2pl cport` | UNIX용 C 코드를 Windows와 Linux 모두에서 돌아가게 함 |
| `pl2pl pyhpc` | Numba로 Python 계산을 빠르게 함 |
| `pl2pl llm setup` | AI 도우미(aruaru-llm)를 설치합니다. 실제 설치는 아직 확인하지 못했습니다 |

## 새 버전을 스스로 따라가기
`pl2pl watch`를 매일 밤과 아침에 실행합니다.
1. 언어와 도구의 새 버전이 나왔는지 확인합니다(목록은 `registry/ecosystem.toml`).
2. 새 버전으로 테스트합니다. 통과하면 "지원하는 버전"을 스스로 새로 고칩니다.
3. 통과하지 못하면 version-control(AI의 도움)이 고쳐 봅니다.
4. 그래도 안 되면 저장해 둔 이메일 주소와 LINE으로 알려 줍니다.

참고: laravel, symfony, flask, django 등은 지금은 "새 버전이 나왔어요"라고 알려 주기만 합니다.
GitHub에서는 매일 밤과 아침에 최신 버전으로 테스트가 자동으로 돌아갑니다.

## 사용법
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 알아 두세요
- pl2pl은 신중합니다. 위험해 보이는 부분은 그대로 두고 이유를 알려 줍니다. 속도는 약속할 수 없습니다.
- 작은 예제로만 시험했습니다. 크고 실제적인 프로그램에서는 아직 해 보지 않았습니다.

## 앞으로
- 자료형과 함수의 내용도 공통 모양에 담고, FastAPI 변환도 그것을 거치게 하기.
- PHP: 클래스로 만든 Blade 부품.
- 실제 이메일 알림과 AI 자동 수정 확인.
- GPU에서 돌아가는지 확인(open-cuda / open-directx).

## v0.2.0의 새 기능
- 마음대로 조합: `pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`. COBOL은 다시 쓰고, C·C++·Fortran은 그대로 실행하며 웹에서 부릅니다.
- `pl2pl c2rust`(단순한 C와 C++)와 `pl2pl f2rust`(단순한 Fortran)는 Rust를 만듭니다. `pl2pl cobol --os <linux|macos|unix|windows>`는 GnuCOBOL 빌드 스크립트를 만듭니다.

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

*일본어와 영어가 기본이고, 다른 언어는 AI 번역입니다(원어민 확인 전).*

현황: 오래된 프로그램(COBOL, C, C++, Fortran, Java 등)을 선택한 언어와 프레임워크로 다시 써 주는 도구입니다. 최근 Java_to의 PetClinic 지원(ResponseEntity, Mapper, 상위 데이터 클래스)을 가져오고 다국어 문서를 동기화했습니다. 남은 일: 실제 Mac과 iPhone에서의 확인(여기에는 해당 기기가 없습니다). 번역: AI 번역이며 원어민의 확인은 아직 받지 않았습니다.


## 더 알아보기
[LANGUAGES.md](../../LANGUAGES.md)(언어 목록) / [COMBINATIONS.md](../../COMBINATIONS.md)(조합) / [DESIGN.md](../../DESIGN.md)(설계, 소유자가 작성 중) / [components/README.md](../../../components/README.md)(함께 설치되는 부품). 이 파일들은 일본어입니다.
