> Nota: el PORTING.md en japonés de la raíz del repositorio es el original. Esta versión es una traducción hecha con IA, en palabras sencillas, y aún no ha sido revisada por hablantes nativos.

# pl2pl – traspaso
(Las notas detalladas anteriores están en `docs/HISTORY-ja.md`, en japonés.)

## Dónde estamos (2026-11-19)
- La versión 0.1.0 está publicada, con instaladores para Windows, Linux y Mac y pruebas en CI.
- PHP → FastAPI está probado de extremo a extremo con FastAPI real: rutas, Eloquent, constructor de consultas, Blade, middleware, relaciones y la familia `@can`.
- `pl2pl web` ahora también reconoce PHP.
- `pl2pl watch` encuentra versiones nuevas, las prueba, arregla problemas y envía una alerta si nada funciona. GitHub también prueba con las versiones más recientes cada noche y cada mañana.
- `pl2pl ir` (versión de prueba) pone las rutas de PHP, Python y Java (Spring, JAX-RS, Servlet, Spark) en una forma común y genera JSON, OpenAPI o un esqueleto de FastAPI. Hemos comprobado que Python puede leer de verdad la salida.
- La página de presentación y descarga está en línea: <https://easy-web.tokyo/pl2pl/> (en el VPS, estática).
- COBOL_to y Java_to están integrados en `converters/`.

## Pendiente (lo más importante primero)
1. **Comprobar**: que llegue un correo real (`version-control --test-notify`; este PC no tiene ajustes SMTP), la corrección automática con version-control `--loop` y una prueba con el último fastapi (la ejecución nocturna en GitHub lo comprobará).
2. Poner los tipos y el contenido de las funciones en la IR. Hacer pasar por la IR la salida actual de FastAPI (`pl2pl php`). Si la búsqueda de rutas se deja cosas, pensar en tree-sitter.
3. Se probó `watch` con una versión nueva real: con fastapi 0.165.0 (starlette 1.10), `tests/run-php.sh` pasó 49 comprobaciones, así que la versión probada del registro subió a 0.165. Pendiente: los nuevos laravel 13 / django 6.1 / spring-boot 4.1 siguen «sin comprobar», porque las pruebas de extremo a extremo no usan los frameworks reales (solo se informa hasta que existan esas pruebas).
4. Programar en un PC las ejecuciones de la noche y de la mañana (aún no hecho: el comando de comprobación no funciona en este PC y daría falsas alarmas; hay un ejemplo al principio de `scripts/pl2pl-watch.cmd`).
5. Añadir una tarjeta de pl2pl en la página principal de easy-web.tokyo.
6. PHP: piezas de Blade escritas como clases, `$attributes`, `@php`, `morphToMany`.
7. Integración con aruaru-llm (la instalación real con `llm setup` no está comprobada). Kokkos en GPU; núcleos en open-cuda / open-directx.
8. Borrar los COBOL_to y Java_to originales (**no se puede deshacer: pedir antes la aprobación final**).
9. El propietario escribe `docs/DESIGN.md`. Comprobar que `docs/LANGUAGES.md` no esté desactualizado.

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
