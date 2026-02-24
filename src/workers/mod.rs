// Definirea modulelor ca subordonate
pub mod universal_commander; // Masterul
pub mod ai_bridge;           // UnaltÄƒ: InterfaÈ›Äƒ LLM
pub mod build_worker;        // UnaltÄƒ: Cargo Operations
pub mod file_watcher;        // UnaltÄƒ: Monitorizare fiÈ™iere
pub mod log_watcher;         // UnaltÄƒ: Monitorizare loguri
pub mod rag_indexer;         // UnaltÄƒ: BazÄƒ de cunoÈ™tinÈ›e (RAG)
pub mod telegram;            // UnaltÄƒ: InterfaÈ›Äƒ Comunicare

// Re-exportÄƒm entitÄƒÈ›ile cheie pentru a fi accesibile din main.rs sau orchestrator
pub use universal_commander::{UniversalCommander, UniversalTask};

/// Tip de date pentru a trimite comenzi rapide cÄƒtre orice worker prin Master
pub enum WorkerCommand {
    StartBuild(String),      // Path
    AskAI(String, String),   // Context, Prompt
    NotifyUser(String),      // Mesaj Telegram
    IndexDocs(String),       // Path pentru RAG
}
