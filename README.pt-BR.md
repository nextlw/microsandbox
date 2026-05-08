<div align="center">
    <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank" align="center">
        <img width="35%" src="./assets/microsandbox-gh-banner-dark.png" alt="microsandbox-banner-xl-dark">
    </a>
</div>

<div align="center">
    <a href="./README.pt-BR.md#gh-light-mode-only" target="_blank">
        <img width="35%" src="./assets/microsandbox-gh-banner-light.png" alt="microsandbox-banner-xl">
    </a>
</div>

<br />

<div align="center"><b>——&nbsp;&nbsp;&nbsp;todo agente merece o próprio computador&nbsp;&nbsp;&nbsp;——</b></div>

<p align="center"><a href="./README.md">English</a> · Português (Brasil)</p>

<br />
<br />

<div align='center'>
  <a href="https://github.com/superradcompany/microsandbox/releases"><img src="https://img.shields.io/github/v/release/superradcompany/microsandbox?include_prereleases&style=for-the-badge" alt="GitHub release"></a>
  <a href="https://discord.gg/T95Y3XnEAK"><img src="https://img.shields.io/discord/1315784565562019870?label=Discord&logo=discord&logoColor=white&color=5865F2&style=for-the-badge" alt="Discord"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache 2.0-blue.svg?style=for-the-badge" alt="Licença Apache 2.0"></a>
</div>

<br />

O **Microsandbox** sobe **VMs leves em milissegundos** a partir dos nossos SDKs. Roda localmente na sua máquina. Sem servidor para configurar. Sem daemon persistente. Tudo embutido e sem precisar de root!

##

- <img height="14" src="https://octicons-col.vercel.app/shield-lock/A770EF"> **Isolamento de hardware**: isolamento em nível de hardware com tecnologia de microVM.
- <img height="14" src="https://octicons-col.vercel.app/zap/A770EF"> **Inicialização instantânea**: tempo médio de boot abaixo de 100 milissegundos.
- <img height="14" src="https://octicons-col.vercel.app/plug/A770EF"> **Embebível**: crie VMs direto no seu código. Sem servidor de setup. Sem daemon de longa duração.
- <img height="14" src="https://octicons-col.vercel.app/lock/A770EF"> **Segredos que não vazam**: chaves secretas que não entram na VM e não podem ser exploradas.
- <img height="14" src="https://octicons-col.vercel.app/package/A770EF"> **Compatível com OCI**: executa imagens de contêiner padrão do Docker Hub, GHCR ou qualquer registry OCI.
- <img height="14" src="https://octicons-col.vercel.app/database/A770EF"> **Execução prolongada**: sandboxes podem rodar em modo destacado. Ideal para sessões de longa duração.
- <img height="14" src="https://octicons-col.vercel.app/terminal/A770EF"> **Pronto para agentes**: seus agentes podem criar as próprias sandboxes com nossas [Agent Skills](https://github.com/superradcompany/skills) e [servidor MCP](https://github.com/superradcompany/microsandbox-mcp).


<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="13" src="https://octicons-col.vercel.app/rocket/ffffff" alt="rocket-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="13" src="https://octicons-col.vercel.app/rocket/000000" alt="rocket"></a>&nbsp;&nbsp;Começando

#### <img height="14" src="https://octicons-col.vercel.app/move-to-bottom/A770EF">&nbsp;&nbsp;Instalar o SDK

> ```sh
> cargo add microsandbox    # 🦀 Rust
> ```
> ```sh
> uv add microsandbox       # 🐍 Python
> ```
> ```sh
> npm i microsandbox        # 🟦 TypeScript
> ```

#### <img height="14" src="https://octicons-col.vercel.app/download/A770EF">&nbsp;&nbsp;Instalar a CLI **(opcional)**

> Suba uma microVM com um único comando.
>
> ```sh
> npx microsandbox run debian
> ```
>
> Ou instale o comando `msb` globalmente:
>
> ```sh
> curl -fsSL https://install.microsandbox.dev | sh
> ```
>
> ```sh
> msb run debian
> ```

##

> **Requisitos**: Linux com KVM habilitado, ou macOS com Apple Silicon.<br />
> **Aviso**: o Microsandbox ainda é **software em beta**. Espere mudanças incompatíveis, recursos faltando e arestas a polir.

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/package-dependencies/ffffff" alt="sdk-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/package-dependencies/000000" alt="sdk"></a>&nbsp;&nbsp;SDK

O SDK permite criar e controlar sandboxes direto da sua aplicação. `Sandbox::builder("...").create()` sobe uma microVM como processo filho. Sem infraestrutura necessária.

#### <img height="14" src="https://octicons-col.vercel.app/play/A770EF">&nbsp;&nbsp;Executar código em uma sandbox

> ```rs
> use microsandbox::Sandbox;
>
> #[tokio::main]
> async fn main() -> Result<(), Box<dyn std::error::Error>> {
>     let sandbox = Sandbox::builder("my-sandbox")
>         .image("python")
>         .cpus(1)
>         .memory(512)
>         .create()
>         .await?;
>
>     let output = sandbox
>         .exec("python", ["-c", "print('Hello from a microVM!')"])
>         .await?;
>
>     println!("{}", output.stdout()?);
>
>     sandbox.stop_and_wait().await?;
>
>     Ok(())
> }
> ```
>
> <details>
> <summary><b>&nbsp;Exemplo em Python →</b></summary>
>
> ```python
> import asyncio
> from microsandbox import Sandbox
>
> async def main():
>     sandbox = await Sandbox.create(
>         "my-sandbox",
>         image="python",
>         cpus=1,
>         memory=512,
>     )
>
>     output = await sandbox.exec("python", ["-c", "print('Hello from a microVM!')"])
>
>     print(output.stdout_text)
>
>     await sandbox.stop_and_wait()
>
> asyncio.run(main())
> ```
>
> </details>
>
> <details>
> <summary><b>&nbsp;Exemplo em TypeScript →</b></summary>
>
> ```typescript
> import { Sandbox } from "microsandbox";
>
> await using sandbox = await Sandbox.builder("my-sandbox")
>   .image("python")
>   .cpus(1)
>   .memory(512)
>   .create();
>
> const output = await sandbox.exec("python", ["-c", "print('Hello from a microVM!')"]);
>
> console.log(output.stdout());
> ```
>
> </details>


> A primeira chamada a `create()` baixa a imagem se ela não estiver em cache local, então pode demorar mais conforme a sua conexão. Execuções seguintes reutilizam o cache.

<br />

<a href="https://docs.microsandbox.dev/sdk/overview"><img src="https://img.shields.io/badge/SDK_Docs-%E2%86%92-A770EF?style=flat-square&labelColor=2b2b2b" alt="Documentação do SDK"></a>

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/terminal/ffffff" alt="cli-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/terminal/000000" alt="cli"></a>&nbsp;&nbsp;CLI

A CLI `msb` oferece uma interface completa para gerenciar sandboxes, imagens e volumes.

#### <img height="14" src="https://octicons-col.vercel.app/play/A770EF">&nbsp;&nbsp;Executar um comando

> ```sh
> msb run python -- python3 -c "print('Hello from a microVM!')"
> ```

#### <img height="14" src="https://octicons-col.vercel.app/stopwatch/A770EF">&nbsp;&nbsp;Sandboxes nomeadas

> ```sh
> # Criar e iniciar uma sandbox nomeada
> msb create --name my-app python
> ```
>
> ```sh
> # Executar comandos
> msb exec my-app -- python -c "import this"
> msb exec my-app -- curl https://example.com
> ```
>
> ```sh
> # Ciclo de vida
> msb stop my-app
> msb start my-app
> msb rm my-app
> ```

#### <img height="14" src="https://octicons-col.vercel.app/cache/A770EF">&nbsp;&nbsp;Gerenciamento de imagens

> ```sh
> msb pull python           # Baixar uma imagem
> msb image ls              # Listar imagens em cache
> msb image rm python       # Remover uma imagem
> ```

#### <img height="14" src="https://octicons-col.vercel.app/download/A770EF">&nbsp;&nbsp;Instalar e desinstalar sandboxes

> ```sh
> msb install ubuntu               # Instala a sandbox ubuntu como comando 'ubuntu'
> ubuntu                           # Abre Ubuntu em uma microVM
> msb uninstall ubuntu             # Remove a sandbox ubuntu
> ```

#### <img height="14" src="https://octicons-col.vercel.app/list-unordered/A770EF">&nbsp;&nbsp;Status e inspeção

> ```sh
> msb ls                         # Listar todas as sandboxes
> msb ps my-app                  # Mostrar status da sandbox
> msb inspect my-app             # Informações detalhadas da sandbox
> msb metrics my-app             # Estatísticas ao vivo de CPU/memória/rede
> ```

> [!TIP]
>
> Execute `msb --tree` para ver todos os comandos disponíveis e suas opções.

<br />

<a href="https://docs.microsandbox.dev/cli/overview"><img src="https://img.shields.io/badge/CLI_Docs-%E2%86%92-A770EF?style=flat-square&labelColor=2b2b2b" alt="Documentação da CLI"></a>

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/dependabot/ffffff" alt="agents-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/dependabot/000000" alt="agents"></a>&nbsp;&nbsp;Agentes de IA

Dê aos seus agentes de IA a capacidade de criar e gerenciar as próprias sandboxes.

#### <img height="14" src="https://octicons-col.vercel.app/book/A770EF">&nbsp;&nbsp;Agent Skills

> Ensine qualquer agente de código a usar o microsandbox instalando as [Agent Skills](https://github.com/superradcompany/skills). Funciona com Claude Code, Cursor, Codex, Gemini CLI, GitHub Copilot e outros.
>
> ```sh
> npx skills add superradcompany/skills
> ```

#### <img height="14" src="https://octicons-col.vercel.app/plug/A770EF">&nbsp;&nbsp;Servidor MCP

> Conecte qualquer agente compatível com MCP ao microsandbox com o [servidor MCP](https://github.com/superradcompany/microsandbox-mcp). Oferece chamadas de ferramenta estruturadas para ciclo de vida da sandbox, execução de comandos, acesso ao sistema de arquivos, volumes e monitoramento.
>
> ```sh
> # Claude Code
> claude mcp add --transport stdio microsandbox -- npx -y microsandbox-mcp
> ```

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/book/ffffff" alt="docs-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/book/000000" alt="docs"></a>&nbsp;&nbsp;Documentação

Para guias, referências de API e exemplos, visite a [documentação do microsandbox](https://docs.microsandbox.dev).

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/gear/ffffff" alt="contributing-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/gear/000000" alt="contributing"></a>&nbsp;&nbsp;Contribuir

Interessado em contribuir com o `microsandbox`? Veja o [CONTRIBUTING.md](./CONTRIBUTING.md) para diretrizes e o [DEVELOPMENT.md](./DEVELOPMENT.md) para build, testes e release.

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/law/ffffff" alt="license-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/law/000000" alt="license"></a>&nbsp;&nbsp;Licença

Este projeto está licenciado sob a [Apache License 2.0](./LICENSE).

<br />

## <a href="./README.pt-BR.md#gh-dark-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/heart/ffffff" alt="acknowledgements-dark"></a><a href="./README.pt-BR.md#gh-light-mode-only" target="_blank"><img height="18" src="https://octicons-col.vercel.app/heart/000000" alt="acknowledgements"></a>&nbsp;&nbsp;Agradecimentos

Um agradecimento especial a todos os contribuidores, testadores e membros da comunidade que ajudam a melhorar o microsandbox todos os dias! Agradecemos também aos seguintes projetos e comunidades que tornaram o `microsandbox` possível: [libkrun](https://github.com/containers/libkrun) e [smoltcp](https://github.com/smoltcp-rs/smoltcp)

<br />
<br />
<br />

<div align='center'>
  <a href="https://www.ycombinator.com/"><img src="https://img.shields.io/badge/BACKED%20BY-Y%20COMBINATOR-F26522?style=for-the-badge&logo=ycombinator&logoColor=white" alt="Apoiado pela Y Combinator"></a>
</div>
