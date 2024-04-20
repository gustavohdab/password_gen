use chrono::Local;
use clipboard_win::{formats, set_clipboard};
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process;

/// Struct que representa o histórico de senhas
struct PasswordHistory {
    history: HashMap<String, String>,
}

impl PasswordHistory {
    /// Cria uma nova instância de PasswordHistory
    fn new() -> PasswordHistory {
        PasswordHistory {
            history: HashMap::new(),
        }
    }

    /// Adiciona uma senha ao histórico
    fn add_password(&mut self, password: String) {
        let timestamp = Local::now().to_string();
        self.history.insert(timestamp.clone(), password.clone()); // Clonando a senha antes de inserir no hashmap
        self.save_history(&timestamp, &password);
    }

    /// Salva o histórico de senhas em um arquivo
    fn save_history(&self, timestamp: &str, password: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("password_history.txt")
        {
            if let Err(_) = writeln!(&mut file, "{}: {}", timestamp, password) {
                println!("Falha ao salvar o histórico de senhas.");
            }
        } else {
            println!("Falha ao abrir o arquivo de histórico de senhas.");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut history = PasswordHistory::new();

    if args.len() != 1 {
        writeln!(io::stderr(), "Uso: password_gen").unwrap();
        process::exit(1);
    }

    loop {
        let length: usize = ask_password_length();
        let password: String = generate_password(length);
        println!("{}", password);
        set_clipboard(formats::Unicode, &password).unwrap();
        history.add_password(password.clone());

        let mut input = String::new();
        println!(
            "Digite '/history' para ver o histórico de senhas, '/exit' para sair, ou pressione Enter para gerar outra senha:"
        );
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "/history" => {
                if let Ok(file) = File::open("password_history.txt") {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        println!("{}", line.unwrap());
                    }
                } else {
                    println!("Nenhum histórico de senhas encontrado.");
                }
            }
            "/exit" => break,
            _ => continue,
        }
    }
}

/// Pede ao usuário o comprimento da senha desejada
fn ask_password_length() -> usize {
    let mut input: String = String::new();
    println!("Digite o comprimento da senha:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

/// Gera uma senha aleatória com o comprimento especificado
fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    for _ in 0..length {
        let choice: i32 = rng.gen_range(0..4);
        match choice {
            0 => password.push(rng.gen_range(48_u8..58_u8) as char), // Números (ASCII 48-57)
            1 => password.push(rng.gen_range(65_u8..91_u8) as char), // Letras maiúsculas (ASCII 65-90)
            2 => password.push(rng.gen_range(97_u8..123_u8) as char), // Letras minúsculas (ASCII 97-122)
            3 => password.push(rng.gen_range(33_u8..48_u8) as char),  // Símbolos (ASCII 33-47)
            _ => panic!("Escolha inválida"),
        }
    }
    password
}
