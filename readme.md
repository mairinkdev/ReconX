# 🎣 ReconX – Scanner de Rede e DNS Recon

> ⚠️ **Aviso:** Esta ferramenta foi desenvolvida **exclusivamente para fins educacionais e de aprendizado**.
> O uso desta ferramenta em redes ou domínios sem autorização pode ser ilegal.
> O autor não se responsabiliza por quaisquer usos indevidos. Use com responsabilidade e ética.

##

ReconX é uma ferramenta escrita em **Rust** para escaneamento de portas TCP e detecção de falhas em configurações DNS via **Zone Transfer (AXFR)**.
Foi feita para ser leve, rápida, prática — e **bonita no terminal**.

---

## ⚙️ Funcionalidades

- 🔍 Escaneamento assíncrono de **portas TCP mais comuns**
- 🌐 Resolução de domínio com detecção de IP público
- 🔓 Tentativa automatizada de **transferência de zona DNS (AXFR)**
- 🧼 Filtro profissional de registros DNS (A, MX, CNAME, etc.)
- 🎨 Saída colorida e intuitiva, com ícones, indentação e visual terminal

---

## Como usar

cargo run -- --target <domínio ou IP> [--dns-zone-transfer]

## Exemplos:

Escanear portas TCP de httpbin.org
cargo run -- --target httpbin.org

Tentar AXFR no domínio zonetransfer.me
cargo run -- --target zonetransfer.me --dns-zone-transfer


# Output Exemplo

🎣 Buscando nameservers...

🔎 Tentando transferência de zona via nsztm2.digi.ninja....
🚨 Transferência Permitida!
🪧 Novos registros únicos encontrados:
   🟢 canberra-office. 7200  A 202.14.81.230
   🟢 alltcpportsopen.firewall.test. 301  A 127.0.0.1

🔎 Tentando transferência de zona via nsztm1.digi.ninja....
🚨 Transferência Permitida!
📭 Nenhum novo registro encontrado nesse NS.

✅ Transferências finalizadas. Todos os registros únicos foram exibidos.

## Instalação

Requisitos:

Rust + Cargo

Linux, macOS ou Windows com dig instalado (necessário para AXFR)

git clone https://github.com/mairinkdev/reconx
cd reconx
cargo run -- --target example.com

👨‍💻 Projeto de @mairinkdev
