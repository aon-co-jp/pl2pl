> Nota: el README.md en japonés de la raíz del repositorio es el original. Esta versión fue traducida con IA y revisada por una IA para que el español suene natural; todavía no la ha comprobado ninguna persona hablante nativa.

# pl2pl

## ¿Qué es pl2pl?
Los programas están escritos en muchos «idiomas» (lenguajes de programación).
**pl2pl es una herramienta que reescribe un programa de un idioma a otro.**
Por ejemplo, puede convertir un sitio hecho con PHP (Laravel) en un sitio hecho con Python (FastAPI).
Funciona en Windows, Linux y Mac (versión 0.1.0, todavía en construcción).

- Se instala en tu ordenador. El sitio web solo tiene guías y descargas → <https://easy-web.tokyo/pl2pl/>
- Si pl2pl no puede reescribir algo, no lo esconde. Deja un `TODO` (una nota para más tarde) y explica el motivo en un informe.
- Nuestra meta: poder reescribir entre el mayor número posible de lenguajes conocidos.

## Lo que ya puede hacer
| Comando | Qué hace |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Más en [PHP.md](../../PHP.md) (en japonés) |
| `pl2pl web` | Mira dentro de una carpeta, reconoce Java, COBOL, PHP y Flask/Django, y pasa cada uno a la herramienta adecuada |
| `pl2pl ir` | (versión de prueba) Pone las «rutas» de PHP, Python y Java en una misma forma y genera JSON, OpenAPI o un esqueleto de FastAPI. La conversión de PHP también pone en esta forma los tipos, el contenido de las funciones, los modelos, el middleware y las plantillas antes de escribir |
| `pl2pl watch` | Encuentra versiones nuevas de lenguajes y herramientas y se pone al día solo |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Hace que los bucles de C/C++ vayan más rápido (gpu es una versión de prueba) |
| `pl2pl cport` | Hace que el código C para UNIX funcione tanto en Windows como en Linux |
| `pl2pl pyhpc` | Acelera los cálculos de Python con Numba |
| `pl2pl llm setup` | Instala el ayudante de IA (aruaru-llm). Todavía no hemos comprobado la instalación real |

## Ponerse al día con las versiones nuevas, solo
Ejecutamos `pl2pl watch` cada noche y cada mañana.
1. Mira si han salido versiones nuevas de lenguajes y herramientas (la lista está en `registry/ecosystem.toml`).
2. Prueba con la versión nueva. Si las pruebas pasan, actualiza por sí solo la «versión compatible».
3. Si fallan, version-control (con ayuda de la IA) intenta arreglarlo.
4. Si aun así no se arregla, envía un mensaje a las direcciones de correo guardadas y a LINE.

Ten en cuenta: con laravel, symfony, flask, django y algunos más, por ahora solo avisa de que «ha salido una versión nueva».
En GitHub, también se ejecutan solas cada noche y cada mañana pruebas con las versiones más recientes.

## Cómo se usa
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Conviene saber
- pl2pl es prudente. Si algo parece arriesgado, lo deja como está y te dice por qué. No podemos prometer velocidad.
- Solo hemos probado ejemplos pequeños. Todavía no hemos probado programas grandes y reales.

## Lo que viene
- Poner también los tipos y el contenido de las funciones en la forma común y hacer pasar por ella la salida de FastAPI.
- PHP: piezas de Blade escritas como clases.
- Comprobar las alertas reales por correo y la corrección automática con IA.
- Comprobar que funciona en la GPU (open-cuda / open-directx).

## Novedades de la v0.2.0
- Elige tu combinación: `pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`. El COBOL se reescribe; C, C++ y Fortran siguen funcionando tal cual y la web los llama.
- `pl2pl c2rust` (C y C++ sencillos) y `pl2pl f2rust` (Fortran sencillo) generan Rust. `pl2pl cobol --os <linux|macos|unix|windows>` crea un script de compilación para GnuCOBOL.

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

*El japonés y el inglés son la base; los demás idiomas son traducciones con IA, aún sin revisión de nativos.*

Estado: una herramienta que reescribe programas antiguos (COBOL, C, C++, Fortran, Java, etc.) en el lenguaje y el framework que elijas. Hace poco incorporó el soporte de PetClinic de Java_to (ResponseEntity, Mapper, clases de datos padre) y se sincronizó la documentación multilingüe. Pendiente: comprobar en un Mac y un iPhone reales (aquí no tenemos esos dispositivos). Traducciones: hechas por IA, aún sin revisión de hablantes nativos.

<!-- directive-20261125-1130 -->
## 2026-11-25 Instrucción (sin confirmación hasta las 11:30)
Hasta el 2026-11-25 a las 11:30, lo pendiente se lleva hasta el final de forma automática, sin pedir confirmación. Aunque parezca que hay poca demanda o poca necesidad, o aunque haya que rehacer mucho desde cero, no se sustituye por otro método: se rehace de frente. Se desarrollan a la vez: COBOL_to, Java_to, MFastAPI, 2MFastAPI (convierte otros lenguajes y frameworks a código compatible con MFastAPI; Node.js y TypeScript se pueden elegir como excepción, y también se puede no elegirlos), RMojo, RPoRea (integración de RPoem y Reaxt), 2RPoRea (convierte otros lenguajes y frameworks a RPoRea) y Reaxt (integración de React + Next.js + Nuxt.js + Nest.js). Cuando cambia uno, se revisan los demás, y README, CLAUDE, PORTING y POLICY se actualizan en japonés e inglés el mismo día.

<!-- directive-20261125-mojo -->
## 2026-11-25 Adenda: MFastAPI debe poder usarse desde Mojo
MFastAPI mantiene la **especificación de poder usarse desde Mojo** (sintaxis de Python, implementación en Rust, compatibilidad con Mojo; en Windows se usa el Mojo real de WSL a través de `compat/wsl-bridge` de RMojo). El resultado de 2MFastAPI también debe poder usarse desde Mojo. Estado actual: `mojo/` es una implementación de referencia, y los servidores de las etapas B y C son experimentales. Solo se escribe «comprobado» sobre lo que se ejecutó de verdad. Se continúa de forma automática, sin pedir confirmación, hasta las 11:30.

## 2026-11-25 Adenda: c2rust, arrays y break/continue
`pl2pl c2rust` ahora admite arrays (arreglos) de longitud fija (`int a[8] = {...};` y `double w[4];`; los valores iniciales que faltan se rellenan con 0), lectura y escritura por índice (`a[j + 1]` pasa a `a[(j + 1) as usize]`), `break;`, `continue;` y `while (1)` como `loop`. Un ejemplo con ordenación de burbuja y `while (1)` se compiló con gcc y rustc en WSL y las salidas coinciden por completo (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). Pruebas unitarias: 56 superadas. Aún no: punteros, estructuras, arrays de 2 dimensiones, pasar arrays a funciones y la conversión implícita entre entero y decimal (`k * 1.5` hay que escribirlo con `(double)k`).

## Más información
[LANGUAGES.md](../../LANGUAGES.md) (lista de lenguajes) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinaciones) / [DESIGN.md](../../DESIGN.md) (el diseño, escrito por el propietario) / [components/README.md](../../../components/README.md) (piezas que vienen con la herramienta). Estos archivos están en japonés.
