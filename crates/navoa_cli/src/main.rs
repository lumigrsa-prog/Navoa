use colored::*;
use inquire::{Select, Text};
use navoa_core::{Language, NavoaEngine};
use std::{thread, time::Duration};

// Função para criar o efeito de animação de digitação letra a letra
fn animate_text(text: &str, delay_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        let _ = std::io::Write::flush(&mut std::io::stdout());
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

fn main() {
    let mut engine = NavoaEngine::new();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", "===============================================".bold().bright_cyan());
    println!("{}", "             NAVOA STUDIO CLI                  ".bold().bright_white());
    println!("{}", "===============================================\n".bold().bright_cyan());

    // Se ainda não estiver configurado, pede o idioma
    if !engine.config.configured {
        let options = vec![
            "Português (PT)",
            "English (EN)",
            "Español (ES)",
            "Français (FR)",
            "Italiano (IT)",
            "Deutsch (DE)",
        ];

        let choice = Select::new("Selecione o seu idioma / Select your language:", options)
            .prompt()
            .unwrap_or("Português (PT)");

        engine.config.language = match choice {
            "Português (PT)" => Language::PT,
            "English (EN)" => Language::EN,
            "Español (ES)" => Language::ES,
            "Français (FR)" => Language::FR,
            "Italiano (IT)" => Language::IT,
            "Deutsch (DE)" => Language::DE,
            _ => Language::PT,
        };

        engine.config.configured = true;
        engine.save_config();

        println!();
        animate_text(&engine.get_intro_text().bright_yellow().to_string(), 10);
        println!("{}", "-----------------------------------------------".dimmed());
    } else {
        // Se já estiver configurado, mostra o estado atual da história se o desafio não estiver concluído
        if !engine.config.challenge_completed {
            animate_text(&engine.get_intro_text().bright_yellow().to_string(), 8);
            println!();
        } else {
            println!("{}\n", "🏠 Bem-vindo de volta ao Apartamento do Navoa. O escritório está aberto. (Escreve 'sair' para terminar).".bright_green());
        }
    }

    // Consola interativa (REPL)
    loop {
        let input = Text::new("navoa>").prompt();

        match input {
            Ok(line) => {
                let trimmed = line.trim();
                if matches!(trimmed, "sair" | "exit" | "quitter" | "esci" | "ende") {
                    println!("{}", "Até breve / Goodbye!".bright_magenta());
                    break;
                }

                if matches!(trimmed, "config" | "settings" | "idioma") {
                    engine.config.configured = false;
                    engine.config.challenge_completed = false;
                    engine.save_config();
                    println!("{}", "Configuração reiniciada. Execute novamente o 'cargo run -p navoa_cli'.".yellow());
                    break;
                }

                if !trimmed.is_empty() {
                    let result = engine.execute(trimmed);
                    if !result.is_empty() {
                        println!("{}", result);
                    }
                }
            }
            Err(_) => break,
        }
    }
}
