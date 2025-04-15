use colored::*;
use std::process::Command;
use std::collections::HashSet;

pub async fn attempt_zone_transfer(domain: &str) {
    println!("{}", "🎣 Buscando nameservers...".blue());

    let ns_output = Command::new("dig")
        .arg(domain)
        .arg("NS")
        .arg("+short")
        .output()
        .expect("Erro ao executar dig para NS");

    let ns_raw = String::from_utf8_lossy(&ns_output.stdout);
    let ns_list: Vec<&str> = ns_raw.lines().collect();

    if ns_list.is_empty() {
        println!("{}", "❌ Nenhum servidor NS encontrado.".red());
        return;
    }

    let mut registros_unicos = HashSet::new(); // <- fora do loop, acumula geral
    let tipos_relevantes = ["IN A", "IN AAAA", "IN MX", "IN CNAME", "IN NS", "IN TXT"];

    for ns in ns_list {
        println!("\n🔎 Tentando transferência de zona via {}...", ns.trim());

        let output = Command::new("dig")
            .arg(format!("@{}", ns.trim()))
            .arg(domain)
            .arg("AXFR")
            .output();

        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);

                if stdout.contains("Transfer failed") || stdout.is_empty() {
                    println!("{} {}", "🛑 Transferência negada por".red(), ns.trim());
                } else {
                    println!("{}", "🚨 Transferência Permitida!".red().bold());

                    let mut novos_registros = vec![];

                    for line in stdout.lines() {
                        if line.trim().is_empty() || line.trim().starts_with(';') {
                            continue;
                        }

                        if line.contains("IN PTR")
                            || line.contains("localhost")
                            || line.contains("-ADDR.ARPA")
                            || line.contains("ip6.arpa")
                        {
                            continue;
                        }

                        if tipos_relevantes.iter().any(|&tipo| line.contains(tipo)) {
                            let clean = line
                                .replace(&format!("{}.", domain), "")
                                .replace(&format!(".{}.", domain), "")
                                .replace("\t", " ")
                                .replace("IN", "")
                                .trim()
                                .to_string();

                            if registros_unicos.insert(clean.clone()) {
                                novos_registros.push(clean);
                            }
                        }
                    }

                    if !novos_registros.is_empty() {
                        println!("{}", "🪧 Novos registros únicos encontrados:".green());
                        for r in novos_registros {
                            println!("   🟢 {}", r);
                        }
                    } else {
                        println!("{}", "📭 Nenhum novo registro encontrado nesse NS.".yellow());
                    }
                }
            }
            Err(_) => {
                println!("{} {}", "❌ Falha ao executar dig para".red(), ns.trim());
            }
        }
    }

    println!("\n{}", "✅ Transferências finalizadas. Todos os registros únicos foram exibidos.".blue().bold());
}
