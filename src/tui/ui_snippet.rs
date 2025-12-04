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

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.pause_menu_index {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Span::styled(format!(" {} ", item), style))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, layout[1]);
}
