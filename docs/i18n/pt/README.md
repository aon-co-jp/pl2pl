> Nota: o README.md em japonês na raiz do repositório é o original. Esta versão foi traduzida por IA e revisada por uma IA quanto à fluência de falante nativo; ainda não foi conferida por uma pessoa nativa.

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
| `pl2pl ir` | (versão de teste) Coloca as «rotas» de PHP, Python e Java numa mesma forma e gera JSON, OpenAPI ou um esqueleto de FastAPI. A conversão de PHP também coloca tipos, conteúdo das funções, modelos, middleware e templates nessa forma antes de escrever |
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

## Novidades (2026-11-22 a 2026-11-23)
- COBOL (`converters/cobol_to`): foram adicionados `SORT` (`USING` / `GIVING`, `INPUT` / `OUTPUT PROCEDURE`, `RELEASE`, `RETURN`), `MERGE`, `SEARCH` / `SEARCH ALL` e `INSPECT ... LEADING` / `CHARACTERS`. Os usos `COMP-3` e `COMP` agora também são lidos.
- SQL embutido: `EXEC SQL` (cursores, `FETCH`, `SELECT ... INTO`), `WHENEVER`, variáveis indicadoras (inclusive a forma com a palavra-chave `INDICATOR`), `SQLSTATE`, `EXECUTE IMMEDIATE` e `PREPARE` / `EXECUTE ... USING`. O Python gerado foi executado de verdade; no lado do Rust, a verificação foi feita só com um manipulador simulado.
- `EXEC CICS` (mínimo): `SEND TEXT` mostra texto, e `RETURN` / `ABEND` encerram o programa. Os outros comandos ficam como `TODO`. Os mapas BMS não são reproduzidos.
- Java (`converters/java_to`): referências a métodos, `Comparator`, `Collectors` e classes anônimas (todos os métodos são mantidos). No lado do Rust, as operações com List (`Vec`, `push`, acesso por índice, `for`) funcionam, e o Rust gerado passa no `cargo check` para spring, jaxrs, servlet e spark.
- Teste com o Spring PetClinic REST (um projeto que parte do OpenAPI): as rotas encontradas passaram de 1 para 37; o Python gerado tem 0 erros de sintaxe; as consultas derivadas (`Or`, `Like`, comparações, `OrderBy` etc.) rodam em um repositório em memória; o aplicativo pode ser importado (41 rotas); e os nomes de `this.field` agora coincidem, de modo que uma chamada percorre controlador → serviço → repositório.
- Ainda em aberto: gerar o repositório e o mapeador MapStruct (as chamadas param em um `TODO`), um banco de dados real, uma execução real do FastAPI gerado, os mapas e arquivos do CICS, e os corpos em Rust com Map ou Stream. Testes do pl2pl: 55 aprovados.

## 2026-11-25 Situação e pendências

*Japonês e inglês são a base; os demais idiomas são traduzidos por IA, ainda sem conferência de falantes nativos.*

Situação: uma ferramenta que reescreve programas antigos (COBOL, C, C++, Fortran, Java etc.) na linguagem e no framework que você escolher. Recentemente incorporou o suporte ao PetClinic do Java_to (ResponseEntity, Mapper, classes de dados pai) e a documentação em vários idiomas foi sincronizada. Pendente: verificar em um Mac e um iPhone de verdade (não temos esses aparelhos aqui). Traduções: feitas por IA, ainda não revisadas por falantes nativos.

<!-- directive-20261125-1130 -->
## 2026-11-25 Diretriz (sem confirmação até as 11:30)
Até 2026-11-25 11:30, o que estiver pendente avança automaticamente até ficar pronto, sem pedir confirmação. Mesmo que a demanda pareça pequena, a necessidade pareça baixa ou seja preciso refazer tudo do zero em grande escala, não se troca por outro caminho: refazemos de frente. Desenvolvidos em conjunto: COBOL_to, Java_to, MFastAPI, 2MFastAPI (converte outras linguagens e frameworks para um código compatível com o MFastAPI; Node.js e TypeScript são uma exceção opcional, e também é permitido não escolher nenhum dos dois), RMojo, RPoRea (RPoem e Reaxt integrados), 2RPoRea (converte outras linguagens e frameworks para o RPoRea) e Reaxt (React + Next.js + Nuxt.js + Nest.js integrados). Quando um muda, conferem-se os outros e atualizam-se README, CLAUDE, PORTING e POLICY em japonês e inglês no mesmo dia.

<!-- directive-20261125-mojo -->
## 2026-11-25 Adendo: o MFastAPI deve poder ser usado a partir do Mojo
O MFastAPI mantém a **especificação de poder ser usado a partir do Mojo** (sintaxe Python, núcleo em Rust, suporte a Mojo; no Windows, o Mojo de verdade dentro do WSL é acionado por meio do `compat/wsl-bridge` do RMojo). O resultado do 2MFastAPI também deve poder ser usado a partir do Mojo. Situação atual: `mojo/` é uma implementação de referência, e os servidores das etapas B e C são experimentais. Só o que foi realmente executado é chamado de «verificado». Seguimos automaticamente, sem pedir confirmação, até as 11:30.

## 2026-11-25 Adendo: c2rust com arrays e break/continue
O `pl2pl c2rust` agora entende arrays de tamanho fixo (`int a[8] = {...};`, `double w[4];`, com os valores iniciais que faltarem preenchidos com 0), leitura e escrita por índice (`a[j + 1]` vira `a[(j + 1) as usize]`), `break;`, `continue;` e `while (1)` como `loop`. Um exemplo com bubble sort e `while (1)` foi executado com gcc e rustc no WSL, e as saídas coincidiram exatamente (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). 56 testes unitários aprovados. Ainda não: ponteiros, structs, arrays de duas dimensões, passar arrays para funções e a conversão implícita entre inteiro e decimal (`k * 1.5` precisa ser escrito como `(double)k`).

## Saiba mais
[LANGUAGES.md](../../LANGUAGES.md) (lista de linguagens) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinações) / [DESIGN.md](../../DESIGN.md) (o projeto, escrito pelo proprietário) / [components/README.md](../../../components/README.md) (peças que vêm junto). Estes arquivos estão em japonês.
