
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
