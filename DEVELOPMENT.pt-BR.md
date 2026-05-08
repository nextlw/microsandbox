[English](./DEVELOPMENT.md) · Português (Brasil)

# Desenvolvendo o Microsandbox

Este guia cobre o que você precisa para **compilar**, **testar** e **publicar** o microsandbox a partir do código-fonte.

Para diretrizes de contribuição (fork, assinatura de commits, pull requests), veja [CONTRIBUTING.md](./CONTRIBUTING.md).

## Pré-requisitos

- **Sistema operacional**:
  - macOS com Apple Silicon (M1/M2/M3/M4)
  - Linux com KVM habilitado
- **Ferramentas**: [`just`](https://github.com/casey/just), `git` e `pre-commit`
  - Linux: `sudo apt install just git` e `pip install pre-commit` (ou `sudo apt install pre-commit`)
  - macOS: `brew install just git pre-commit`
- **Docker** (somente macOS): necessário para cross-compilar o `agentd` e compilar o kernel
- **Rust**: instalado automaticamente por `just setup` se faltar, ou instale via [rustup](https://rustup.rs)

## Configuração inicial

Clone o repositório e rode o setup único:

```bash
git clone https://github.com/microsandbox/microsandbox.git
cd microsandbox
just setup
```

O `just setup` faz o seguinte:

1. Instala dependências de sistema (ferramentas de build, toolchain musl, etc.)
2. Inicializa submódulos Git (`vendor/libkrunfw`, etc.)
3. Compila dependências binárias (`agentd` e `libkrunfw`)
4. Compila a CLI `msb`
5. Instala binários em `~/.microsandbox/bin/` e bibliotecas em `~/.microsandbox/lib/`
6. Instala hooks do pre-commit

> Durante a compilação, podem aparecer prompts de configuração do kernel — pressione **Enter** para aceitar os padrões.

Depois do setup, adicione ao seu perfil de shell (por exemplo `~/.bashrc`, `~/.zshrc`):

```bash
# Linux
export PATH="$HOME/.microsandbox/bin:$PATH"
export LD_LIBRARY_PATH="$HOME/.microsandbox/lib:$LD_LIBRARY_PATH"

# macOS
export PATH="$HOME/.microsandbox/bin:$PATH"
export DYLD_LIBRARY_PATH="$HOME/.microsandbox/lib:$DYLD_LIBRARY_PATH"
```

Verifique a instalação:

```bash
msb --version
```

## Ciclo de build e instalação

O ciclo principal de desenvolvimento é:

```bash
just build && just install
```

Isso recompila a CLI `msb` (e garante que `agentd` e `libkrunfw` estejam atualizados) e instala os binários atualizados em `~/.microsandbox/`.

Para um build otimizado para release:

```bash
just build release && just install
```

### Alvos de build individuais

| Comando | Descrição |
| --- | --- |
| `just build` | Compila tudo (agentd + libkrunfw + msb) em modo debug |
| `just build release` | Compila tudo em modo release |
| `just build-msb` | Compila só a CLI `msb` (debug) |
| `just build-msb release` | Compila só a CLI `msb` (release) |
| `just build-deps` | Compila só dependências binárias (agentd + libkrunfw) |
| `just build-agentd` | Compila só o agentd |
| `just build-libkrunfw` | Compila só o libkrunfw |
| `just install` | Instala msb + libkrunfw em `~/.microsandbox/` |
| `just uninstall` | Remove os binários instalados |
| `just clean` | Remove artefatos em `build/` e limpa o libkrunfw |

## Estrutura do projeto

### Crates do workspace

O projeto é um workspace Cargo com estes crates (ordem de dependência):

| Crate | Caminho | Descrição |
| --- | --- | --- |
| `microsandbox-utils` | `crates/utils` | Utilitários compartilhados |
| `microsandbox-protocol` | `crates/protocol` | Definições do protocolo (wire) |
| `microsandbox-db` | `crates/db` | Camada de banco de dados |
| `microsandbox-migration` | `crates/migration` | Migrações de banco de dados |
| `microsandbox-image` | `crates/image` | Manipulação de imagens OCI |
| `microsandbox-filesystem` | `crates/filesystem` | Composição do sistema de arquivos |
| `microsandbox-network` | `crates/network` | Rede baseada em smoltcp |
| `microsandbox-runtime` | `crates/runtime` | Runtime da VM (integração libkrun) |
| `microsandbox` | `crates/microsandbox` | Crate público do SDK |
| `microsandbox-cli` | `crates/cli` | Binário da CLI `msb` |
| `agentd` | `crates/agentd` | Agente dentro do guest (fora do workspace; compilado separadamente para musl) |

### Outros pacotes

| Pacote | Caminho | Descrição |
| --- | --- | --- |
| `microsandbox` (npm) | `sdk/node-ts` | SDK TypeScript/Node.js (bindings NAPI) |
| `microsandbox-mcp` (npm) | `mcp/` | Servidor MCP para agentes de IA |

### Diretórios importantes

- `vendor/libkrunfw` — Submódulo da biblioteca de firmware do kernel
- `build/` — Saída da compilação (binário agentd, biblioteca compartilhada libkrunfw, binário msb)
- `examples/rust/` — Exemplos em Rust
- `examples/typescript/` — Exemplos em TypeScript

## Testes

Rode todos os testes do workspace:

```bash
cargo test --workspace
```

Testes de um crate específico:

```bash
cargo test -p microsandbox-runtime
```

Um teste específico:

```bash
cargo test -p microsandbox test_name
```

## Benchmarks

```bash
just bench-fs
```

Roda benchmarks de sistema de arquivos Docker vs Microsandbox (14 cargas de trabalho cobrindo metadados, leituras,
escritas, exclusões, renomeações, mmap e I/O concorrente). Os resultados são gravados em JSON em
`build/bench/fs/`. Veja [`benchmarks/README.md`](benchmarks/README.md) para uso completo,
descrição das cargas, execução com várias imagens e comparação com baseline.

## Qualidade de código

### Hooks do pre-commit

Os hooks do pre-commit são instalados pelo `just setup`. Eles rodam automaticamente a cada commit e verificam:

- `cargo fmt --all --check` — formatação
- `cargo clippy --workspace -- -D warnings` — lints
- `cargo doc` — documentação compila sem warnings
- `cargo build -p microsandbox-cli` — a CLI compila
- Verificações habituais (espaço em branco no fim da linha, conflitos de merge, TOML/YAML válidos)
- Bloqueia commits diretos na branch `main`

Para rodar todas as verificações manualmente:

```bash
pre-commit run --all-files
```

> Recomenda-se rodar isso uma vez antes do primeiro commit.

Se o pre-commit não estiver instalado, instale com `pip install pre-commit` (ou `brew install pre-commit` no macOS) e depois `pre-commit install`.

### Formatação e lint

```bash
cargo fmt --all           # Formatar código
cargo clippy --workspace  # Rodar lints
```

## Releases

As releases do Microsandbox são automatizadas via CI. O processo tem duas etapas:

### 1. PR de bump de versão

Abra um PR que aumente a versão em todos os crates e pacotes. Todos compartilham o mesmo número de versão:

- `Cargo.toml` (campo `version` do workspace — todos os crates herdam disso)
- `sdk/node-ts/package.json`
- `mcp/package.json`

O título do PR deve seguir o formato: `chore: bump version to X.Y.Z`

### 2. Tag e release

Depois que o PR de bump for mergeado, crie uma tag assinada em `main` para disparar o CI de release:

```bash
git tag -a v0.X.Y -m "v0.X.Y"
git push origin v0.X.Y
```

O workflow de release (`.github/workflows/release.yml`) vai:

1. Compilar `msb`, `agentd` e `libkrunfw` para todas as plataformas (linux-x86_64, linux-aarch64, darwin-aarch64)
2. Criar bundles por plataforma (`.tar.gz`) com checksums SHA256
3. Criar uma GitHub release com os bundles
4. Publicar o SDK Node.js no npm (`microsandbox` + pacotes por plataforma)
5. Publicar o servidor MCP no npm (`microsandbox-mcp`)
6. Publicar os crates Rust no crates.io (ordem de dependência, 10 crates)

## Recursos adicionais

- [CONTRIBUTING.md](./CONTRIBUTING.md) — Como contribuir
- [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) — Código de conduta da comunidade
- [SECURITY.md](./SECURITY.md) — Políticas de segurança e como reportar vulnerabilidades
