> Hinweis: Das japanische PORTING.md im Wurzelverzeichnis ist das Original. Diese Fassung ist eine KI-Übersetzung in einfachen Worten und wurde noch nicht von Muttersprachlern geprüft.

# pl2pl – Übergabe
(Ältere, ausführliche Notizen stehen in `docs/HISTORY-ja.md`, auf Japanisch.)

## Wo wir stehen (2026-11-19)
- Version 0.1.0 ist veröffentlicht, mit Installern für Windows, Linux und Mac sowie Tests in der CI.
- PHP → FastAPI ist mit echtem FastAPI durchgängig getestet: Routen, Eloquent, Query Builder, Blade, Middleware, Beziehungen und die `@can`-Familie.
- `pl2pl web` erkennt jetzt auch PHP.
- `pl2pl watch` findet neue Versionen, testet sie, behebt Probleme und schickt eine Warnung, wenn nichts hilft. Auch auf GitHub wird jeden Abend und jeden Morgen mit den neuesten Versionen getestet.
- `pl2pl ir` (Testversion) bringt die Routen von PHP, Python und Java (Spring, JAX-RS, Servlet, Spark) in eine gemeinsame Form und schreibt JSON, OpenAPI oder ein FastAPI-Gerüst. Wir haben geprüft, dass Python die Ausgabe wirklich lesen kann.
- Die Seite mit Infos und Downloads ist online: <https://easy-web.tokyo/pl2pl/> (auf dem VPS, statisch).
- COBOL_to und Java_to sind in `converters/` übernommen.

## Noch zu tun (Wichtigstes zuerst)
1. **Prüfen**: Kommt eine echte E-Mail an (`version-control --test-notify`; dieser PC hat keine SMTP-Einstellungen)? Funktioniert die automatische Reparatur über version-control `--loop`? Test mit dem neuesten fastapi (der nächtliche Lauf auf GitHub prüft das).
2. Typen und Funktionsinhalte in die IR bringen. Die jetzige FastAPI-Ausgabe (`pl2pl php`) über die IR laufen lassen. Wenn die Routensuche etwas übersieht, tree-sitter in Betracht ziehen.
3. `watch` wurde mit einer echten neuen Version ausprobiert: Mit fastapi 0.165.0 (starlette 1.10) bestand `tests/run-php.sh` 49 Prüfungen, daher rückte die getestete Version in der Registry auf 0.165. Offen: Die neuen laravel 13 / django 6.1 / spring-boot 4.1 bleiben „ungeprüft“, weil die Durchlauftests die echten Frameworks nicht nutzen (nur Meldung, bis es solche Tests gibt).
4. Die nächtlichen und morgendlichen Läufe auf einem PC einrichten (noch nicht getan: Der Prüfbefehl läuft auf diesem PC nicht und würde Fehlalarme geben; ein Beispiel steht oben in `scripts/pl2pl-watch.cmd`).
5. Eine pl2pl-Karte auf die Startseite von easy-web.tokyo setzen.
6. PHP: Blade-Bausteine als Klassen, `$attributes`, `@php`, `morphToMany`.
7. Anbindung an aruaru-llm (die echte Installation per `llm setup` ist ungeprüft). Kokkos auf der GPU; Kernel auf open-cuda / open-directx.
8. Die ursprünglichen COBOL_to und Java_to löschen (**nicht rückgängig zu machen, deshalb vorher die endgültige Zustimmung einholen**).
9. `docs/DESIGN.md` schreibt der Eigentümer. Prüfen, ob `docs/LANGUAGES.md` noch aktuell ist.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP durchgängig) / `cargo test --release`

## Nachricht zum Weitermachen
„F:\pl2pl fortsetzen. ‚Noch zu tun‘ in PORTING.md von oben abarbeiten. Was sich nicht umschreiben lässt, nie verstecken, sondern als TODO stehen lassen. Erst durch echtes Ausführen prüfen, dann Erledigung melden. Antworten nur auf Japanisch. Bei jedem Schritt committen und pushen, auch den VPS aktualisieren.“

## Neu in v0.2.1 (2026-11-20)
- Wählt man bei `pl2pl web --backend` `rpoem` / `rporea`, erscheinen Hinweise (Japanisch und Englisch): RPoem enthält schon Tauri- und WunderGraph-Cosmo-kompatible Funktionen, tokio und WASM, also nicht extra wählen; HTML5 und CSS3 werden genutzt, aber knapp gehalten; JavaScript, TypeScript, Node.js und REST werden vom Entwurf her nicht genutzt. Was man mit `--also` hinzufügt und schon enthalten ist, wird abgelehnt (Exit-Code 2); was enthalten, aber noch nicht ausgereift ist, bleibt mit Warnhinweis wählbar.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` beurteilt diese Reife mit Testläufen (Liste: `registry/maturity.toml`). Die KI-Meinung von aruaru-llm ändert das Urteil nie allein.
- Bei Stacks außer RPoem / RPoRea wird `--also nodejs` (oder `next.js`) mit Hinweis akzeptiert: Wir zwingen unsere Regel „kein Node.js“ niemandem auf, empfehlen es aber nicht. Alternativen für die gewählte Sprache werden aufgelistet (RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js und weitere, die wir nicht geprüft haben).
- Schwester-Repositorys, die gemeinsam entwickelt werden: Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js und pl2pl. Ändert sich eines, werden die anderen am selben Tag geprüft.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` und `webpage/info.html` erklären die Entwurfsprinzipien von RPoRea / RPoem mit Vor- und Nachteilen in 11 Sprachen.
- Die ursprünglichen `COBOL_to` / `Java_to` bleiben erhalten (nicht gelöscht); die zusammengeführten Kopien in `converters/` lieferten identische Ausgabe.
