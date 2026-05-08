//! Hello, Sandbox — tour completo do SDK Rust do microsandbox.
//!
//! Este exemplo demonstra as principais funcionalidades do SDK:
//!   1. Criação de sandbox com builder
//!   2. Execução de comandos (exec, exec_with, shell)
//!   3. Streaming de saída de comandos
//!   4. Operações no filesystem (ler, escrever, listar)
//!   5. Patches no rootfs antes do boot
//!   6. Volumes (bind mount, tmpfs)
//!   7. Política de rede e secrets
//!   8. Port publishing
//!   9. Scripts injetados
//!  10. Métricas da sandbox
//!  11. Logs capturados
//!  12. Ciclo de vida (stop, remove)
//!
//! Uso: cargo run -p hello-sandbox

use microsandbox::Sandbox;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║       🏖️  Hello, Sandbox — SDK Rust Tour        ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    // ───────────────────────────────────────────────────────────
    // 1. Criação da sandbox com builder
    // ───────────────────────────────────────────────────────────
    println!("── 1. Criando sandbox ──────────────────────────────");

    let sb = Sandbox::builder("hello-sandbox")
        .image("alpine:latest")
        .cpus(1)
        .memory(512)
        .replace()               // substitui se já existir
        .create()
        .await?;

    println!("   ✓ Sandbox 'hello-sandbox' criada e rodando\n");

    // ───────────────────────────────────────────────────────────
    // 2. Execução de comandos
    // ───────────────────────────────────────────────────────────
    println!("── 2. Execução de comandos ─────────────────────────");

    // 2a. exec() — comando simples com argumentos
    let output = sb.exec("echo", ["Hello from inside the sandbox!"]).await?;
    println!("   exec():  {}", output.stdout()?.trim());

    // 2b. shell() — comando via shell (/bin/sh -c)
    let output = sb.shell("uname -a && cat /etc/os-release 2>/dev/null || true").await?;
    println!("   shell():");
    for line in output.stdout()?.lines() {
        println!("           {}", line);
    }

    // 2c. exec_with() — comando com opções avançadas
    let output = sb
        .exec_with("sh", |e| {
            e.args(["-c", "for i in 1 2 3; do echo \"Linha $i\"; done"])
                .env("CUSTOM_VAR", "hello-from-sdk")
                .timeout(Duration::from_secs(10))
        })
        .await?;
    println!("   exec_with():");
    for line in output.stdout()?.lines() {
        println!("               {}", line);
    }

    // 2d. Exit code
    let output = sb.exec("sh", ["-c", "exit 42"]).await?;
    println!(
        "   exit code: {} (esperado: 42)",
        output.status().code
    );
    println!();

    // ───────────────────────────────────────────────────────────
    // 3. Streaming de saída (event loop)
    // ───────────────────────────────────────────────────────────
    println!("── 3. Streaming de eventos ─────────────────────────");

    let mut handle = sb.exec_stream("sh", ["-c", r#"
        echo "linha 1"
        echo "linha 2" >&2
        echo "linha 3"
        exit 0
    "#]).await?;

    use microsandbox::ExecEvent;
    while let Some(event) = handle.recv().await {
        match event {
            ExecEvent::Stdout(data) => {
                println!("   [stdout] {}", String::from_utf8_lossy(&data).trim());
            }
            ExecEvent::Stderr(data) => {
                println!("   [stderr] {}", String::from_utf8_lossy(&data).trim());
            }
            ExecEvent::Exited { code } => {
                println!("   [exit]   código {code}");
                break;
            }
            _ => {}
        }
    }
    println!();

    // ───────────────────────────────────────────────────────────
    // 4. Operações no filesystem
    // ───────────────────────────────────────────────────────────
    println!("── 4. Filesystem ───────────────────────────────────");

    let fs = sb.fs();

    // 4a. Escrever arquivo
    fs.write("/tmp/hello.txt", "Ola do SDK Rust!\n").await?;
    println!("   ✓ write: /tmp/hello.txt");

    // 4b. Ler arquivo
    let content = fs.read_to_string("/tmp/hello.txt").await?;
    println!("   ✓ read:  {}", content.trim());

    // 4c. Criar diretório
    fs.mkdir("/tmp/data").await?;
    println!("   ✓ mkdir: /tmp/data");

    // 4d. Listar diretório
    let entries = fs.list("/tmp").await?;
    println!("   ✓ list:  /tmp ({})", entries.len());
    for entry in &entries {
        println!("           {:?} {} ({} bytes)", entry.kind, entry.path, entry.size);
    }

    // 4e. Estatísticas do arquivo
    let meta = fs.stat("/tmp/hello.txt").await?;
    println!(
        "   ✓ stat:  {} bytes, mode={:o}",
        meta.size, meta.mode
    );

    // 4f. Copiar arquivo
    fs.copy("/tmp/hello.txt", "/tmp/hello_copy.txt").await?;
    println!("   ✓ copy:  /tmp/hello_copy.txt");

    // 4g. Renomear
    fs.rename("/tmp/hello_copy.txt", "/tmp/hello_renamed.txt").await?;
    println!("   ✓ rename: /tmp/hello_renamed.txt");

    // 4h. Remover
    fs.remove("/tmp/hello_renamed.txt").await?;
    println!("   ✓ remove: /tmp/hello_renamed.txt");

    // 4i. Verificar existência
    let exists = fs.exists("/tmp/hello.txt").await?;
    println!("   ✓ exists: {} -> {}", "/tmp/hello.txt", exists);
    println!();

    // ───────────────────────────────────────────────────────────
    // 5. Patches no rootfs (aplicados antes do boot)
    // ───────────────────────────────────────────────────────────
    println!("── 5. Rootfs patches ───────────────────────────────");

    // Nota: patches precisam ser definidos na criação.
    // Vamos criar uma segunda sandbox só para demonstrar.
    let sb_patched = Sandbox::builder("hello-patched")
        .image("alpine:latest")
        .cpus(1)
        .memory(256)
        .replace()
        .patch(|p| {
            p.text("/etc/motd", "Bem-vindo a sandbox patchada!\n", Some(0o644), true)
                .mkdir("/app/data", Some(0o755))
                .append("/etc/hosts", "127.0.0.1 meuservidor.local\n")
        })
        .create()
        .await?;

    let output = sb_patched.shell("cat /etc/motd").await?;
    println!("   motd: {}", output.stdout()?.trim());

    let output = sb_patched.shell("cat /etc/hosts | grep meuservidor").await?;
    println!("   hosts: {}", output.stdout()?.trim());

    let output = sb_patched.shell("ls -la /app/data").await?;
    println!("   /app/data: {}", output.stdout()?.trim());

    sb_patched.stop_and_wait().await?;
    Sandbox::remove("hello-patched").await?;
    println!("   ✓ sandbox 'hello-patched' removida\n");

    // ───────────────────────────────────────────────────────────
    // 6. Volumes (bind mount + tmpfs)
    // ───────────────────────────────────────────────────────────
    println!("── 6. Volumes ──────────────────────────────────────");

    let sb_vol = Sandbox::builder("hello-volumes")
        .image("alpine:latest")
        .cpus(1)
        .memory(256)
        .replace()
        // Bind mount: diretório local read-only
        .volume("/mnt/host", |v| v.bind(".").readonly())
        // Tmpfs: 10 MB em /scratch
        .volume("/scratch", |v| v.tmpfs().size(10))
        .create()
        .await?;

    let output = sb_vol.shell("ls /mnt/host/Cargo.toml 2>/dev/null && echo 'bind mount OK'").await?;
    println!("   bind mount: {}", output.stdout()?.trim());

    let output = sb_vol.shell("df -h /scratch | tail -1").await?;
    println!("   tmpfs: {}", output.stdout()?.trim());

    sb_vol.stop_and_wait().await?;
    Sandbox::remove("hello-volumes").await?;
    println!("   ✓ sandbox 'hello-volumes' removida\n");

    // ───────────────────────────────────────────────────────────
    // 7. Política de rede + Secrets
    // ───────────────────────────────────────────────────────────
    println!("── 7. Rede e Secrets ───────────────────────────────");

    use microsandbox::NetworkPolicy;

    let sb_net = Sandbox::builder("hello-network")
        .image("alpine:latest")
        .cpus(1)
        .memory(256)
        .replace()
        // Política: apenas internet pública (bloqueia ranges privados)
        .network(|n| n.policy(NetworkPolicy::public_only()))
        // Secret com placeholder: guest vê $MSB_API_KEY, nunca o valor real
        .secret_env("API_KEY", "sk-real-secret-123", "api.example.com")
        .create()
        .await?;

    // O placeholder substitui o valor real fora da VM
    let output = sb_net.shell("echo \"Minha chave: $API_KEY\"").await?;
    println!("   env var: {}", output.stdout()?.trim());
    println!("   (O guest NÃO vê o valor real do secret)");

    // DNS e conectividade básica
    let output = sb_net
        .shell("wget -q -O - http://example.com 2>&1 | head -3")
        .await?;
    let http_out = output.stdout()?;
    if !http_out.is_empty() {
        println!("   HTTP:\n{}", http_out.lines().map(|l| format!("          {l}")).collect::<Vec<_>>().join("\n"));
    } else {
        println!("   HTTP: (conteúdo vazio — domínio pode estar bloqueado)");
    }

    sb_net.stop_and_wait().await?;
    Sandbox::remove("hello-network").await?;
    println!("   ✓ sandbox 'hello-network' removida\n");

    // ───────────────────────────────────────────────────────────
    // 8. Port publishing
    // ───────────────────────────────────────────────────────────
    println!("── 8. Port publishing ──────────────────────────────");

    let sb_port = Sandbox::builder("hello-ports")
        .image("alpine:latest")
        .cpus(1)
        .memory(256)
        .replace()
        // Mapeia host:9090 -> guest:8080
        .port(9090, 8080)
        .create()
        .await?;

    let output = sb_port.shell("apk add -q lighttpd && echo 'servidor web instalado'").await?;
    println!("   {}", output.stdout()?.trim());

    sb_port.stop_and_wait().await?;
    Sandbox::remove("hello-ports").await?;
    println!("   ✓ sandbox 'hello-ports' removida\n");

    // ───────────────────────────────────────────────────────────
    // 9. Scripts injetados
    // ───────────────────────────────────────────────────────────
    println!("── 9. Scripts injetados ────────────────────────────");

    let sb_script = Sandbox::builder("hello-scripts")
        .image("alpine:latest")
        .cpus(1)
        .memory(256)
        .replace()
        // Script que pode ser executado com sb.shell("setup")
        .script("setup", r#"#!/bin/sh
echo "Script de setup executado em $(date)"
apk add -q curl
echo "Setup concluído"
"#)
        .create()
        .await?;

    let output = sb_script.shell("setup").await?;
    for line in output.stdout()?.lines() {
        println!("   {line}");
    }

    sb_script.stop_and_wait().await?;
    Sandbox::remove("hello-scripts").await?;
    println!("   ✓ sandbox 'hello-scripts' removida\n");

    // ───────────────────────────────────────────────────────────
    // 10. Métricas
    // ───────────────────────────────────────────────────────────
    println!("── 10. Métricas ────────────────────────────────────");

    let metrics = sb.metrics().await?;
    println!(
        "   CPU:      {:.1}%",
        metrics.cpu_percent
    );
    println!(
        "   Memória:  {} / {} bytes",
        metrics.memory_bytes, metrics.memory_limit_bytes
    );
    println!(
        "   Disco:    {} lidos, {} escritos",
        metrics.disk_read_bytes, metrics.disk_write_bytes
    );
    println!(
        "   Rede:     {} RX, {} TX",
        metrics.net_rx_bytes, metrics.net_tx_bytes
    );
    println!("   Uptime:   {:?}", metrics.uptime);
    println!();

    // ───────────────────────────────────────────────────────────
    // 11. Logs
    // ───────────────────────────────────────────────────────────
    println!("── 11. Logs ────────────────────────────────────────");

    let logs = sb.logs(&Default::default())?;
    println!("   {} entradas de log capturadas", logs.len());
    for entry in logs.iter().rev().take(5).rev() {
        let text = String::from_utf8_lossy(&entry.data);
        let preview = if text.len() > 80 {
            format!("{}...", &text[..80])
        } else {
            text.to_string()
        };
        println!("   [{:?}] {:?} | {}", entry.source, entry.timestamp, preview.trim());
    }
    println!();

    // ───────────────────────────────────────────────────────────
    // 12. Lifecycle: listagem, stop e remoção
    // ───────────────────────────────────────────────────────────
    println!("── 12. Ciclo de vida ───────────────────────────────");

    // Listar sandboxes ativas
    let all = Sandbox::list().await?;
    println!("   Sandboxes ativas:");
    for handle in &all {
        println!("      - {} ({:?})", handle.name(), handle.status());
    }

    // Obter handle (sem conexão viva)
    let handle = Sandbox::get("hello-sandbox").await?;
    println!("   Handle: name={}, status={:?}", handle.name(), handle.status());

    // Parada graciosa
    let exit_status = sb.stop_and_wait().await?;
    println!("   Sandbox parou com código {:?}", exit_status.code());

    // Remover do banco
    Sandbox::remove("hello-sandbox").await?;
    println!("   ✓ Sandbox 'hello-sandbox' removida");

    let remaining = Sandbox::list().await?;
    println!(
        "   Sandboxes restantes: {}",
        remaining.iter().map(|h| h.name().to_string()).collect::<Vec<_>>().join(", ")
    );

    println!();
    println!("╔══════════════════════════════════════════════════╗");
    println!("║       ✅ Tour completo!                         ║");
    println!("╚══════════════════════════════════════════════════╝");

    Ok(())
}
