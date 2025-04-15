use std::net::{ToSocketAddrs, SocketAddr};
use tokio::net::TcpStream;
use colored::*;

const COMMON_PORTS: [u16; 30] = [
    21, 22, 23, 25, 53, 80, 110, 143, 443, 3389,
    8080, 8443, 3306, 5432, 6379, 27017, 1521, 433,
    5000, 8000, 8888, 9200, 11211, 5900, 25465, 993,
    995, 389, 8081, 10000,
];

pub async fn scan_target(domain: &str) {
    println!("{}", "🔎 Resolvendo dominio...".blue());

    let addr = format!("{}:0", domain);
    let resolved: Vec<SocketAddr> = match addr.to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(_) => {
            println!("{}", "❌ Dominio inválido.".red());
            return;
        }
    };

    if resolved.is_empty() {
        println!("{}", "❌ Nenhum IP encontrado.".red());
        return;
    }

    let ip = resolved[0].ip();
    println!("🎣 IP encontrado: {}", ip.to_string().green());
    println!("{}", "😼 Iniciando escaneamento...".yellow());

    for port in COMMON_PORTS {
        let socket = format!("{}:{}", ip, port);
        let timeout = std::time::Duration::from_millis(800);

        let res = tokio::time::timeout(timeout, TcpStream::connect(&socket)).await;

        if let Ok(Ok(_)) = res {
            println!("{} {}:{} {}", "✅".green(), domain, port.to_string().bold(), "(aberto)".green());
        } else {
            println!("{} {}:{} {}", "🛑".red(), domain, port.to_string().dimmed(), "(fechado)".red());
        }
    }

    println!("{}", "✅ Escaneamento finalizado!".blue());
}
