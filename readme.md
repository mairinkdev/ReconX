# 🎣 ReconX – Scanner de Rede e DNS Recon

> ⚠️ **Aviso:** Esta ferramenta foi desenvolvida **exclusivamente para fins educacionais e de aprendizado**.
> O uso desta ferramenta em redes ou domínios sem autorização pode ser ilegal.
> O autor não se responsabiliza por quaisquer usos indevidos. Use com responsabilidade e ética.

---

ReconX é uma ferramenta escrita em **Rust** para escaneamento de portas TCP e detecção de falhas em configurações DNS via **Zone Transfer (AXFR)**.
Foi feita para ser leve, rápida e prática.

---

## ⚙️ Funcionalidades

- 🔍 Escaneamento assíncrono de **portas TCP mais comuns** ou por **faixa personalizada** (ex: `--ports 20-30`)
- 🧠 Filtro de resultados para mostrar **apenas portas abertas** (`--only-open`)
- 💾 Exportação opcional dos resultados em **.txt**, **.json** ou **.csv** (`--output resultado.txt`)
- 🌐 Resolução de domínio com detecção de IP público
- 🔓 Tentativa automatizada de **transferência de zona DNS (AXFR)**
- 🧼 Filtro profissional de registros DNS (A, MX, CNAME, etc.)
- 🔁 Resolução reversa (PTR) dos IPs encontrados
- 🎨 Saída colorida e intuitiva, com ícones, indentação e visual terminal

---

## ▶️ Como usar

```sh
cargo run -- --target <domínio ou IP> [opções]
```

### Exemplos:

- Escanear portas TCP de httpbin.org:

```sh
cargo run -- --target httpbin.org
```

- Escanear apenas portas abertas:

```sh
cargo run -- --target httpbin.org --only-open
```

- Especificar um range de portas:

```sh
cargo run -- --target example.com --ports 1000-1050
```

- Exportar para txt/json:

```sh
cargo run -- --target example.com --only-open --output resultado.txt
cargo run -- --target example.com --only-open --output resultado.json
```

- Tentar AXFR no domínio zonetransfer.me:

```sh
cargo run -- --target zonetransfer.me --dns-zone-transfer
```

---

## 💡 Exemplo de Output em Arquivo

### 📄 `resultado.txt`

```
📄 Relatório de Escaneamento - ReconX
Alvo: example.com
IP Resolvido: 93.184.216.34
Data: 2025-04-15 15:32:07

Portas escaneadas:

✅ Porta 80    - aberta
❌ Porta 21    - fechada
❌ Porta 22    - fechada
✅ Porta 443   - aberta

👨‍💻 Gerado com ReconX • github.com/mairinkdev
```

---

### 📦 `resultado.json`

```json
{
  "alvo": "example.com",
  "ip_resolvido": "93.184.216.34",
  "data": "2025-04-15T18:32:07Z",
  "resultado": [
    {
      "porta": 80,
      "status": "aberta",
      "emoji": "✅"
    },
    {
      "porta": 21,
      "status": "fechada",
      "emoji": "❌"
    },
    {
      "porta": 443,
      "status": "aberta",
      "emoji": "✅"
    }
  ],
  "creditos": "https://github.com/mairinkdev"
}
```

---

## 📦 Instalação

### Requisitos:
- [Rust](https://www.rust-lang.org/tools/install) + Cargo
- Linux, macOS ou Windows com `dig` instalado (necessário para AXFR)

```sh
git clone https://github.com/mairinkdev/reconx
cd reconx
cargo run -- --target example.com
```

---

👨‍💻 Projeto de [@mairinkdev](https://github.com/mairinkdev)
