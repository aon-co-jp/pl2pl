> 안내: 저장소 루트의 일본어 PORTING.md가 원문입니다. 이 문서는 쉬운 말로 쓴 AI 번역이며, 아직 원어민의 검수를 받지 않았습니다.

# pl2pl 인수인계
(지난 자세한 기록은 `docs/HISTORY-ja.md`에 있습니다. 일본어입니다.)

## 지금 상태(2026-11-19)
- v0.1.0 공개. Windows·Linux·Mac 설치 프로그램과 CI 테스트가 있습니다.
- PHP → FastAPI는 실제 FastAPI로 종단 간 테스트를 마쳤습니다: 라우트, Eloquent, 쿼리 빌더, Blade, 미들웨어, 관계, `@can` 계열.
- `pl2pl web`이 이제 PHP도 알아봅니다.
- `pl2pl watch`가 새 버전을 찾고, 테스트하고, 고치고, 안 되면 알려 줍니다. GitHub에서도 매일 밤과 아침에 최신 버전으로 테스트합니다.
- `pl2pl ir`(시험 단계)이 PHP, Python, Java(Spring, JAX-RS, Servlet, Spark)의 라우트를 공통 모양으로 만들어 JSON, OpenAPI, FastAPI 뼈대를 냅니다. 출력이 실제로 Python에서 읽히는 것을 확인했습니다.
- 소개·다운로드 페이지 공개: <https://easy-web.tokyo/pl2pl/>(VPS, 정적).
- COBOL_to와 Java_to는 `converters/`에 합쳤습니다.

## 아직 남은 일(중요한 순)
1. **확인**: 실제 이메일 도착(`version-control --test-notify`. 이 PC에는 SMTP 설정이 없음), version-control `--loop`로 하는 자동 수정, 최신 fastapi로 테스트(GitHub의 매일 밤 실행이 확인해 줌).
2. IR에 자료형과 함수 내용을 담기. 지금의 FastAPI 출력(`pl2pl php`)을 IR을 거치게 하기. 라우트 찾기에서 놓치는 것이 있으면 tree-sitter를 검토.
3. 진짜 새 버전으로 `watch`를 시험함: fastapi 0.165.0(starlette 1.10)에서 `tests/run-php.sh`가 49개 통과해서 registry의 시험 완료 버전을 0.165로 올렸습니다. 남은 것: 새 laravel 13 / django 6.1 / spring-boot 4.1은 통합 시험이 실제 프레임워크를 쓰지 않아서 '미확인'으로 남습니다(실물을 쓰는 시험을 만들 때까지는 보고만 합니다).
4. 컴퓨터에서 매일 밤·아침 자동 실행 등록(아직 안 함: 이 PC에서는 확인 명령이 돌지 않아 오경보가 남. 예시는 `scripts/pl2pl-watch.cmd` 맨 위).
5. easy-web.tokyo 첫 화면에 pl2pl 소개 카드 추가.
6. PHP: 클래스로 만든 Blade 부품, `$attributes`, `@php`, `morphToMany`.
7. aruaru-llm 연동(`llm setup` 실제 설치는 미확인). Kokkos GPU 실행, open-cuda / open-directx에서 커널 실행.
8. 원래의 COBOL_to와 Java_to 삭제(**되돌릴 수 없으니 마지막 확인을 받은 뒤에**).
9. `docs/DESIGN.md`는 소유자가 씁니다. `docs/LANGUAGES.md`가 오래되지 않았는지 확인.

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
