> 안내: 저장소 루트의 일본어 PORTING.md가 원문입니다. 이 문서는 쉬운 말로 쓴 것으로, AI가 번역하고 AI가 원어민 수준의 자연스러움을 검수했습니다. 사람 원어민의 검수는 아직 받지 않았습니다.

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
