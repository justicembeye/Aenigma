use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Clear};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use crate::tui::app::{App, AppState, Focus, Action};

/// Helper pour faire défiler du texte horizontalement (Marquee)
fn scroll_text(text: &str, width: usize, frame: usize) -> String {
    let text_len = text.chars().count();
    if text_len <= width {
        return text.to_string();
    }

    let padding = "   "; // Espace entre la fin et le début
    let full_text = format!("{}{}", text, padding);
    let full_len = full_text.chars().count();
    
    // Vitesse : 1 caractère tous les 3 frames (environ 300ms)
    let offset = (frame / 3) % full_len;
    
    let cycled: String = full_text.chars().cycle().skip(offset).take(width).collect();
    cycled
}

pub fn ui(f: &mut Frame, app: &mut App) {

    // ... layout principal ...
    // Calcul du temps écoulé
    let title = "🌏 Aenigma".to_string();

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
        AppState::SetupSelectSubTheme => {
            draw_sub_theme_selection_ui(f, app, inner_area);
        }
        AppState::SetupThemeSelectionMethod => {
            draw_theme_selection_method_ui(f, app, inner_area);
        }
        AppState::SetupEnterSecretWord => {
            let title = format!("👤 {} ", app.current_setup_name);
            draw_input_screen(f, app, inner_area, title.as_str(), "🧩 Mot Secret: ");
        }
        AppState::Playing | AppState::SelectTarget | AppState::InputLetter | AppState::InputGuess | AppState::TurnResult | AppState::Paused | AppState::Rules | AppState::ConfirmQuit | AppState::SelectPower | AppState::SelectPowerTarget | AppState::SelectPowerLetter | AppState::RespondToInquiry | AppState::InputPosition => {
            draw_playing_ui(f, app, inner_area);
            if app.current_state == AppState::Rules {
                draw_rules_popup(f, app, inner_area);
            }

        }
        AppState::MultiplayerMenu => {
            draw_welcome_ui(f, app, inner_area); // On réutilise le style du menu principal pour l'instant
        }
        AppState::MultiplayerNameInput => {
            draw_name_input_ui(f, app, inner_area);
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

    // Notification Overlay
    if let Some((msg, time)) = &app.notification {
        if time.elapsed() < std::time::Duration::from_secs(3) {
            let area = create_centered_rect(f.area(), 40, 10);
            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Yellow).fg(Color::Black))
                .border_type(ratatui::widgets::BorderType::Double);
            
            let paragraph = Paragraph::new(msg.as_str())
                .block(block)
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().add_modifier(Modifier::BOLD));
            
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
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

    let title_lines = vec![
        Line::from("    _    _____ _   _ ___ ____ __  __    _    "),
        Line::from("   / \\  | ____| \\ | |_ _/ ___|  \\/  |  / \\   "),
        Line::from("  / _ \\ |  _| |  \\| || | |  _| |\\/| | / _ \\  "),
        Line::from(" / ___ \\| |___| |\\  || | |_| | |  | |/ ___ \\ "),
        Line::from("/_/   \\_\\_____|_| \\_|___\\____|_|  |_/_/   \\_\\"),
        Line::from(""),
        Line::from(Span::styled("             Le Jeu de Déduction et de Stratégie             ", Style::default().add_modifier(Modifier::BOLD))),
    ];

    let title_paragraph = Paragraph::new(title_lines)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Cyan));
    
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

fn draw_lobby_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Lobby - En attente ")
        .border_type(ratatui::widgets::BorderType::Rounded);

    let center_area = create_centered_rect(area, 60, 50);
    f.render_widget(Clear, center_area);
    f.render_widget(block, center_area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Titre/Status
            Constraint::Min(5),    // Liste joueurs
            Constraint::Length(2), // Footer
        ])
        .margin(2)
        .split(center_area);

    let status_text = if app.is_host {
        "Vous êtes l'Hôte. Configurez la partie..."
    } else {
        "En attente de l'hôte..."
    };
    
    f.render_widget(Paragraph::new(status_text).style(Style::default().fg(Color::Yellow)), layout[0]);

    let players: Vec<ListItem> = app.connected_players.iter().map(|(id, name)| {
        let is_me = Some(*id) == app.my_player_id;
        let prefix = if is_me { "➤ " } else { "  " };
        let style = if is_me { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default() };
        ListItem::new(format!("{}{} (ID: {})", prefix, name, id)).style(style)
    }).collect();

    let players_list = List::new(players)
        .block(Block::default().borders(Borders::ALL).title(" Joueurs Connectés "));
    
    f.render_widget(players_list, layout[1]);

    f.render_widget(Paragraph::new("Esc: Quitter").style(Style::default().fg(Color::DarkGray)), layout[2]);
}

fn draw_rules_popup(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .title(" Règles & Commandes ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Green));

    let area = create_centered_rect(area, 85, 85);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let text = vec![
        Line::from(Span::styled("🕵️  BUT DU JEU", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("Découvrez le mot secret de vos adversaires avant qu'ils ne trouvent le vôtre !"),
        Line::from(""),
        Line::from(Span::styled("⚙️  NIVEAUX DE DIFFICULTÉ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(vec![Span::styled("• FACILE : ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Longueur connue (_ _ _). Indices placés auto. IA bête.")]),
        Line::from(vec![Span::styled("• NORMAL : ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Longueur INCONNUE (░░░). À vous de déduire les positions. IA humaine.")]),
        Line::from(vec![Span::styled("• DIFFICILE : ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Aucune aide. IA Optimale (Ne devine que si 100% sûre).")]),
        Line::from(""),
        Line::from(Span::styled("🎮  DÉROULEMENT D'UN TOUR", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
        Line::from("1. PROPOSER UNE LETTRE : Demandez si l'adversaire a un 'A'."),
        Line::from("   - S'il l'a, il DOIT donner la position (ex: 1ère et 3ème)."),
        Line::from("   - En Normal/Difficile, notez-le vous-même ! Le jeu ne remplit pas la grille."),
        Line::from("2. DEVINER UN MOT : Tentez de trouver le mot complet."),
        Line::from("   - Attention : Si vous ratez, vous êtes ÉLIMINÉ !"),
        Line::from(""),
        Line::from(Span::styled("⚠️  NIVEAU DE MENACE (Gauche)", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from("Affiche VOTRE mot secret. Les lettres que l'ennemi a trouvées sont en ROUGE."),
        Line::from("Plus c'est rouge, plus vous êtes en danger !"),
        Line::from(""),
        Line::from(Span::styled("⌨️  COMMANDES", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from("• Flèches   : Naviguer"),
        Line::from("• Entrée    : Valider"),
        Line::from("• Échap     : Pause / Retour"),
        Line::from("• Saisie    : Pour les positions, séparez par virgule (ex: '1, 3')"),
        Line::from(""),
        Line::from(Span::styled("Appuyez sur Echap pour revenir", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))),
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
            } else if i == 0 { // Le premier est le vainqueur si le jeu est fini
                Span::styled("VAINQUEUR", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("ACTIF", Style::default().fg(Color::White))
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

        // Afficher le mot secret du vainqueur
        if let Some(winner) = sorted_players.first() {
            let secret_text = format!("🏆 Mot secret du vainqueur : {}", winner.secret_word.content.to_uppercase());
            let secret_paragraph = Paragraph::new(secret_text)
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
            f.render_widget(secret_paragraph, chunks[6]);
        }
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


    if app.show_suggestions {
        draw_suggestion_popup(f, app, inner_input_area);
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
            Constraint::Length(15),  // Carte d'Agent (Ajusté)
            Constraint::Min(10),    // Menace & Secret
        ])
        .split(left_area);

    // 1. Carte d'Agent
    // 1. Carte d'Agent
    let (player_name, player_score, energy, my_secret_word, _progress_map, player_id) = if let Some(game) = &app.game {
        // On affiche TOUJOURS le joueur local (Moi)
        let my_id = app.my_player_id.unwrap_or(0); // 0 par défaut en solo
        if let Some(p) = game.players.iter().find(|p| p.id == my_id) {
             (p.name.clone(), p.score, p.energy, p.secret_word.content.clone(), p.progress_on_opponents.clone(), p.id)
        } else {
             ("Erreur".to_string(), 0, 0, "???".to_string(), std::collections::HashMap::new(), 0)
        }
    } else {
        ("Inconnu".to_string(), 0, 0, "???".to_string(), std::collections::HashMap::new(), 0)
    };

    let is_my_turn = if let Some(game) = &app.game {
        game.current_player().id == player_id
    } else {
        false
    };
    let (border_color, title_text) = if is_my_turn {
        (Color::Green, " 🕵️  Carte d'Agent ")
    } else {
        (Color::Cyan, " 🕵️  Carte d'Agent ")
    };

    let agent_block = Block::default()
        .title(title_text)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .style(Style::default().fg(border_color));
    
    // Jauge d'Énergie visuelle (8 blocs pour gagner de l'espace)
    let buzz_bar: String = (0..8).map(|i| if i * 12 < energy { "█" } else { "░" }).collect();
    
    // Construction du contenu de la carte d'agent avec styling
    let mut agent_lines = Vec::new();

    agent_lines.push(Line::from("")); // Padding top
    
    // Ligne Nom
    let mut name_spans = vec![
        Span::styled(" Nom     : ", Style::default().fg(Color::White)),
    ];
    
    if is_my_turn {
        name_spans.push(Span::styled(player_name.clone(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
        name_spans.push(Span::styled(" 🟢", Style::default().fg(Color::Green))); // Indicateur de tour
    } else {
        name_spans.push(Span::styled(player_name.clone(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    }
    agent_lines.push(Line::from(name_spans));
    
    agent_lines.push(Line::from("")); // Espace
    
    // Ligne Score
    agent_lines.push(Line::from(vec![
        Span::styled(" Score   : ", Style::default().fg(Color::White)),
        Span::styled(format!("{} pts", player_score), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]));
    
    agent_lines.push(Line::from("")); // Espace
    
    // Ligne Énergie
    agent_lines.push(Line::from(vec![
        Span::styled(" Énergie : ", Style::default().fg(Color::White)),
        Span::styled(format!("[{}] {}%", buzz_bar, energy), Style::default().fg(Color::Yellow)),
    ]));

    agent_lines.push(Line::from("")); // Espace

    // Ligne Horloge (Déplacée ici)
    if let Some(start_time) = app.start_time {
        let elapsed = start_time.elapsed();
        let secs = elapsed.as_secs();
        let mins = secs / 60;
        let secs = secs % 60;
        agent_lines.push(Line::from(vec![
            Span::styled(" Temps   : ", Style::default().fg(Color::White)),
            Span::styled(format!("{:02}:{:02}", mins, secs), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]));
    }

    agent_lines.push(Line::from("")); // Espace entre Temps et Niveau

    // Ligne Difficulté
    if let Some(game) = &app.game {
        let (difficulty_text, difficulty_color) = match game.difficulty {
            crate::game::Difficulty::Easy => ("Facile", Color::Green),
            crate::game::Difficulty::Normal => ("Normal", Color::Yellow),
            crate::game::Difficulty::Hard => ("Difficile", Color::Red),
            crate::game::Difficulty::Expert => ("Expert", Color::Magenta),
        };
        agent_lines.push(Line::from(vec![
            Span::styled(" Niveau  : ", Style::default().fg(Color::White)),
            Span::styled(difficulty_text, Style::default().fg(difficulty_color).add_modifier(Modifier::BOLD)),
        ]));
    }
    
    agent_lines.push(Line::from("")); // Espace bas

    // On rend le block séparément pour que le fond du paragraphe ne dépasse pas sur la bordure
    f.render_widget(agent_block.clone(), left_chunks[0]);
    
    let inner_agent_area = agent_block.inner(left_chunks[0]);
    
    let bg_color = Color::Rgb(20, 30, 40);
    let inner_bg_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(bg_color).bg(bg_color)); // Couleur de fond pour le block et sa bordure

    let agent_paragraph = Paragraph::new(agent_lines)
        .block(inner_bg_block)
        .style(Style::default().fg(Color::White).bg(bg_color));
        
    f.render_widget(agent_paragraph, inner_agent_area);

    // 2. Menace & Secret
    let threat_block = Block::default()
        .title(" ⚠️  Niveau de Menace ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White)); // Bordure neutre
    
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
    let mut threat_lines = Vec::new();
    threat_lines.push(Line::from("")); // Padding top

    // Ligne 1 : "VOTRE SECRET : M O T"
    threat_lines.push(Line::from(vec![
        Span::styled("  VOTRE SECRET : ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(spaced_secret, Style::default().fg(if is_ai { Color::DarkGray } else { Color::Cyan })), 
    ]));

    // Ligne 2 : Vide
    threat_lines.push(Line::from(""));

    // Ligne 3 : "CE QU'ILS SAVENT DE VOUS :"
    threat_lines.push(Line::from(Span::styled("  CE QU'ILS SAVENT DE VOUS : ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))));
    threat_lines.push(Line::from("")); // Espace de respiration

    // Itération sur les adversaires pour afficher leur progression
    if let Some(game) = &app.game {
        for opponent in &game.players {
            if opponent.id != player_id && !opponent.is_eliminated {
                // On récupère ce que l'adversaire sait de MOI (player_id)
                let found_letters = opponent.progress_on_opponents.get(&player_id)
                    .map(|p| p.found_letters.clone())
                    .unwrap_or_default();
                
                let mut spans = vec![
                    Span::styled(format!("  • {} : ", opponent.name), Style::default().fg(Color::Gray)),
                ];

                // On récupère aussi le masque pour les cas où la lettre est révélée par Voyance/Révélation mais pas "trouvée" par enquête
                let (mask_chars, length_known) = opponent.progress_on_opponents.get(&player_id)
                    .map(|p| (p.mask.chars().collect::<Vec<char>>(), p.length_known))
                    .unwrap_or((Vec::new(), false));

                // On affiche MON mot secret, mais on colorie en ROUGE les lettres que l'adversaire a trouvées
                for (i, c) in my_secret_word.chars().enumerate() {
                    let upper_c = c.to_ascii_uppercase();
                    let s = format!("{} ", upper_c);
                    
                    // 1. Est-ce dans la liste des lettres trouvées ?
                    let is_in_found = found_letters.iter().any(|fl| fl.to_ascii_uppercase() == upper_c);
                    
                    // 2. Est-ce visible dans le masque ? (Si longueur connue et index valide)
                    let is_in_mask = length_known && i < mask_chars.len() && mask_chars[i].to_ascii_uppercase() == upper_c;

                    if is_in_found || is_in_mask {
                        // DANGER : L'adversaire a cette lettre !
                        spans.push(Span::styled(s, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
                    } else {
                        // SÛR : L'adversaire n'a pas encore cette lettre
                        spans.push(Span::styled(s, Style::default().fg(Color::DarkGray)));
                    }
                }
                threat_lines.push(Line::from(spans));
            }
        }
    }
    
    let threat_paragraph = Paragraph::new(threat_lines)
        .block(threat_block);
    f.render_widget(threat_paragraph, left_chunks[1]);


    // --- ZONE CENTRALE : CARNET DE DÉTECTIVE ---
    let notebook_block = Block::default()
        .title(" 📓 Carnet de Détective ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default().fg(Color::Yellow));
    
    let inner_notebook = notebook_block.inner(center_area);
    f.render_widget(notebook_block, center_area);

    // 1. Define Layout (Notebook vs Interaction)
    let notebook_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_notebook);
    
    let notebook_area = notebook_chunks[0];
    let interaction_area = notebook_chunks[1];

    let mut opponents_ids: Vec<u32> = Vec::new();
    if let Some(game) = &app.game {
        let my_id = app.my_player_id.unwrap_or(0);
        if let Some(me) = game.players.iter().find(|p| p.id == my_id) {
            for (opp_id, _) in &me.progress_on_opponents {
                opponents_ids.push(*opp_id);
            }
        }
        opponents_ids.sort(); // Garder l'ordre stable
    }



    // === MODE CAROUSEL (TOUS MODES) : VUE DÉTAILLÉE (CARTE) ===
    let mut notes_items = Vec::new();
    if !opponents_ids.is_empty() {
         let selected_idx = app.selected_opponent_index % opponents_ids.len();
         let opp_id = opponents_ids[selected_idx];
         
         if let Some(game) = &app.game {
             let my_id = app.my_player_id.unwrap_or(0);
             let me = game.players.iter().find(|p| p.id == my_id).unwrap();
             let progress_struct = me.progress_on_opponents.get(&opp_id).unwrap();
             let opp = game.players.iter().find(|op| op.id == opp_id);
             let opp_name = opp.map(|op| op.name.as_str()).unwrap_or("Adversaire");
             let is_eliminated = opp.map(|op| op.is_eliminated).unwrap_or(false);
             
             let card_color = match opp_id % 4 {
                 0 => Color::Cyan,
                 1 => Color::Magenta,
                 2 => Color::Green,
                 _ => Color::Yellow,
             };

             // En-tête avec Navigation
             let mut name_line = vec![
                 Span::styled(" ╔═ ", Style::default().fg(card_color)),
             ];
             
             // Flèche Gauche
             if opponents_ids.len() > 1 {
                 if app.current_focus == Focus::Notebook {
                     name_line.push(Span::styled("◄ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
                 } else {
                     name_line.push(Span::styled("◄ ", Style::default().fg(Color::DarkGray)));
                 }
             }

             // Nom avec défilement
             let scrolled_name = scroll_text(&opp_name.to_uppercase(), 15, app.time_frame);
             name_line.push(Span::styled(format!("🕵️  {}", scrolled_name), Style::default().fg(card_color).add_modifier(Modifier::BOLD)));

             // Flèche Droite
             if opponents_ids.len() > 1 {
                 if app.current_focus == Focus::Notebook {
                     name_line.push(Span::styled(" ► ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
                 } else {
                     name_line.push(Span::styled(" ► ", Style::default().fg(Color::DarkGray)));
                 }
                 // Compteur
                 name_line.push(Span::styled(format!("({}/{})", selected_idx + 1, opponents_ids.len()), Style::default().fg(Color::DarkGray)));
             }

             if let Some(timer) = app.ai_thinking_timer {
                 if opp.map(|p| p.control_type == crate::game::model::ControlType::AI).unwrap_or(false) {
                      let elapsed = timer.elapsed().as_millis();
                      let icon = match (elapsed / 500) % 4 { 0 => " ⏳", 1 => " ⌛", 2 => " ⏳", 3 => " ⌛", _ => " ⏳" };
                      name_line.push(Span::styled(icon, Style::default().fg(Color::Cyan)));
                 }
             }
             name_line.push(Span::styled(" ═══════════════════", Style::default().fg(card_color)));
             notes_items.push(ListItem::new(Line::from(name_line)));
             notes_items.push(ListItem::new(Line::from(vec![Span::styled(" ║", Style::default().fg(card_color))])));

             // Thème avec défilement
             if let Some(player) = opp {
                 let scrolled_theme = scroll_text(&player.secret_word.theme, 20, app.time_frame);
                 notes_items.push(ListItem::new(Line::from(vec![
                     Span::styled(" ║ ", Style::default().fg(card_color)),
                     Span::styled("📚 Thème: ", Style::default().fg(Color::White)),
                     Span::styled(scrolled_theme, Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC)),
                 ])));
                 notes_items.push(ListItem::new(Line::from(vec![Span::styled(" ║", Style::default().fg(card_color))])));
             }

             // Mot
             let mut mask_line = vec![
                 Span::styled(" ║ ", Style::default().fg(card_color)),
                 Span::styled("🔍 Mot: ", Style::default().fg(Color::White)),
             ];
             if is_eliminated {
                 mask_line.push(Span::styled(format!("{} (ÉLIMINÉ)", progress_struct.mask), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
             } else {
                 if !progress_struct.length_known {
                     mask_line.push(Span::styled("░░░░░░░░░", Style::default().fg(Color::DarkGray)));
                 } else {
                     for c in progress_struct.mask.chars() {
                         let s = format!("{} ", c);
                         if c.is_alphabetic() && c != '_' {
                             mask_line.push(Span::styled(s, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
                         } else {
                             mask_line.push(Span::styled(s, Style::default().fg(Color::DarkGray)));
                         }
                     }
                 }
             }
             notes_items.push(ListItem::new(Line::from(mask_line)));
             notes_items.push(ListItem::new(Line::from(vec![Span::styled(" ║", Style::default().fg(card_color))])));

             // Indices
             if !progress_struct.found_letters.is_empty() {
                 let mut indices_line = vec![
                     Span::styled(" ║ ", Style::default().fg(card_color)),
                     Span::styled("✅ Indices: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                 ];
                 for (idx, c) in progress_struct.found_letters.iter().enumerate() {
                     if idx > 0 { indices_line.push(Span::raw("  ")); }
                     indices_line.push(Span::styled(format!("[{}]", c), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
                     if let Some(positions) = progress_struct.found_letter_positions.get(c) {
                         if !positions.is_empty() {
                             let pos_str = positions.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
                             indices_line.push(Span::styled(format!(" → {}", pos_str), Style::default().fg(Color::Cyan)));
                         }
                     }
                 }
                 notes_items.push(ListItem::new(Line::from(indices_line)));
                 notes_items.push(ListItem::new(Line::from(vec![Span::styled(" ║", Style::default().fg(card_color))])));
             } else {
                 notes_items.push(ListItem::new(Line::from(vec![
                     Span::styled(" ║ ", Style::default().fg(card_color)),
                     Span::styled("(Aucun indice connu)", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
                 ])));
                 notes_items.push(ListItem::new(Line::from(vec![Span::styled(" ║", Style::default().fg(card_color))])));
             }

             // Absents
             if !progress_struct.missed_letters.is_empty() {
                 let mut missed_line = vec![
                     Span::styled(" ║ ", Style::default().fg(card_color)),
                     Span::styled("❌ Absents: ", Style::default().fg(Color::Red)),
                 ];
                 for c in &progress_struct.missed_letters {
                     missed_line.push(Span::styled(format!("{} ", c), Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT)));
                 }
                 notes_items.push(ListItem::new(Line::from(missed_line)));
                 notes_items.push(ListItem::new(Line::from(vec![Span::styled(" ║", Style::default().fg(card_color))])));
             }

             notes_items.push(ListItem::new(Line::from(vec![
                 Span::styled(" ╚════════════════════════════════════════", Style::default().fg(card_color)),
             ])));
             notes_items.push(ListItem::new(Line::from("")));
         }
    }
        
        if notes_items.is_empty() {
             notes_items.push(ListItem::new("\n  Pas encore d'indices..."));
        }
        
        let mut padded_items = vec![ListItem::new("")];
        padded_items.extend(notes_items);
        
        let notes_list = List::new(padded_items).style(Style::default().fg(Color::White));
        f.render_widget(notes_list, notebook_area);


    // Affichage des popups dans la zone d'interaction
    match app.current_state {
        AppState::SelectTarget => draw_select_target_ui(f, app, interaction_area),
        AppState::InputLetter => draw_input_letter_ui(f, app, interaction_area),
        AppState::InputGuess => draw_input_guess_ui(f, app, interaction_area),
        AppState::TurnResult => draw_turn_result_ui(f, app, interaction_area),
        AppState::Paused => draw_pause_popup(f, app, interaction_area),
        AppState::ConfirmQuit => draw_confirm_quit_popup(f, app, interaction_area),
        AppState::SelectPower => draw_power_selection_popup(f, app, interaction_area),
        AppState::SelectPowerTarget => draw_power_target_popup(f, app, interaction_area),
        AppState::SelectPowerLetter => draw_power_letter_popup(f, app, interaction_area),
        AppState::RespondToInquiry => draw_inquiry_popup(f, app, interaction_area),
        AppState::InputPosition => draw_input_position_ui(f, app, interaction_area),
        AppState::Playing => {
            // Afficher la grille de pouvoirs si énergie = 100% ET difficulté != Facile
            let should_show_powers = if let Some(game) = &app.game {
                let my_id = app.my_player_id.unwrap_or(1);
                if let Some(player) = game.players.iter().find(|p| p.id == my_id) {
                    player.energy >= 100 && game.difficulty != crate::game::Difficulty::Easy
                } else {
                    false
                }
            } else {
                false
            };

            if should_show_powers {
                draw_power_grid(f, app, interaction_area);
            }
        }
        _ => {}
    }


    // --- ZONE DROITE : ACTIONS & LOG ---
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),      // System (Pause/Quit) - garde fixe
            Constraint::Percentage(50), // Menu Actions (50% pour 4 boutons de 4 lignes)
            Constraint::Percentage(50), // Game Log (50% de l'espace)
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

    // Layout vertical dynamique pour les boutons d'action
    let action_items = &app.playing_list_items;
    
    if !action_items.is_empty() {
        // Copier exactement l'approche du menu Pause
        let items_count = action_items.len();
        let constraints: Vec<Constraint> = (0..items_count).map(|_| Constraint::Length(3)).collect();
        
        let actions_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_actions); // PAS de vertical_margin !
        
        
        for (i, item) in action_items.iter().enumerate() {
            if i < actions_layout.len() {
                let is_selected = app.playing_list_state.selected() == Some(i);
                let is_focused = app.current_focus == Focus::Actions;
                
                let is_buzz = if let Some(action) = app.available_actions.get(i) {
                    matches!(action, Action::Buzz)
                } else {
                    false
                };
                
                let energy = if let Some(game) = &app.game {
                    game.players.iter()
                        .find(|p| p.id == app.my_player_id.unwrap_or(1))
                        .map(|p| p.energy)
                        .unwrap_or(0)
                } else {
                    0
                };
                
                let is_disabled = is_buzz && energy < 100;
                
                let (border_fg, content_bg, text_fg) = if is_disabled {
                    // Buzz désactivé : grisé
                    (Color::DarkGray, Color::Reset, Color::DarkGray)
                } else if is_focused && is_selected {
                    (Color::Cyan, Color::Cyan, Color::Black)
                } else {
                    (Color::DarkGray, Color::Reset, Color::White)
                };

                let button_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .style(Style::default().fg(border_fg));

                let paragraph = Paragraph::new(item.clone())
                    .block(button_block)
                    .style(Style::default().fg(text_fg).bg(content_bg).add_modifier(if is_focused && is_selected && !is_disabled { Modifier::BOLD } else { Modifier::empty() }))
                    .alignment(ratatui::layout::Alignment::Center);
                
                // Ajouter une marge horizontale pour que les boutons respirent
                let button_area = Layout::default()
                    .constraints([Constraint::Percentage(100)])
                    .horizontal_margin(2)
                    .split(actions_layout[i])[0];
                f.render_widget(paragraph, button_area);
            }
        }
    }

    // 2. Game Log (Prend toute la place restante)
    let is_log_focused = app.current_focus == Focus::GameLog;
    let log_block = Block::default()
        .title(" 📜 Journal de Bord ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if is_log_focused { Color::Cyan } else { Color::Gray }));
    
    let inner_log = log_block.inner(right_chunks[2]);
    f.render_widget(log_block, right_chunks[2]);

    // Grouper les logs par tour
    let mut turns: Vec<Vec<String>> = Vec::new();
    let mut current_turn: Vec<String> = Vec::new();
    
    for log_entry in &app.game_log {
        if log_entry.starts_with("> [Tour") {
            if !current_turn.is_empty() {
                turns.push(current_turn.clone());
                current_turn.clear();
            }
        }
        current_turn.push(log_entry.clone());
    }
    if !current_turn.is_empty() {
        turns.push(current_turn);
    }

    let total_turns = turns.len();
    if total_turns == 0 {
        return; // Pas de logs à afficher
    }

    let max_offset = total_turns.saturating_sub(1);
    let display_turn_index = total_turns.saturating_sub(1).saturating_sub(app.log_scroll_offset.min(max_offset));
    
    // Layout: Header (Tour X) + Content
    let log_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header: Tour X
            Constraint::Min(0),    // Content: événements
        ])
        .margin(1)
        .split(inner_log);

    // === HEADER: TOUR X ===
    // Afficher le tour qu'on regarde actuellement (avec navigation)
    let viewed_turn = display_turn_index + 1;
    let current_game_turn = if let Some(game) = &app.game {
        game.get_current_round()
    } else {
        1
    };
    
    let header_text = if total_turns > 1 {
        // Avec navigation : afficher "◄ TOUR X / Y ►" où X = tour affiché, Y = tour actuel
        format!("◄ TOUR {} / {} ►", viewed_turn, current_game_turn)
    } else {
        format!("TOUR {}", viewed_turn)
    };
    
    let header = Paragraph::new(header_text)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default()
            .fg(if is_log_focused { Color::Cyan } else { Color::Yellow })
            .add_modifier(Modifier::BOLD));
    f.render_widget(header, log_layout[0]);

    // === EVENTS: Liste Timeline ===
    let mut event_items: Vec<ListItem> = Vec::new();
    
    if let Some(turn_logs) = turns.get(display_turn_index) {
        // Spacer du haut
        event_items.push(ListItem::new(Line::from("")));

        for (idx, s) in turn_logs.iter().enumerate() {
            // Ignorer l'en-tête "[Tour X]"
            if idx == 0 && s.starts_with("> [Tour") {
                continue;
            }
            
            let text = s.replace("> ", "");
            
            // Style Timeline
            let timeline_char = " │ ";
            let timeline_style = Style::default().fg(Color::DarkGray);
            
            let mut spans = vec![
                Span::styled(timeline_char, timeline_style),
            ];

            let scrolled_text = scroll_text(&text, 40, app.time_frame);

            if text.contains("trouvé") || text.contains("confirmé") {
                spans.push(Span::styled("✅ ", Style::default().fg(Color::Green)));
                spans.push(Span::styled(scrolled_text, Style::default().fg(Color::White)));
            } else if text.contains("raté") || text.contains("nié") {
                spans.push(Span::styled("❌ ", Style::default().fg(Color::Red)));
                spans.push(Span::styled(scrolled_text, Style::default().fg(Color::Gray)));
            } else if text.contains("interroge") {
                spans.push(Span::styled("🕵️  ", Style::default().fg(Color::Cyan)));
                spans.push(Span::styled(scrolled_text, Style::default().fg(Color::Cyan)));
            } else if text.contains("éliminé") {
                spans.push(Span::styled("💀 ", Style::default().fg(Color::Red)));
                spans.push(Span::styled(scrolled_text, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
            } else if !text.trim().is_empty() {
                spans.push(Span::styled("ℹ️  ", Style::default().fg(Color::Blue)));
                spans.push(Span::styled(scrolled_text, Style::default().fg(Color::Gray)));
            }

            // Ajouter la ligne d'événement
            event_items.push(ListItem::new(Line::from(spans)));
            
            // Ajouter un spacer "timeline" entre les événements (sauf le dernier)
            if idx < turn_logs.len() - 1 {
                 event_items.push(ListItem::new(Line::from(vec![
                     Span::styled(timeline_char, timeline_style),
                 ])));
            }
        }
    }

    // Navigation hint en bas si plusieurs tours
    if total_turns > 1 {
        event_items.push(ListItem::new(Line::from("")));
        event_items.push(ListItem::new(Line::from(vec![
            Span::styled(
                format!("   Tour {}/{} ", viewed_turn, total_turns),
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)
            ),
            Span::styled(
                if is_log_focused { "←→" } else { "Tab" },
                Style::default().fg(if is_log_focused { Color::Cyan } else { Color::DarkGray }).add_modifier(Modifier::ITALIC)
            ),
        ])));
    }

    let events_list = List::new(event_items)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(events_list, log_layout[1]);
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
    
    f.render_widget(Clear, area);
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Help text
            Constraint::Length(1), // Espace
            Constraint::Length(3), // Input box
            Constraint::Length(1), // Error message
        ])
        .margin(1)
        .split(block.inner(area));

    let help_text = Paragraph::new("Entrez une lettre :")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help_text, layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(format!(" {}", app.input)) // Padding manuel simple
        .block(input_block)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left); // Alignement gauche
    f.render_widget(paragraph, layout[2]);
    f.set_cursor_position((layout[2].x + 2 + app.cursor_position as u16, layout[2].y + 1));

    if let Some(err) = &app.input_error {
        let error_text = Paragraph::new(err.as_str())
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_text, layout[3]);
    }
}

fn draw_input_guess_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🎯 Deviner un mot (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Magenta));
    
    f.render_widget(Clear, area);
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Help text
            Constraint::Length(1), // Espace
            Constraint::Length(3), // Input box
            Constraint::Length(1), // Error message
        ])
        .margin(1)
        .split(block.inner(area));

    let help_text = Paragraph::new("Entrez le mot secret :")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help_text, layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(format!(" {}", app.input)) // Padding manuel simple
        .block(input_block)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left); // Alignement gauche
    f.render_widget(paragraph, layout[2]);
    f.set_cursor_position((layout[2].x + 2 + app.cursor_position as u16, layout[2].y + 1));

    if let Some(err) = &app.input_error {
        let error_text = Paragraph::new(err.as_str())
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_text, layout[3]);
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

    let timer_text = if let Some(timer) = app.result_timer {
        let elapsed = timer.elapsed();
        if elapsed < std::time::Duration::from_secs(2) {
            format!("(Fermeture dans {} s...)", 2 - elapsed.as_secs())
        } else {
            "(Fermeture...)".to_string()
        }
    } else if app.ai_thinking_timer.is_some() {
        "(En attente de la réponse...)".to_string()
    } else {
        "(Appuyez sur Entrée)".to_string()
    };

    let text = format!("\n{}\n\n{}", app.last_action_result, timer_text);
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, inner_area);
}



fn draw_pause_popup(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" PAUSE ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    // let area = create_centered_rect(area, 40, 30); // On utilise la zone d'interaction directement
    
    f.render_widget(Clear, area); // Efface le fond pour que la popup soit lisible
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Marge haut
            Constraint::Min(0),    // Menu
            Constraint::Length(1), // Marge bas
        ])
        // .margin(1) // Removed margin to save space
        .split(block.inner(area));

    let menu_items = vec![
        "Reprendre la partie",
        "Aide / Commandes",
        "Menu Principal",
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
        
        let (border_fg, content_bg, text_fg) = if is_selected {
            (Color::Cyan, Color::Cyan, Color::Black)
        } else {
            (Color::DarkGray, Color::Reset, Color::White)
        };

        let button_block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().fg(border_fg));

        let paragraph = Paragraph::new(*item)
            .block(button_block)
            .style(Style::default().fg(text_fg).bg(content_bg).add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() }))
            .alignment(ratatui::layout::Alignment::Center);
        
        if i < list_layout.len() {
            let button_area = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .horizontal_margin(2) // Marge pour effet "pilule"
                .split(list_layout[i])[0];
            f.render_widget(paragraph, button_area);
        }
    }
}

fn draw_name_input_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" Entrez votre nom d'agent ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    
    let area = create_centered_rect(area, 40, 20);
    f.render_widget(Clear, area);
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Marge
            Constraint::Length(1), // Marge
            Constraint::Length(3), // Input
            Constraint::Length(1), // Erreur
            Constraint::Length(1), // Erreur
        ])
        .margin(1)
        .split(area);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(format!(" {}", app.input))
        .block(input_block)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(paragraph, layout[1]);
    f.set_cursor_position((layout[1].x + 2 + app.cursor_position as u16, layout[1].y + 1));

    if let Some(err) = &app.input_error {
        let error_text = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_text, layout[2]);
    }
}

fn draw_theme_selection_method_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let config_area = create_centered_rect(area, 60, 40);

    let config_block = Block::default()
        .borders(Borders::ALL)
        .title(" Choix du Thème ")
        .border_type(ratatui::widgets::BorderType::Rounded)
        .padding(ratatui::widgets::Padding::new(2, 2, 1, 1));

    f.render_widget(config_block.clone(), config_area);

    let list_area = config_block.inner(config_area);

    let items: Vec<ListItem> = app.setup_list_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if Some(i) == app.setup_list_state.selected() {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!(" {} ", item)).style(style)
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, list_area, &mut app.setup_list_state);
    
    // Instructions
    let instructions = Paragraph::new(Line::from(vec![
        Span::styled(" (Entrée) ", Style::default().fg(Color::Cyan)),
        Span::raw("Valider   "),
        Span::styled(" (Esc) ", Style::default().fg(Color::Cyan)),
        Span::raw("Retour"),
    ]))
    .alignment(ratatui::layout::Alignment::Center);
    
    let instruction_area = Rect::new(config_area.x, config_area.y + config_area.height + 1, config_area.width, 1);
    f.render_widget(instructions, instruction_area);
}

fn draw_confirm_quit_popup(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" CONFIRMATION ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Red));

    f.render_widget(Clear, area);
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Margin top
            Constraint::Length(1), // Text
            Constraint::Length(1), // Margin mid
            Constraint::Length(3), // Buttons
        ])
        .split(block.inner(area));

    // Text
    let text = Paragraph::new("Voulez-vous vraiment quitter ?")
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(text, layout[1]);

    // Buttons Layout (Horizontal)
    let button_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10), // Espace
            Constraint::Percentage(35), // OUI
            Constraint::Percentage(10), // Espace
            Constraint::Percentage(35), // NON
            Constraint::Percentage(10), // Espace
        ])
        .split(layout[3]);

    // OUI Button (Index 0)
    let oui_selected = app.pause_menu_index == 0;
    let (oui_border, oui_bg, oui_text_fg) = if oui_selected {
        (Color::Red, Color::Red, Color::Black)
    } else {
        (Color::DarkGray, Color::Reset, Color::Gray)
    };
    let oui_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(oui_border));
    let oui_text = Paragraph::new("OUI")
        .block(oui_block)
        .style(Style::default().fg(oui_text_fg).bg(oui_bg).add_modifier(if oui_selected { Modifier::BOLD } else { Modifier::empty() }))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(oui_text, button_layout[1]); // Index 1 car 0 est l'espace

    // NON Button (Index 1)
    let non_selected = app.pause_menu_index == 1;
    let (non_border, non_bg, non_text_fg) = if non_selected {
        (Color::Green, Color::Green, Color::Black)
    } else {
        (Color::DarkGray, Color::Reset, Color::Gray)
    };
    let non_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(non_border));
    let non_text = Paragraph::new("NON")
        .block(non_block)
        .style(Style::default().fg(non_text_fg).bg(non_bg).add_modifier(if non_selected { Modifier::BOLD } else { Modifier::empty() }))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(non_text, button_layout[3]); // Index 3 car 2 est l'espace
}
fn draw_theme_selection_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🌍 Choisissez un Thème ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Layout avec marge verticale pour éviter de coller au titre
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .vertical_margin(2) // Ajout de marge verticale
        .horizontal_margin(2)
        .split(inner_area)[0];

    let items_count = app.setup_list_items.len();
    let constraints: Vec<Constraint> = (0..items_count).map(|_| Constraint::Length(3)).collect();

    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(content_layout);

    for (i, item) in app.setup_list_items.iter().enumerate() {
        if i < list_layout.len() {
            let is_selected = app.setup_list_state.selected() == Some(i);
            
            let (border_fg, content_bg, text_fg) = if is_selected {
                (Color::Cyan, Color::Cyan, Color::Black)
            } else {
                (Color::DarkGray, Color::Reset, Color::Gray)
            };

            let button_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(border_fg));

            let paragraph = Paragraph::new(item.as_str())
                .block(button_block)
                .style(Style::default().fg(text_fg).bg(content_bg).add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() }))
                .alignment(ratatui::layout::Alignment::Center);
            
            let button_area = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .horizontal_margin(4) // Marge plus large pour centrer visuellement
                .split(list_layout[i])[0];
            
            f.render_widget(paragraph, button_area);
        }
    }
}

fn draw_sub_theme_selection_ui(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 📂 Précisez le Sous-Thème ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Layout avec marge verticale pour éviter de coller au titre
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .vertical_margin(2) // Ajout de marge verticale
        .horizontal_margin(2)
        .split(inner_area)[0];

    let items_count = app.setup_list_items.len();
    // Si trop d'items, on limite l'affichage ou on change de stratégie. 
    // Ici on assume que ça rentre ou que le Layout gère le débordement (ce qui n'est pas le cas avec constraints fixes).
    // Pour faire simple et robuste : on affiche max 8 items, sinon on devrait scroller.
    // Mais le style "Actions" est fait pour peu d'items.
    // On va garder le style "Boutons" mais avec un scroll si nécessaire ?
    // Pour l'instant, appliquons le style boutons.
    
    let constraints: Vec<Constraint> = (0..items_count).map(|_| Constraint::Length(3)).collect();

    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(content_layout);

    for (i, item) in app.setup_list_items.iter().enumerate() {
        if i < list_layout.len() {
            let is_selected = app.setup_list_state.selected() == Some(i);
            
            let (border_fg, content_bg, text_fg) = if is_selected {
                (Color::Cyan, Color::Cyan, Color::Black)
            } else {
                (Color::DarkGray, Color::Reset, Color::Gray)
            };

            let button_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(border_fg));

            let paragraph = Paragraph::new(item.as_str())
                .block(button_block)
                .style(Style::default().fg(text_fg).bg(content_bg).add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() }))
                .alignment(ratatui::layout::Alignment::Center);
            
            let button_area = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .horizontal_margin(4)
                .split(list_layout[i])[0];
            
            f.render_widget(paragraph, button_area);
        }
    }
}

fn draw_power_selection_popup(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" ⚡ Pouvoirs Spéciaux (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));
    
    let inner_area = block.inner(area);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let items: Vec<ListItem> = app.power_list_items.iter().enumerate().map(|(i, item)| {
        let style = if app.power_list_state.selected() == Some(i) {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(Line::from(item.as_str())).style(style)
    }).collect();

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(list, inner_area);
}

fn draw_inquiry_popup(f: &mut Frame, app: &App, area: Rect) {
    let (attacker_id, letter) = if let Some((id, l)) = app.pending_inquiry {
        (id, l)
    } else {
        return;
    };

    let attacker_name = if let Some(game) = &app.game {
        game.players.iter().find(|p| p.id == attacker_id).map(|p| p.name.as_str()).unwrap_or("Inconnu")
    } else {
        "Inconnu"
    };

    let block = Block::default()
        .title(" 🕵️  INTERROGATOIRE ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .style(Style::default().fg(Color::Yellow));

    // On n'utilise plus Clear car on est dans la zone d'interaction
    // f.render_widget(Clear, area); 
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Margin top
            Constraint::Length(2), // Text Question
            Constraint::Length(1), // Margin mid
            Constraint::Length(3), // Buttons
        ])
        .split(block.inner(area));

    // Text
    let question = format!("{} demande :", attacker_name);
    let detail = format!("\"Avez-vous la lettre '{}' ?\"", letter);

    let text_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(layout[1]);

    f.render_widget(Paragraph::new(question).alignment(ratatui::layout::Alignment::Center), text_area[0]);
    f.render_widget(Paragraph::new(detail).style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)).alignment(ratatui::layout::Alignment::Center), text_area[1]);

    // Buttons Layout (Horizontal)
    let button_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10), // Espace
            Constraint::Percentage(35), // OUI
            Constraint::Percentage(10), // Espace
            Constraint::Percentage(35), // NON
            Constraint::Percentage(10), // Espace
        ])
        .split(layout[3]);

    // OUI Button (Index 0)
    let oui_selected = app.pause_menu_index == 0; // On réutilise pause_menu_index pour la sélection (0=Oui, 1=Non)
    let (oui_border, oui_bg, oui_text_fg) = if oui_selected {
        (Color::Green, Color::Green, Color::Black)
    } else {
        (Color::DarkGray, Color::Reset, Color::Gray)
    };
    let oui_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(oui_border));
    let oui_text = Paragraph::new("OUI")
        .block(oui_block)
        .style(Style::default().fg(oui_text_fg).bg(oui_bg).add_modifier(if oui_selected { Modifier::BOLD } else { Modifier::empty() }))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(oui_text, button_layout[1]);

    // NON Button (Index 1)
    let non_selected = app.pause_menu_index == 1;
    let (non_border, non_bg, non_text_fg) = if non_selected {
        (Color::Red, Color::Red, Color::Black)
    } else {
        (Color::DarkGray, Color::Reset, Color::Gray)
    };
    let non_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(non_border));
    let non_text = Paragraph::new("NON")
        .block(non_block)
        .style(Style::default().fg(non_text_fg).bg(non_bg).add_modifier(if non_selected { Modifier::BOLD } else { Modifier::empty() }))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(non_text, button_layout[3]);
}



fn draw_input_position_ui(f: &mut Frame, app: &App, area: Rect) {
    // On utilise directement la zone d'interaction (pas de centrage écran)
    let block = Block::default()
        .title(" 📍 Position ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    
    f.render_widget(Clear, area); // On efface le fond
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Help
            Constraint::Length(1), // Espace
            Constraint::Length(3), // Input Box
        ])
        .margin(1)
        .split(block.inner(area));

    let instructions = Paragraph::new("Entrez la/les position(s) (ex: 1 ou 1, 3) :\n(Appuyez sur Entrée pour valider)")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(instructions, layout[0]);

    // Input Box
    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    
    let input_text = Paragraph::new(format!(" {}", app.input)) // Padding manuel
        .block(input_block)
        .alignment(ratatui::layout::Alignment::Left) // Alignement gauche
        .style(Style::default().add_modifier(Modifier::BOLD));
    
    f.render_widget(input_text, layout[2]);
    
    // Curseur
    f.set_cursor_position((layout[2].x + 2 + app.cursor_position as u16, layout[2].y + 1));
}

fn draw_suggestion_popup(f: &mut Frame, app: &App, input_area: Rect) {
    if app.suggestions.is_empty() {
        return;
    }

    // Positionner la popup juste en dessous de la zone de saisie
    let popup_area = Rect {
        x: input_area.x,
        y: input_area.y + input_area.height,
        width: input_area.width.max(20), // Largeur min
        height: (app.suggestions.len() as u16).min(5) + 2, // Hauteur selon le nombre d'items + bordures
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));
    
    f.render_widget(Clear, popup_area);
    f.render_widget(block.clone(), popup_area);

    let inner_area = block.inner(popup_area);

    let items: Vec<ListItem> = app.suggestions.iter().enumerate().map(|(i, s)| {
        let style = if i == app.suggestion_index {
            Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(s.clone()).style(style)
    }).collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);
}

fn draw_power_target_popup(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🎯 Choisir une cible pour le pouvoir (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    
    let inner_area = block.inner(area);
    f.render_widget(Clear, area); // Clear background
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

fn draw_power_letter_popup(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🔮 Voyance (Lettre) (Esc: Retour) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));
    
    f.render_widget(Clear, area);
    f.render_widget(block.clone(), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Help text
            Constraint::Length(1), // Espace
            Constraint::Length(3), // Input box
            Constraint::Length(1), // Error message
        ])
        .margin(1)
        .split(block.inner(area));

    let help_text = Paragraph::new("Entrez la lettre à révéler :")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help_text, layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(format!(" {}", app.input)) 
        .block(input_block)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Left); 
    f.render_widget(paragraph, layout[2]);
    f.set_cursor_position((layout[2].x + 2 + app.cursor_position as u16, layout[2].y + 1));

    if let Some(err) = &app.input_error {
        let error_text = Paragraph::new(err.as_str())
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_text, layout[3]);
    }
}


fn draw_power_grid(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🔮 Pouvoirs Spéciaux ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(if app.current_focus == Focus::Powers { Color::Cyan } else { Color::White }));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Limiter la hauteur des boutons
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Hauteur fixe pour les boutons
            Constraint::Min(0),    // Reste vide
        ])
        .split(inner_area);

    // Grille 2x1 pour les 2 pouvoirs
    let power_items = &app.power_list_items;
    let count = power_items.len();
    if count == 0 { return; }

    // Layout horizontal pour 2 colonnes
    let cols = 2;
    let constraints: Vec<Constraint> = (0..cols).map(|_| Constraint::Percentage(50)).collect();
    
    let grid_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .margin(1)
        .split(vertical_layout[0]);

    for (i, item) in power_items.iter().enumerate() {
        if i < grid_layout.len() {
            let is_selected = app.power_grid_state.selected() == Some(i);
            let is_focused = app.current_focus == Focus::Powers;
            
            // Style élégant : bordure et texte cyan si focus+sélectionné, sinon gris
            let (border_fg, text_fg, text_modifier) = if is_focused && is_selected {
                (Color::Cyan, Color::Cyan, Modifier::BOLD)
            } else if is_selected {
                (Color::White, Color::White, Modifier::empty())
            } else {
                (Color::DarkGray, Color::Gray, Modifier::empty())
            };

            let button_block = Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(border_fg));

            let paragraph = Paragraph::new(item.as_str())
                .block(button_block)
                .style(Style::default().fg(text_fg).add_modifier(text_modifier))
                .alignment(ratatui::layout::Alignment::Center);
            
            let button_area = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .horizontal_margin(2)
                .split(grid_layout[i])[0];

            f.render_widget(paragraph, button_area);
        }
    }
}
