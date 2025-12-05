use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Clear};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use crate::tui::app::{App, AppState, Focus};

pub fn ui(f: &mut Frame, app: &mut App) {

    // ... layout principal ...
    // Calcul du temps écoulé
    let elapsed = if let Some(start) = app.start_time {
        start.elapsed()
    } else {
        std::time::Duration::from_secs(0)
    };
    let timer_str = format!("{:02}:{:02}", elapsed.as_secs() / 60, elapsed.as_secs() % 60);
    let title = format!("🌏 Aenigma - ⏱️ {}", timer_str);

    // ... layout principal ...
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(title);

    // On clone la zone principale pour le réutiliser
    let main_zone = main_block.clone();

    // On dessine le cadre principal
    f.render_widget(main_zone, f.area());

    // Calcul de la zone a l'intérieur
    let inner_area = main_block.inner(f.area());



    match app.current_state {
        AppState::Welcome => draw_welcome_ui(f, app, inner_area),
        AppState::Setup => draw_setup_ui(f, app, inner_area),
        AppState::SetupEnterPlayerName => {
            let title = format!("🎮 Player {} ", app.players.len() + 1);
            draw_input_screen(f, app, inner_area, title.as_str(),   "👤 Nom: ");
        }
        AppState::SetupSelectTheme => {
            draw_theme_selection_ui(f, app, inner_area);
        }
        AppState::SetupEnterSecretWord => {
            let title = format!("👤 {} ", app.current_setup_name);
            draw_input_screen(f, app, inner_area, title.as_str(), "🧩 Mot Secret: ");
        }
        AppState::Playing | AppState::SelectTarget | AppState::InputLetter | AppState::InputGuess | AppState::TurnResult | AppState::Paused | AppState::Rules => {
            draw_playing_ui(f, app, inner_area);
            if app.current_state == AppState::Paused {
                draw_pause_popup(f, app, inner_area);
            } else if app.current_state == AppState::Rules {
                draw_rules_popup(f, app, inner_area);
            }
        }
        AppState::MultiplayerMenu => {
            draw_welcome_ui(f, app, inner_area); // On réutilise le style du menu principal pour l'instant
        }
        AppState::JoinGame => {
             draw_input_screen(f, app, inner_area, "Rejoindre une partie", "Entrez l'adresse IP du serveur (ex: 127.0.0.1:8080) :");
        }
        AppState::Lobby => {
            draw_lobby_ui(f, app, inner_area);
        }
        AppState::GameOver => {
            draw_game_over_ui(f, app, inner_area);
        }
    }
}

fn draw_setup_ui(f: &mut Frame, app: &mut App, area: Rect) {
    // 1. On crée une zone centrale pour la configuration
    let config_area = create_centered_rect(area, 60, 80);

    // 2. Titre de l'étape en fonction du step
    let (step_title, step_index) = match app.setup_step {
        0 => ("Nombre de joueurs", "1/3"),
        1 => ("Niveau de Difficulté", "2/3"),
        2 => ("Choix du Thème", "3/3"),
        _ => ("Configuration", "?/?"),
    };

    let title = format!(" Configuration - Étape {} : {} ", step_index, step_title);

    // 3. Bloc principal
    let config_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 1));

    f.render_widget(config_block.clone(), config_area);

    // 4. Zone de la liste (avec padding)
    let list_area = config_block.inner(config_area);

    if app.setup_step == 0 {
        // --- MODE GRILLE POUR LE CHOIX DU NOMBRE DE JOUEURS ---
        let items = &app.setup_list_items;
        let count = items.len();
        if count == 0 { return; }

        // Grille dynamique : 2 colonnes
        let cols = 2;
        let rows = (count as f32 / cols as f32).ceil() as usize;

        // On divise la zone en lignes
        let row_constraints: Vec<Constraint> = (0..rows).map(|_| Constraint::Length(3)).collect();
        
        // On centre verticalement la grille
        let grid_vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1), // Espace haut
                Constraint::Length((rows * 3) as u16), // Hauteur de la grille
                Constraint::Min(1), // Espace bas
            ])
            .split(list_area);
            
        let grid_area = grid_vertical_layout[1];

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(grid_area);

        for (i, item_text) in items.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;

            if row < vertical_layout.len() {
                let row_area = vertical_layout[row];
                let col_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50), 
                        Constraint::Length(1), // Espace entre colonnes
                        Constraint::Percentage(50)
                    ])
                    .split(row_area);
                
                // col 0 -> index 0, col 1 -> index 2 (car index 1 est l'espace)
                let col_index = if col == 0 { 0 } else { 2 };

                if col_index < col_layout.len() {
                    let is_selected = app.setup_list_state.selected() == Some(i);
                    let border_color = if is_selected { Color::Cyan } else { Color::DarkGray };
                    let text_style = if is_selected { 
                        Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) 
                    } else { 
                        Style::default().fg(Color::Gray) 
                    };

                    let button_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .style(Style::default().fg(border_color));

                    let paragraph = Paragraph::new(item_text.clone())
                        .block(button_block)
                        .style(text_style)
                        .alignment(ratatui::layout::Alignment::Center);

                    f.render_widget(paragraph, col_layout[col_index]);
                }
            }
        }
    } else {
        // --- MODE BOUTONS POUR LA DIFFICULTÉ ---
        // On centre la liste verticalement si possible
        let items_count = app.setup_list_items.len();
        
        // On crée un layout vertical centré
        let constraints: Vec<Constraint> = (0..items_count).map(|_| Constraint::Length(3)).collect();
        
        // On ajoute des marges flexibles haut/bas pour centrer
        let vertical_centering = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length((items_count * 3) as u16),
                Constraint::Min(1),
            ])
            .split(list_area);
            
        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(vertical_centering[1]);

        for (i, item) in app.setup_list_items.iter().enumerate() {
            let is_selected = app.setup_list_state.selected() == Some(i);
            
            let border_color = if is_selected { Color::Cyan } else { Color::DarkGray };
            let text_style = if is_selected { 
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) 
            } else { 
                Style::default().fg(Color::Gray) 
            };

            let button_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(border_color));

            let paragraph = Paragraph::new(item.as_str())
                .block(button_block)
                .style(text_style)
                .alignment(ratatui::layout::Alignment::Center);
            
            if i < list_layout.len() {
                f.render_widget(paragraph, list_layout[i]);
            }
        }
    }
}

fn draw_welcome_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Titre
            Constraint::Percentage(60), // Menu
        ])
        .split(area);

    let title_text = vec![
        "    _    _____ _   _ ___ ____ __  __    _    ",
        "   / \\  | ____| \\ | |_ _/ ___|  \\/  |  / \\   ",
        "  / _ \\ |  _| |  \\| || | |  _| |\\/| | / _ \\  ",
        " / ___ \\| |___| |\\  || | |_| | |  | |/ ___ \\ ",
        "/_/   \\_\\_____|_| \\_|___\\____|_|  |_/_/   \\_\\",
        "",
        "             Le Jeu de Déduction et de Stratégie             ",
    ];

    let title_paragraph = Paragraph::new(title_text.join("\n"))
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    f.render_widget(title_paragraph, chunks[0]);

    // Menu
    let menu_area = create_centered_rect(chunks[1], 40, 40);
    
    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(" Menu Principal ")
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 1)); // Padding: Left, Right, Top, Bottom
    
    f.render_widget(menu_block.clone(), menu_area);

    let inner_menu = menu_block.inner(menu_area);

    let items: Vec<ListItem> = app
        .welcome_list_items
        .iter()
        .map(|s| ListItem::new(s.clone()))
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("➤ ");

    f.render_stateful_widget(list, inner_menu, &mut app.welcome_list_state);
}

fn draw_lobby_ui(f: &mut Frame, _app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Lobby ")
        .border_type(ratatui::widgets::BorderType::Rounded);

    let center_area = create_centered_rect(area, 60, 20);
    f.render_widget(Clear, center_area);
    f.render_widget(block, center_area);

    let text = vec![
        Line::from(Span::styled("En attente de l'hôte...", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("L'hôte configure la partie."),
        Line::from("Le jeu se lancera automatiquement."),
        Line::from(""),
        Line::from(Span::styled("Appuyez sur Echap pour quitter", Style::default().fg(Color::DarkGray))),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    let inner_area = center_area.inner(ratatui::layout::Margin { vertical: 4, horizontal: 2 });
    f.render_widget(paragraph, inner_area);
}

fn draw_rules_popup(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title(" Règles & Commandes ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Green));

    let area = create_centered_rect(area, 70, 60);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let text = vec![
        Line::from(Span::styled("But du Jeu :", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("Devinez le mot secret de vos adversaires avant qu'ils ne trouvent le vôtre !"),
        Line::from(""),
        Line::from(Span::styled("Déroulement :", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("1. Proposez une lettre pour révéler des indices."),
        Line::from("2. Si la lettre est dans le mot adverse, elle est révélée."),
        Line::from("3. Devinez le mot complet si vous pensez l'avoir trouvé."),
        Line::from(""),
        Line::from(Span::styled("Commandes :", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("TAB       : Changer de menu (Actions / Système)"),
        Line::from("Flèches   : Naviguer dans les menus"),
        Line::from("Entrée    : Valider / Sélectionner"),
        Line::from("Echap     : Pause / Retour"),
        Line::from(""),
        Line::from(Span::styled("Multijoueur :", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("Chacun son tour ! Vous ne pouvez jouer que quand c'est à vous."),
        Line::from("L'hôte voit quand un joueur gagne."),
        Line::from(""),
        Line::from(Span::styled("Appuyez sur Echap pour revenir", Style::default().fg(Color::DarkGray))),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(Block::default().borders(Borders::NONE).padding(ratatui::widgets::Padding::new(2, 2, 1, 1)));

    let inner_area = area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 1 });
    f.render_widget(paragraph, inner_area);
}

fn draw_game_over_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .style(Style::default().fg(Color::Green))
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 1)); // Padding interne
    
    f.render_widget(block.clone(), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Marge haut
            Constraint::Length(3), // Titre
            Constraint::Length(2), // Espace
            Constraint::Length(3), // Durée
            Constraint::Length(2), // Espace
            Constraint::Min(5),    // Tableau des scores
            Constraint::Length(1), // Espace
            Constraint::Length(3), // Instructions
            Constraint::Length(1), // Marge bas
        ])
        .split(block.inner(area));

    // 1. Titre (Index 1)
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double) // Plus stylé
        .style(Style::default().fg(Color::Yellow));
    
    let title = Paragraph::new("RAPPORT DE MISSION")
        .block(title_block)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[1]);

    if let Some(game) = &app.game {
        // 2. Durée (Index 3)
        // On utilise la durée finale si elle est définie, sinon le temps écoulé (cas de fallback)
        let duration = game.final_duration.unwrap_or_else(|| game.start_time.elapsed());
        let duration_secs = duration.as_secs();
        let minutes = duration_secs / 60;
        let seconds = duration_secs % 60;
        let stats_text = format!("⏱️ Durée : {:02}:{:02}   |   🔄 Tours : {}", minutes, seconds, game.get_current_round());
        
        let duration_paragraph = Paragraph::new(stats_text)
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(duration_paragraph, chunks[3]);

        // 3. Tableau des scores (Index 5)
        // On trie les joueurs par score décroissant
        let mut sorted_players = game.players.clone();
        sorted_players.sort_by(|a, b| b.score.cmp(&a.score));

        let header_cells = ["Rang", "Agent", "Statut", "Score", "Tours", "Éliminations"]
            .iter()
            .map(|h| ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Yellow)));
        let header = ratatui::widgets::Row::new(header_cells)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .height(1)
            .bottom_margin(1);

        let rows = sorted_players.iter().enumerate().map(|(i, p)| {
            let rank = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };
            
            let status = if p.is_eliminated {
                Span::styled("ÉLIMINÉ", Style::default().fg(Color::Red))
            } else {
                Span::styled("ACTIF", Style::default().fg(Color::Green))
            };

            let cells = vec![
                ratatui::widgets::Cell::from(rank),
                ratatui::widgets::Cell::from(p.name.as_str()),
                ratatui::widgets::Cell::from(status),
                ratatui::widgets::Cell::from(p.score.to_string()),
                ratatui::widgets::Cell::from(p.turns_played.to_string()),
                ratatui::widgets::Cell::from(p.correct_guesses.to_string()),
            ];
            ratatui::widgets::Row::new(cells).height(2) // Espace vertical entre les lignes
        });

        let table = ratatui::widgets::Table::new(
            rows,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(25),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Classement Final ").padding(ratatui::widgets::Padding::new(1, 1, 1, 1))); // Padding interne au tableau
        
        f.render_widget(table, chunks[5]);
    }

    // 4. Instructions (Index 7)
    let instructions = Paragraph::new("Appuyez sur Entrée pour retourner au QG")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(instructions, chunks[7]);
}

fn draw_input_screen(f: &mut Frame, app: &mut App, area: Rect, parent_title: &str, input_title: &str) {
    // Layout principal (2 colonnes)
    let chunks = create_two_column_layout(area);
    let right_chunk = chunks[1];

    // Bloc parent (le grand encadré "Player 1")
    let block_parent_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(right_chunk);

    let parent_area = block_parent_chunks[1];

    let parent_block = Block::default()
        .borders(Borders::ALL)
        .title(parent_title)
        .border_type(ratatui::widgets::BorderType::Rounded);
    let inner_parent_area = parent_block.inner(parent_area);
    f.render_widget(parent_block, parent_area);

    // ⚙️ Voici la partie critique : layout du bloc input + message d’erreur
    // On crée 3 zones verticales dans le parent : [haut] [input + erreur] [bas]
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // petit espace au-dessus
            Constraint::Min(5),    // zone pour input + erreur
            Constraint::Length(1), // petit espace en bas
        ])
        .split(inner_parent_area);

    let work_area = inner_layout[1];

    // 🔸 Dans cette zone, on fait deux sous-zones : [input] [erreur]
    let input_and_error_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // hauteur fixe pour le bloc input
            Constraint::Length(2), // juste en dessous : zone erreur
        ])
        .split(work_area);

    let input_area = input_and_error_chunks[0];
    let error_area = input_and_error_chunks[1];

    // Bloc de saisie
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_title)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .border_type(ratatui::widgets::BorderType::Rounded);

    let inner_input_area = input_block.inner(input_area);
    f.render_widget(input_block, input_area);

    // Texte d’entrée
    let text_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(1),
            Constraint::Percentage(40),
        ])
        .split(inner_input_area);
    let text_area = text_chunks[1];

    f.render_widget(Paragraph::new(app.input.as_str()), text_area);
    f.set_cursor_position((text_area.x + app.cursor_position as u16, text_area.y));

    // 🔻 Message d’erreur (collé sous le bloc orange)
    // On priorise l'erreur de saisie locale, sinon l'erreur globale
    if let Some(ref msg) = app.input_error.as_ref().or(app.error_message.as_ref()) {
        // On calcule la marge horizontale du bloc input (ici : UN caractère)
        // Et on la réapplique au message d’erreur.
        let adjusted_error_area = Rect {
            x: error_area.x + 1,        // petit décalage à droite
            y: error_area.y,
            width: error_area.width.saturating_sub(1),
            height: error_area.height,
        };

        let error_paragraph = Paragraph::new(Span::styled(
            format!("⚠️ {}", msg),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(error_paragraph, adjusted_error_area);
    }


}

/*
fn draw_input_screen(f: &mut Frame, app: &mut App, area: Rect, parent_title: &str, input_title: &str) {
    // On utilise la fonction de layout qu'on a créée pour les 2 colonnes
    let chunks = create_two_column_layout(area);


    // C'est notre zone de travail principale
    let right_chunk = chunks[1];


    /*
    // (Visualisation des colonnes)
    f.render_widget(Block::default().borders(Borders::ALL), left_chunk);
    f.render_widget(Block::default().borders(Borders::ALL), right_chunk);
    */

    // 1. On crée le premier bloc centré dans la colonne de droite, comme tu l'avais fait
    let block_parent_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(right_chunk);

    // On récupère le rectangle de la zone du bloc parent
    let parent_area = block_parent_chunks[1];

    // 2. On définit et dessine le bloc parent en utilisant le paramètre `parent_title`
    let parent_block = Block::default()
        .borders(Borders::ALL)
        .title(parent_title)
        .border_type(ratatui::widgets::BorderType::Rounded);

    // Calcule de l'aire intérieure
    let inner_parent_area = parent_block.inner(parent_area);

    // On dessine le bloc parent
    f.render_widget(parent_block, parent_area);

    // 3. On crée le bloc de saisie à l'intérieur du bloc parent

    // Split de la zone de travail en 3 parties : Vertical
    let input_block_chunks_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // un petit espace au-dessus
            Constraint::Min(3),      // zone d'input
            Constraint::Length(2),   // juste assez pour le message d'erreur
        ])
        .split(inner_parent_area);


    // Split de la zone de travail en 3 parties : Horizontal
    let input_block_chunks_horizontal =  Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(92),
        ])
        .split(input_block_chunks_vertical[1]);

    // On récupère le rectangle de la zone de saisie
    let input_area = input_block_chunks_horizontal[0];

    // 4. On définit et dessine le bloc de saisie avec le style de focus et `input_title`
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_title)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .border_type(ratatui::widgets::BorderType::Rounded);

    // Calcule de l'aire intérieure
    let inner_input_area = input_block.inner(input_area);

    // On dessine le bloc de saisie
    f.render_widget(input_block, input_area);

    // 5. On crée la zone de texte d'une seule ligne, COMME TU L'AVAIS FAIT
    let text_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(1),
            Constraint::Percentage(40),
        ])
        .split(inner_input_area);
    let text_area = text_chunks[1];

    // 6. On dessine le paragraphe et le curseur
    let error_area = input_block_chunks_vertical[2];

    if let Some(ref msg) = app.error_message {
        let error_paragraph = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(error_paragraph, error_area);
    }

    f.render_widget(Paragraph::new(app.input.as_str()), text_area);
    f.set_cursor(text_area.x + app.cursor_position as u16, text_area.y);

}
*/
fn create_two_column_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(area)
        .to_vec()
}

fn create_centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {

    // 1. Crée un layout VERTICAL pour centrer sur l'axe Y.
    // Comment calcules-tu la contrainte pour les bords
    // afin qu'ils prennent l'espace restant autour de `percent_y` ?
    // (Indice : la formule est `(100 - percent_y) / 2`)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2)
        ].as_ref())
        .split(area);

    // 2. Prends la zone du milieu

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]).split(vertical_chunks[1]);
    // 3. Retourne le rectangle central final.
    horizontal_chunks[1]
}

fn draw_playing_ui(f: &mut Frame, app: &mut App, area: Rect) {
    // Layout principal : 3 colonnes verticales avec espacement
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Gauche
            Constraint::Length(1),      // Espace
            Constraint::Percentage(40), // Centre
            Constraint::Length(1),      // Espace
            Constraint::Percentage(30), // Droite
        ])
        .margin(1) // Marge globale
        .split(area);

    let left_area = chunks[0];
    // chunks[1] est l'espace
    let center_area = chunks[2];
    // chunks[3] est l'espace
    let right_area = chunks[4];

    // --- ZONE GAUCHE : IDENTITÉ & MENACE ---
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Carte d'Agent
            Constraint::Min(10),    // Menace & Secret
        ])
        .split(left_area);

    // 1. Carte d'Agent
    let (player_name, player_score, energy, my_secret_word, progress_map, player_id) = if let Some(game) = &app.game {
        let p = game.current_player();
        (p.name.clone(), p.score, p.energy, p.secret_word.content.clone(), p.progress_on_opponents.clone(), p.id)
    } else {
        ("Inconnu".to_string(), 0, 0, "???".to_string(), std::collections::HashMap::new(), 0)
    };

    let agent_block = Block::default()
        .title(" 🕵️  Carte d'Agent ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .style(Style::default().fg(Color::Cyan));
    
    // Jauge d'Énergie visuelle (10 blocs)
    let buzz_bar: String = (0..10).map(|i| if i * 10 < energy { "█" } else { "░" }).collect();
    let _buzz_status = if energy >= 100 { "PRÊT !" } else { "Charge..." };
    
    let agent_info = format!(
        "\n  Nom     : {}\n  Score   : {} pts\n  Énergie : [{}] {}%",
        player_name, player_score, buzz_bar, energy
    );
    
    let agent_paragraph = Paragraph::new(agent_info).block(agent_block);
    f.render_widget(agent_paragraph, left_chunks[0]);

    // 2. Menace & Secret
    let threat_block = Block::default()
        .title(" ⚠️  Niveau de Menace ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White)); // Bordure neutre
    
    let threat_level = if let Some(game) = &app.game {
        game.get_threat_level(player_id)
    } else {
        "???".to_string()
    };

    // Formatage espacé pour le mot secret
    let is_ai = if let Some(game) = &app.game {
        if let Some(p) = game.players.iter().find(|p| p.id == player_id) {
            matches!(p.control_type, crate::game::ControlType::AI)
        } else {
            false
        }
    } else {
        false
    };

    let spaced_secret: String = if is_ai {
        "C O N F I D E N T I E L".to_string()
    } else {
        my_secret_word.chars().map(|c| format!("{} ", c)).collect()
    };
    
    // Construction des lignes avec styling dynamique
    // Ligne 1 : "VOTRE SECRET : M O T"
    let secret_line = Line::from(vec![
        Span::styled("  VOTRE SECRET : ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(spaced_secret, Style::default().fg(if is_ai { Color::DarkGray } else { Color::Cyan })), 
    ]);

    // Ligne 2 : Vide
    let empty_line = Line::from("");

    // Ligne 3 : "CE QU'ILS SAVENT : M _ T" (Lettres en rouge)
    let mut threat_spans = vec![
        Span::styled("  CE QU'ILS SAVENT : ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ];

    for c in threat_level.chars() {
        let s = format!("{} ", c);
        if c.is_alphabetic() {
            threat_spans.push(Span::styled(s, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
        } else {
            threat_spans.push(Span::styled(s, Style::default().fg(Color::DarkGray)));
        }
    }
    let threat_line = Line::from(threat_spans);

    let text = vec![
        Line::from(""), // Padding top
        secret_line,
        empty_line,
        threat_line,
    ];
    
    let threat_paragraph = Paragraph::new(text)
        .block(threat_block);
        // .style(Style::default().fg(Color::Red)); // REMOVED: On ne met plus tout en rouge
    f.render_widget(threat_paragraph, left_chunks[1]);


    // --- ZONE CENTRALE : CARNET DE DÉTECTIVE ---
    let notebook_block = Block::default()
        .title(" 📓 Carnet de Détective ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default().fg(Color::Yellow));
    
    let inner_notebook = notebook_block.inner(center_area);
    f.render_widget(notebook_block, center_area);

    let mut notes_items = Vec::new();
    if let Some(game) = &app.game {
         for (opp_id, progress) in &progress_map {
             let opp = game.players.iter().find(|op| op.id == *opp_id);
             let opp_name = opp.map(|op| op.name.as_str()).unwrap_or("Adversaire");
             let is_eliminated = opp.map(|op| op.is_eliminated).unwrap_or(false);
             
             // Visualisation des slots : "_ _ A _ _"
             let visual_slots: String = progress.chars().map(|c| format!("{} ", c)).collect();
             
             let status_text = if is_eliminated {
                 format!("{} (ÉLIMINÉ)", visual_slots.trim())
             } else {
                 visual_slots.trim().to_string()
             };

             // Format une ligne : "Bob : _ _ A _ _"
             notes_items.push(ListItem::new(format!(
                 "  {:<10} : {}", 
                 opp_name, status_text
             )));
             notes_items.push(ListItem::new("")); // Espacement
        }
    }
    
    if notes_items.is_empty() {
         notes_items.push(ListItem::new("\n  Pas encore d'indices..."));
    }
    
    // Ajout d'un padding en haut
    let mut padded_items = vec![ListItem::new("")];
    padded_items.extend(notes_items);

    let notebook_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_notebook);

    let notes_list = List::new(padded_items).style(Style::default().fg(Color::White));
    f.render_widget(notes_list, notebook_chunks[0]);

    // Zone d'interaction (Popups)
    let interaction_area = notebook_chunks[1];

    // Affichage des popups dans la zone d'interaction
    match app.current_state {
        AppState::SelectTarget => draw_select_target_ui(f, app, interaction_area),
        AppState::InputLetter => draw_input_letter_ui(f, app, interaction_area),
        AppState::InputGuess => draw_input_guess_ui(f, app, interaction_area),
        AppState::TurnResult => draw_turn_result_ui(f, app, interaction_area),
        _ => {}
    }


    // --- ZONE DROITE : ACTIONS & LOG ---
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // System (Pause/Quit) - Augmenté pour afficher les boutons
            Constraint::Length(16), // Menu Actions (4 boutons * 3 lignes + bordures)
            Constraint::Min(5),     // Game Log (Reste de l'espace)
        ])
        .split(right_area);

    // 0. System Card (Horizontal)
    let system_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.current_focus == Focus::System { Color::Yellow } else { Color::DarkGray }));
    
    let inner_system = system_block.inner(right_chunks[0]);
    f.render_widget(system_block, right_chunks[0]);

    // On utilise un Layout horizontal pour les boutons avec un peu d'espace
    let system_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Length(1), // Espace entre les boutons
            Constraint::Percentage(50)
        ])
        .split(inner_system);

    // Bouton Pause (system_layout[0])
    let pause_selected = app.current_focus == Focus::System && app.system_list_state.selected() == Some(0);
    let pause_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if pause_selected { Color::Yellow } else { Color::DarkGray }));
    
    let pause_text = Paragraph::new("Pause")
        .block(pause_block)
        .alignment(ratatui::layout::Alignment::Center)
        .style(if pause_selected { Style::default().add_modifier(Modifier::BOLD) } else { Style::default() });
    
    f.render_widget(pause_text, system_layout[0]);

    // Bouton Quitter (system_layout[2] car [1] est l'espace si on en met un, sinon [1])
    // Ici j'ai mis 3 constraints, donc c'est [0] et [2] pour les boutons.
    // Mais attention, system_layout a 3 chunks.
    
    let quit_selected = app.current_focus == Focus::System && app.system_list_state.selected() == Some(1);
    let quit_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if quit_selected { Color::Yellow } else { Color::DarkGray }));

    let quit_text = Paragraph::new("Quitter")
        .block(quit_block)
        .alignment(ratatui::layout::Alignment::Center)
        .style(if quit_selected { Style::default().add_modifier(Modifier::BOLD) } else { Style::default() });

    f.render_widget(quit_text, system_layout[2]);

    // 1. Actions
    let actions_block = Block::default()
        .title(" ⚡ Actions ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.current_focus == Focus::Actions { Color::Cyan } else { Color::White }));
    
    let inner_actions = actions_block.inner(right_chunks[1]);
    f.render_widget(actions_block, right_chunks[1]);

    // Layout vertical pour les 3 boutons d'action
    let actions_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Bouton 1
            Constraint::Length(3), // Bouton 2
            Constraint::Length(3), // Bouton 3
            Constraint::Length(3), // Bouton 4 (Passer son tour)
        ])
        .split(inner_actions);

    let action_items = &app.playing_list_items;
    
    for (i, item) in action_items.iter().enumerate() {
        if i < 4 { // On s'assure de ne pas dépasser (on a 4 slots)
            let is_selected = app.playing_list_state.selected() == Some(i);
            // Si le focus est sur Actions, on highlight la sélection. Sinon on grise tout.
            let is_focused = app.current_focus == Focus::Actions;
            
            let border_color = if is_focused && is_selected { Color::Cyan } else { Color::DarkGray };
            let text_style = if is_focused && is_selected { 
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) 
            } else { 
                Style::default().fg(Color::Gray) 
            };

            let button_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(border_color));

            let paragraph = Paragraph::new(item.as_str())
                .block(button_block)
                .style(text_style)
                .alignment(ratatui::layout::Alignment::Center);
            
            f.render_widget(paragraph, actions_layout[i]);
        }
    }

    // 2. Game Log
    let log_block = Block::default()
        .title(" 📜 Journal de Bord ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Gray));
    
    let inner_log = log_block.inner(right_chunks[2]);
    f.render_widget(log_block, right_chunks[2]);

    // On ajoute un peu de padding pour que le texte ne colle pas aux bords
    let padded_log_area = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(1)
        .vertical_margin(1)
        .split(inner_log)[0];

    let log_items: Vec<ListItem> = app.game_log.iter().rev().map(|s| ListItem::new(format!("> {}", s))).collect();
    let log_list = List::new(log_items).style(Style::default().fg(Color::DarkGray));
    f.render_widget(log_list, padded_log_area);
}

fn draw_select_target_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🎯 Choisir une cible (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let targets = &app.target_list_items;
    let count = targets.len();
    if count == 0 { return; }

    // Grille dynamique : 2 colonnes
    let cols = 2;
    let rows = (count as f32 / cols as f32).ceil() as usize;

    let row_constraints: Vec<Constraint> = (0..rows).map(|_| Constraint::Length(3)).collect();
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .margin(1)
        .split(inner_area);

    for (i, (id, name)) in targets.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;

        if row < vertical_layout.len() {
            let row_area = vertical_layout[row];
            let col_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(row_area);
            
            if col < col_layout.len() {
                let is_selected = app.target_list_state.selected() == Some(i);
                let border_color = if is_selected { Color::Cyan } else { Color::DarkGray };
                let text_style = if is_selected { 
                    Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) 
                } else { 
                    Style::default().fg(Color::Gray) 
                };

                let button_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .style(Style::default().fg(border_color));

                let paragraph = Paragraph::new(format!("{} - {}", id, name))
                    .block(button_block)
                    .style(text_style)
                    .alignment(ratatui::layout::Alignment::Center);

                f.render_widget(paragraph, col_layout[col]);
            }
        }
    }
}

fn draw_input_letter_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🔎 Enquête (Lettre) (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Help text
            Constraint::Length(3), // Input box
            Constraint::Length(1), // Error message
        ])
        .margin(1)
        .split(inner_area);

    let help_text = Paragraph::new("Entrez une lettre :").style(Style::default().fg(Color::Gray));
    f.render_widget(help_text, layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(format!("  {}", app.input)) // Padding manuel simple
        .block(input_block)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left); // Alignement gauche
    f.render_widget(paragraph, layout[1]);

    if let Some(err) = &app.input_error {
        let error_text = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_text, layout[2]);
    }
}

fn draw_input_guess_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🎯 Deviner un mot (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Magenta));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Help text
            Constraint::Length(3), // Input box
            Constraint::Length(1), // Error message
        ])
        .margin(1)
        .split(inner_area);

    let help_text = Paragraph::new("Entrez le mot secret :").style(Style::default().fg(Color::Gray));
    f.render_widget(help_text, layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(format!("  {}", app.input)) // Padding manuel simple
        .block(input_block)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left); // Alignement gauche
    f.render_widget(paragraph, layout[1]);

    if let Some(err) = &app.input_error {
        let error_text = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_text, layout[2]);
    }
}

fn draw_turn_result_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let color = if app.last_action_success { Color::Green } else { Color::Red };
    let title = if app.last_action_success { " Succès ! " } else { " Échec " };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default().fg(color));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let text = format!("\n{}\n\n(Appuyez sur Entrée)", app.last_action_result);
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, inner_area);
}fn draw_theme_selection_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let config_area = create_centered_rect(area, 60, 80);
    
    let title = format!(" Configuration Joueur {} - Thème ", app.setup_player_index + 1);

    let config_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 1));

    f.render_widget(config_block.clone(), config_area);

    let list_area = config_block.inner(config_area);

    // Layout vertical pour les boutons
    let items_count = app.setup_list_items.len();
    let constraints: Vec<Constraint> = (0..items_count).map(|_| Constraint::Length(3)).collect();
    
    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(list_area);

    for (i, item) in app.setup_list_items.iter().enumerate() {
        let is_selected = app.setup_list_state.selected() == Some(i);
        
        let border_color = if is_selected { Color::Cyan } else { Color::DarkGray };
        let text_style = if is_selected { 
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) 
        } else { 
            Style::default().fg(Color::Gray) 
        };

        let button_block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().fg(border_color));

        let paragraph = Paragraph::new(item.as_str())
            .block(button_block)
            .style(text_style)
            .alignment(ratatui::layout::Alignment::Center);
        
        f.render_widget(paragraph, list_layout[i]);
    }
}

fn draw_pause_popup(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" PAUSE ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    let area = create_centered_rect(area, 30, 30);
    
    f.render_widget(Clear, area); // Efface le fond pour que la popup soit lisible
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Marge haut
            Constraint::Min(0),    // Menu
            Constraint::Length(1), // Marge bas
        ])
        .margin(1)
        .split(area);

    let menu_items = vec![
        "Reprendre la partie",
        "Aide / Commandes",
        "Quitter vers le menu principal",
        "Quitter le jeu",
    ];

    // Layout vertical pour les boutons
    let items_count = menu_items.len();
    let constraints: Vec<Constraint> = (0..items_count).map(|_| Constraint::Length(3)).collect();
    
    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(layout[1]);

    for (i, item) in menu_items.iter().enumerate() {
        let is_selected = app.pause_menu_index == i;
        
        let border_color = if is_selected { Color::Cyan } else { Color::DarkGray };
        let text_style = if is_selected { 
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) 
        } else { 
            Style::default().fg(Color::Gray) 
        };

        let button_block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().fg(border_color));

        let paragraph = Paragraph::new(*item)
            .block(button_block)
            .style(text_style)
            .alignment(ratatui::layout::Alignment::Center);
        
        if i < list_layout.len() {
            f.render_widget(paragraph, list_layout[i]);
        }
    }
}
