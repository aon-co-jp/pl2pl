> Nota: el PORTING.md en japonés de la raíz del repositorio es el original. Esta versión fue traducida con IA y revisada por una IA para que el español suene natural; todavía no la ha comprobado ninguna persona hablante nativa.

# pl2pl – traspaso
(Las notas detalladas anteriores están en `docs/HISTORY-ja.md`, en japonés.)

## Dónde estamos (2026-11-19)
- La versión 0.1.0 está publicada, con instaladores para Windows, Linux y Mac y pruebas en CI.
- PHP → FastAPI está probado de extremo a extremo con FastAPI real: rutas, Eloquent, constructor de consultas, Blade, middleware, relaciones y la familia `@can`.
- `pl2pl web` ahora también reconoce PHP.
- `pl2pl watch` encuentra versiones nuevas, las prueba, arregla problemas y envía una alerta si nada funciona. GitHub también prueba con las versiones más recientes cada noche y cada mañana.
- `pl2pl ir` (versión de prueba) pone las rutas de PHP, Python y Java (Spring, JAX-RS, Servlet, Spark) en una forma común y genera JSON, OpenAPI o un esqueleto de FastAPI. La IR también lleva parámetros con tipo, contenido de las funciones, modelos, middleware y plantillas, y `pl2pl php` ahora escribe su salida con el emisor común de `src/ir_emit.rs` (salida sin cambios respecto a antes; 49 comprobaciones de extremo a extremo superadas).
- La página de presentación y descarga está en línea: <https://easy-web.tokyo/pl2pl/> (en el VPS, estática). También hay una tarjeta en la página principal de easy-web.tokyo.
- COBOL_to y Java_to están integrados en `converters/`.

## Pendiente (lo más importante primero)
1. **Comprobar**: que llegue un correo real (`version-control --test-notify`; este PC no tiene ajustes SMTP), la corrección automática con version-control `--loop` y una prueba con el último fastapi (la ejecución nocturna en GitHub lo comprobará).
2. Poner los tipos y el contenido de las funciones en la IR. Hacer pasar por la IR la salida actual de FastAPI (`pl2pl php`). Si la búsqueda de rutas se deja cosas, pensar en tree-sitter.
3. Se probó `watch` con una versión nueva real: con fastapi 0.165.0 (starlette 1.10), `tests/run-php.sh` pasó 49 comprobaciones, así que la versión probada del registro subió a 0.165. Pendiente: los nuevos laravel 13 / django 6.1 / spring-boot 4.1 siguen «sin comprobar», porque las pruebas de extremo a extremo no usan los frameworks reales (solo se informa hasta que existan esas pruebas).
4. Programar en un PC las ejecuciones de la noche y de la mañana (aún no hecho: el comando de comprobación no funciona en este PC y daría falsas alarmas; hay un ejemplo al principio de `scripts/pl2pl-watch.cmd`).
5. (Hecho) Hay una tarjeta de pl2pl en la página principal de easy-web.tokyo, ya en producción (open-easy-web `95cb1e7`; el binario antiguo es `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP (hecho, con límites): el `@php` de Blade ahora convierte asignaciones y `++` `--` `+=` `-=` `*=` `/=` `.=` en `{% set %}`; `$attributes` funciona como salida, y también `->merge` (valores de texto o expresiones convertibles), `->class([...])` (una lista de textos), `->only` y `->except`; `morphToMany` (columna de tipo en la tabla intermedia); y los componentes de Blade escritos como clases (`app/View/Components/*.php`: los argumentos del constructor pasan a ser valores por defecto y `_props`, y se sigue un `view('x.y')` dentro de `render()`) (55 comprobaciones de extremo a extremo, 41 pruebas unitarias). Pendiente: los métodos públicos de una clase de componente (un `{{ $method() }}` en su plantilla sigue como TODO, y el informe los cuenta), `->class()` con claves (condicional) y `@php` con if/foreach o llamadas a funciones (sigue como TODO).
7. Integración con aruaru-llm (hecha): el 2026-11-19 incorporamos aruaru-llm v0.2.5 y ejecutamos un `pl2pl llm setup` real. La elección automática, 3B, falló porque sus pesos están divididos en fragmentos (shards); por eso la instalación ahora recurre al siguiente modelo más pequeño (no cuando se fija uno). `GenerateQwen` devuelve texto con qwen2.5-1.5b-instruct (unos 2,9 GB, en CPU). **`pl2pl php --explain`** (opcional; `--explain-max N`, 3 por defecto) añade una nota de IA bajo cada punto TODO del informe. Medido: unos 2 minutos por punto en CPU, y **la calidad es pobre y a veces errónea** (1 de 2 no acertó), por eso está desactivado por defecto. Un aruaru-llm reiniciado no tiene modelo elegido (400), así que primero elige uno instalado. **Un modelo más grande (unos 12 GB)**: el mayor del catálogo de aruaru-llm es 3B (unos 6,2 GB), y 3B y 7B se entregan en varios archivos fragmento que el instalador actual no puede descargar. Primero van el soporte de fragmentos (en aruaru-llm) y la velocidad de CPU (ya es así de lenta con 1.5B). Kokkos en GPU y los núcleos en open-cuda / open-directx siguen sin comprobar. **GPU (comprobado el 2026-11-19)**: en una GeForce GT 730 (2 GB, sm_35, controlador 475 = clase CUDA 11.4) instalamos el nvcc de CUDA 11.8 sin permisos de administrador mediante micromamba (canal conda de NVIDIA), ejecutamos un núcleo CUDA sencillo y la salida de `pl2pl gpu --target open-cuda` (`vec_loop12.cu`) en la GPU real, y vimos 0 diferencias respecto a la CPU (`tests/gpu-check.ps1`). Trampas: MSVC 14.44 necesita `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` y `-allow-unsupported-compiler`; PyPI no tiene nvcc para Windows; CUDA 12 no puede apuntar a sm_35. **Sigue sin comprobar**: el backend CUDA de Kokkos y la ejecución en open-cuda (la implementación en Rust) y open-directx.
8. Borrar los COBOL_to y Java_to originales (**no se puede deshacer: pedir antes la aprobación final**).
9. El propietario escribe `docs/DESIGN.md`. Releímos `docs/LANGUAGES.md` el 2026-11-19 y no encontramos nada que no encaje con el estado actual (Java y COBOL tienen conversores). Sigue siendo una lista basada en conocimiento, no comprobada con una búsqueda web reciente.

## Pruebas
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de extremo a extremo) / `cargo test --release`

## Mensaje para retomar
«Retomar F:\pl2pl. Seguir «Pendiente» de PORTING.md desde arriba. Nunca esconder lo que no se pueda reescribir: dejar un TODO. Comprobar ejecutándolo de verdad antes de decir que está terminado. Responder solo en japonés. Hacer commit y push en cada paso y actualizar también el VPS.»

## Novedades de la v0.2.1 (2026-11-20)
- Si eliges `rpoem` / `rporea` en `pl2pl web --backend`, se muestran avisos (japonés e inglés): RPoem ya incluye funciones compatibles con Tauri y WunderGraph Cosmo, tokio y WASM, así que no los elijas aparte; HTML5 y CSS3 se usan, pero en poca cantidad; JavaScript, TypeScript, Node.js y REST no se usan por diseño. Lo que se añade con `--also` y ya está incluido se rechaza (código de salida 2); lo que está incluido pero aún no es maduro se puede elegir, con una advertencia.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` juzga esa madurez ejecutando pruebas de sondeo (registro: `registry/maturity.toml`). La opinión de la IA de aruaru-llm nunca cambia sola el veredicto.
- Con pilas distintas de RPoem / RPoRea, `--also nodejs` (o `next.js`) se acepta con un aviso: no imponemos nuestra regla de «sin Node.js», pero no lo recomendamos. Se listan alternativas según el lenguaje elegido (RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js y otras que no hemos verificado).
- Repositorios hermanos que se desarrollan juntos: Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js y pl2pl. Cuando uno cambia, los demás se revisan el mismo día.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` y `webpage/info.html` explican los principios de diseño de RPoRea / RPoem, con ventajas e inconvenientes, en 11 idiomas.
- Los originales `COBOL_to` / `Java_to` se conservan (no se borran); las copias integradas en `converters/` dieron una salida idéntica.

## Novedades (2026-11-22 a 2026-11-23)
- COBOL (`converters/cobol_to`): se añadieron `SORT` (`USING` / `GIVING`, `INPUT` / `OUTPUT PROCEDURE`, `RELEASE`, `RETURN`), `MERGE`, `SEARCH` / `SEARCH ALL` e `INSPECT ... LEADING` / `CHARACTERS`. Ahora también se leen los usos `COMP-3` y `COMP`.
- SQL embebido: `EXEC SQL` (cursores, `FETCH`, `SELECT ... INTO`), `WHENEVER`, variables indicadoras (incluida la forma con la palabra clave `INDICATOR`), `SQLSTATE`, `EXECUTE IMMEDIATE`, y `PREPARE` / `EXECUTE ... USING`. El Python generado se ejecutó de verdad; en el lado de Rust solo se comprobó con un manejador simulado.
- `EXEC CICS` (mínimo): `SEND TEXT` muestra texto, y `RETURN` / `ABEND` terminan el programa. Los demás comandos quedan como `TODO`. Los mapas BMS no se reproducen.
- Java (`converters/java_to`): referencias a métodos, `Comparator`, `Collectors` y clases anónimas (se conservan todos los métodos). En el lado de Rust funcionan las operaciones con List (`Vec`, `push`, acceso por índice, `for`), y el Rust generado pasa `cargo check` con spring, jaxrs, servlet y spark.
- Prueba con Spring PetClinic REST (un proyecto que parte de OpenAPI): las rutas encontradas pasaron de 1 a 37; el Python generado tiene 0 errores de sintaxis; las consultas derivadas (`Or`, `Like`, comparaciones, `OrderBy`, etc.) se ejecutan sobre un repositorio en memoria; la aplicación se puede importar (41 rutas); y los nombres de `this.field` ya coinciden, así que una llamada recorre controlador → servicio → repositorio.
- Pendiente: generar el repositorio y el mapeador MapStruct (las llamadas se detienen en un `TODO`), una base de datos real, una ejecución real del FastAPI generado, los mapas y archivos de CICS, y los cuerpos en Rust con Map o Stream. Pruebas de pl2pl: 55 superadas.

## 2026-11-25 Situación actual y pendientes

Estado: una herramienta que reescribe programas antiguos (COBOL, C, C++, Fortran, Java, etc.) en el lenguaje y el framework que elijas. Hace poco incorporó el soporte de PetClinic de Java_to (ResponseEntity, Mapper, clases de datos padre) y se sincronizó la documentación multilingüe. Pendiente: comprobar en un Mac y un iPhone reales (aquí no tenemos esos dispositivos). Traducciones: hechas por IA, aún sin revisión de hablantes nativos.

## 2026-11-25 Adenda: sincronización de los conversores
Se incorporaron a `converters/cobol_to` el paso de EXEC CICS LINK/XCTL a CALL de COBOL_to (`7d67bbf`) y a `converters/java_to` el operador ternario de Java_to en los cuerpos de Rust (`e9ac300`). Pruebas: Java_to 39, COBOL_to 25 y pl2pl 55, todas superadas. Pendiente: dispositivos Mac e iPhone reales y comprobar que el FastAPI generado arranca con una base de datos real.

## 2026-11-25 Adenda: la elección entre varias implementaciones, fijada con una prueba
Cuando la interfaz de un repositorio tiene varias implementaciones, el generador ya daba prioridad a la que funciona en memoria (sin Jdbc ni Jpa en el nombre) y, si no hay, tomaba la primera (la nota anterior de «no admitido» estaba desfasada). Ahora queda fijado con la prueba unitaria `multiple_impls_prefer_non_db_one` (Java_to: 40 superadas). Pendiente: mapeadores complejos de MapStruct y comprobar que arranca con una base de datos real.

## 2026-11-25 Adenda: soporte de @Mapping de MapStruct
`@Mapping(target=..., source="a.b")` (atributo de origen anidado), `ignore=true` y `constant="..."`, puestos en los métodos del mapeador, ahora se pasan a `MapperBase._to(cls, src, rename, ignore, const)` del Python generado. Se cubre con la prueba unitaria `mapstruct_mapping_annotations` (Java_to: 41 superadas) y con una comprobación en tiempo de ejecución (`owner.name` a `fullName`, ignore y constante). Pendiente: la forma anidada `@Mappings({...})`, `expression=`, `@MappingTarget` y comprobar que arranca con una base de datos real.

<!-- directive-20261125-1130 -->
## 2026-11-25 Instrucción (sin confirmación hasta las 11:30)
Hasta el 2026-11-25 a las 11:30, lo pendiente se lleva hasta el final de forma automática, sin pedir confirmación. Aunque parezca que hay poca demanda o poca necesidad, o aunque haya que rehacer mucho desde cero, no se sustituye por otro método: se rehace de frente. Se desarrollan a la vez: COBOL_to, Java_to, MFastAPI, 2MFastAPI (convierte otros lenguajes y frameworks a código compatible con MFastAPI; Node.js y TypeScript se pueden elegir como excepción, y también se puede no elegirlos), RMojo, RPoRea (integración de RPoem y Reaxt), 2RPoRea (convierte otros lenguajes y frameworks a RPoRea) y Reaxt (integración de React + Next.js + Nuxt.js + Nest.js). Cuando cambia uno, se revisan los demás, y README, CLAUDE, PORTING y POLICY se actualizan en japonés e inglés el mismo día.

<!-- directive-20261125-mojo -->
## 2026-11-25 Adenda: MFastAPI debe poder usarse desde Mojo
MFastAPI mantiene la **especificación de poder usarse desde Mojo** (sintaxis de Python, implementación en Rust, compatibilidad con Mojo; en Windows se usa el Mojo real de WSL a través de `compat/wsl-bridge` de RMojo). El resultado de 2MFastAPI también debe poder usarse desde Mojo. Estado actual: `mojo/` es una implementación de referencia, y los servidores de las etapas B y C son experimentales. Solo se escribe «comprobado» sobre lo que se ejecutó de verdad. Se continúa de forma automática, sin pedir confirmación, hasta las 11:30.

## 2026-11-25 Adenda: c2rust, arrays y break/continue
`pl2pl c2rust` ahora admite arrays (arreglos) de longitud fija (`int a[8] = {...};` y `double w[4];`; los valores iniciales que faltan se rellenan con 0), lectura y escritura por índice (`a[j + 1]` pasa a `a[(j + 1) as usize]`), `break;`, `continue;` y `while (1)` como `loop`. Un ejemplo con ordenación de burbuja y `while (1)` se compiló con gcc y rustc en WSL y las salidas coinciden por completo (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). Pruebas unitarias: 56 superadas. Aún no: punteros, estructuras, arrays de 2 dimensiones, pasar arrays a funciones y la conversión implícita entre entero y decimal (`k * 1.5` hay que escribirlo con `(double)k`).
