> Nota: o README em japonês na raiz do repositório é o original. Esta versão é uma tradução feita por IA, em palavras simples, e ainda não foi revisada por falantes nativos.

# pl2pl

## O que é o pl2pl?
Os programas são escritos em muitas «línguas» (linguagens de programação).
**O pl2pl é uma ferramenta que reescreve um programa de uma língua para outra.**
Por exemplo, ele pode transformar um site feito com PHP (Laravel) em um site feito com Python (FastAPI).
Funciona no Windows, no Linux e no Mac (versão 0.1.0, ainda em construção).

- Você instala no seu computador. O site só tem guias e downloads → <https://easy-web.tokyo/pl2pl/>
- Se o pl2pl não consegue reescrever algo, ele não esconde. Deixa um `TODO` (uma nota para depois) e explica o motivo em um relatório.
- Nossa meta: conseguir reescrever entre o maior número possível de linguagens conhecidas.

## O que ele já faz
| Comando | O que faz |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Mais em [PHP.md](../../PHP.md) (em japonês) |
| `pl2pl web` | Olha dentro de uma pasta, reconhece Java, COBOL, PHP e Flask/Django, e passa cada um para a ferramenta certa |
| `pl2pl ir` | (versão de teste) Coloca as «rotas» de PHP, Python e Java numa mesma forma e gera JSON, OpenAPI ou um esqueleto de FastAPI |
| `pl2pl watch` | Encontra versões novas de linguagens e ferramentas e se atualiza sozinho |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Faz os laços de C/C++ rodarem mais rápido (gpu é uma versão de teste) |
| `pl2pl cport` | Faz código C para UNIX funcionar tanto no Windows quanto no Linux |
| `pl2pl pyhpc` | Acelera cálculos de Python com Numba |
| `pl2pl llm setup` | Instala o ajudante de IA (aruaru-llm). Ainda não verificamos a instalação real |

## Acompanhar as versões novas, sozinho
Executamos o `pl2pl watch` toda noite e toda manhã.
1. Ele verifica se saíram versões novas de linguagens e ferramentas (a lista está em `registry/ecosystem.toml`).
2. Testa com a versão nova. Se os testes passarem, atualiza sozinho a «versão compatível».
3. Se falharem, o version-control (com ajuda de IA) tenta consertar.
4. Se ainda assim não resolver, envia uma mensagem para os e-mails guardados e para o LINE.

Atenção: para laravel, symfony, flask, django e alguns outros, por enquanto ele só avisa que «saiu uma versão nova».
No GitHub, testes com as versões mais recentes também rodam sozinhos toda noite e toda manhã.

## Como usar
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Bom saber
- O pl2pl é cuidadoso. Se algo parece arriscado, ele deixa como está e diz por quê. Não podemos prometer velocidade.
- Só testamos exemplos pequenos. Ainda não tentamos programas grandes e reais.

## O que vem a seguir
- Colocar também os tipos e o conteúdo das funções na forma comum e fazer a saída do FastAPI passar por ela.
- PHP: peças do Blade escritas como classes.
- Verificar os alertas reais por e-mail e a correção automática por IA.
- Verificar o funcionamento na GPU (open-cuda / open-directx).

## Novidades da v0.2.0
- Escolha sua combinação: `pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`. O COBOL é reescrito; C, C++ e Fortran continuam rodando como estão e a web os chama.
- `pl2pl c2rust` (C e C++ simples) e `pl2pl f2rust` (Fortran simples) geram Rust. `pl2pl cobol --os <linux|macos|unix|windows>` cria um script de compilação para o GnuCOBOL.

## Novidades da v0.2.1 (2026-11-20)
- Ao escolher `rpoem` / `rporea` em `pl2pl web --backend`, aparecem avisos (japonês e inglês): o RPoem já inclui recursos compatíveis com Tauri e WunderGraph Cosmo, tokio e WASM, então não os escolha à parte; HTML5 e CSS3 são usados, mas em pouca quantidade; JavaScript, TypeScript, Node.js e REST não são usados por projeto. O que for adicionado com `--also` e já estiver incluído é recusado (código de saída 2); o que está incluído mas ainda não é maduro pode ser escolhido, com um aviso.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` julga essa maturidade rodando testes de sondagem (registro: `registry/maturity.toml`). A opinião da IA do aruaru-llm nunca muda sozinha o veredito.
- Com pilhas diferentes de RPoem / RPoRea, `--also nodejs` (ou `next.js`) é aceito com um aviso: não impomos nossa regra de «sem Node.js», mas não o recomendamos. São listadas alternativas para a linguagem escolhida (RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js e outras que não verificamos).
- Repositórios irmãos desenvolvidos juntos: Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js e pl2pl. Quando um muda, os outros são conferidos no mesmo dia.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` e `webpage/info.html` explicam os princípios de projeto do RPoRea / RPoem, com vantagens e desvantagens, em 11 idiomas.
- Os originais `COBOL_to` / `Java_to` foram mantidos (não apagados); as cópias incorporadas em `converters/` deram saída idêntica.

## Saiba mais
[LANGUAGES.md](../../LANGUAGES.md) (lista de linguagens) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinações) / [DESIGN.md](../../DESIGN.md) (o projeto, escrito pelo proprietário) / [components/README.md](../../../components/README.md) (peças que vêm junto). Estes arquivos estão em japonês.
