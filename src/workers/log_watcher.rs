use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use tokio::time::{sleep, Duration};
use crate::workers::universal_commander::{UniversalCommander, UniversalTask};

pub async fn watch_logs_for_commands(log_path: &str) {
    let path = Path::new(log_path);
    
    // Așteptăm să apară fișierul dacă nu există
    while !path.exists() {
        sleep(Duration::from_millis(500)).await;
    }

    let file = File::open(path).expect("Nu am putut deschide fisierul de log");
    let mut reader = BufReader::new(file);
    
    // Mergem la finalul fișierului pentru a nu procesa comenzi vechi la startup
    let _ = reader.seek(SeekFrom::End(0));

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(bytes) if bytes > 0 => {
                if line.contains("COMMANDER_INPUT:") {
                    let json_part = line.split("COMMANDER_INPUT:").last().unwrap_or("").trim();
                    if let Ok(task) = serde_json::from_str::<UniversalTask>(json_part) {
                        println!(">>> [LOG_WATCHER] Comanda detectata! Delegare catre Commander...");
                        let _ = UniversalCommander::dispatch(task).await;
                    } else {
                        eprintln!(">>> [ERROR] JSON invalid detectat in log: {}", json_part);
                    }
                }
            }
            Ok(_) => {
                // Nu sunt linii noi, așteptăm puțin
                sleep(Duration::from_millis(500)).await;
            }
            Err(e) => {
                eprintln!(">>> [ERROR] Eroare la citirea logului: {:?}", e);
                sleep(Duration::from_secs(1)).await;
            }
        }
        line.clear();
    }
}
