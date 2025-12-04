// src/main.rs

mod game;
mod tui;
mod network;

// On importe les outils spécifiques dont `main` a besoin

use tui::run;

#[tokio::main]
async fn main() {
    // 3. On lance l'interface TUI
    if let Err(e) = run().await { // On lance la fonction `run` du module `tui`
        eprintln!("Une erreur est survenue: {}", e);
    }
}