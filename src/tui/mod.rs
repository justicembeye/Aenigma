mod app;
mod ui;
mod events;

use app::{App};
use ui::{ui};
use events::{handle_events};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend},
    Terminal,
};
use std::io;



// --- LA FONCTION RUN ---
pub async fn run() -> io::Result<()> {
    // 1. Initialisation du terminal
    let mut terminal = init_terminal()?;

    // 2. Création de l'application
    let mut app = App::new();

    // 3. Boucle principale
    while app.running {

        // Étape A : Dessiner l'interface
        terminal.draw(|f| ui(f, &mut app))?;

        // Étape B : Gérer les événements
        handle_events(&mut app).await?;
        crate::tui::events::handle_network_events(&mut app).await?;
    }

    // 4. Restauration du terminal
    restore_terminal(&mut terminal)?;

    Ok(())
}

// --- Fonctions "assistantes" pour garder le code propre ---
fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

