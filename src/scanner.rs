use std::net::{ToSocketAddrs, SocketAddr};
use tokio::net::TcpStream;
use colored::*;
use tokio;

const COMMON_PORTS: [u16; 30] = [
    21, 22, 23, 25, 53, 80, 110, 143, 443, 3389,
    8080, 8443, 3306, 5432, 6379, 27017, 1521, 433,
    5000, 8000, 8888, 9200, 11211, 5900, 25465, 993,
    995, 389, 8081, 10000,
];

pub async fn scan_target(domain: &str, port_arg: &Option<String>, only_open: bool) {
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

    let ports = match parse_ports(port_arg) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", format!("❌ Erro ao interpretar as portas: {}", e).red());
            return;
        }
    };

    let mut open_ports = Vec::new();

    let tasks: Vec<_> = ports.into_iter().map(|port| {
        let ip = ip.clone();
        let domain = domain.to_string();
        tokio::spawn({
            let domain = domain.to_string();
            async move {
                let socket = format!("{}:{}", ip, port);
                let timeout = std::time::Duration::from_millis(800);
                let res = tokio::time::timeout(timeout, TcpStream::connect(&socket)).await;
                let is_open = matches!(res, Ok(Ok(_)));
                (port, is_open, domain)
            }
        })
    }).collect();

    for task in tasks {
        if let Ok((port, is_open, domain)) = task.await {
            if is_open {
                println!("{} {}:{} {}", "✅".green(), domain, port.to_string().bold(), "(aberto)".green());
                open_ports.push(port);
            } else if !only_open {
                println!("{} {}:{} {}", "🛑".red(), domain, port.to_string().dimmed(), "(fechado)".red());
            }
        }
    }

    println!("{}", "✅ Escaneamento finalizado!".blue());
}

fn parse_ports(input: &Option<String>) -> Result<Vec<u16>, String> {
    if let Some(raw) = input {
        if raw.contains(',') {
            let mut ports = Vec::new();
            for part in raw.split(',') {
                let port = part.trim().parse::<u16>()
                    .map_err(|_| format!("Porta inválida: {}", part))?;
                ports.push(port);
            }
            Ok(ports)
        } else if raw.contains('-') {
            let parts: Vec<&str> = raw.split('-').collect();
            if parts.len() != 2 {
                return Err("Range inválido. Use o formato: 1000-1010".to_string());
            }
            let start = parts[0].trim().parse::<u16>().map_err(|_| "Início inválido")?;
            let end = parts[1].trim().parse::<u16>().map_err(|_| "Fim inválido")?;
            if start > end {
                return Err("Range invertido".to_string());
            }
            Ok((start..=end).collect())
        } else {
            let port = raw.trim().parse::<u16>().map_err(|_| "Porta inválida")?;
            Ok(vec![port])
        }
    } else {
        Ok(COMMON_PORTS.to_vec())
    }
}
