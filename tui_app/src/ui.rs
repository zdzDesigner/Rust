use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    // 创建整体布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 标题
            Constraint::Min(5),    // 列表
            Constraint::Length(3), // 状态栏
        ])
        .split(f.area());

    draw_title(f, chunks[0]);
    draw_list(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);

    // 如果在添加模式，绘制输入框
    if app.add_mode {
        let input_area = centered_rect(60, 20, f.area());
        draw_input(f, app, input_area);
    }
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("  📝 Rust Todo App")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn draw_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|todo| {
            let (icon, style) = if todo.completed {
                ("✅", Style::default().fg(Color::Green))
            } else {
                ("⬜", Style::default().fg(Color::Yellow))
            };

            let line = Line::from(vec![
                Span::styled(format!(" {icon} "), style),
                Span::styled(
                    &todo.text,
                    if todo.completed {
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::CROSSED_OUT)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Todos ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    let mut state = ListState::default();
    state.select(Some(app.selected));
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let help = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k] ", Style::default().fg(Color::Yellow)),
        Span::raw("Navigate  "),
        Span::styled("[Space] ", Style::default().fg(Color::Yellow)),
        Span::raw("Toggle  "),
        Span::styled("[a] ", Style::default().fg(Color::Yellow)),
        Span::raw("Add  "),
        Span::styled("[d] ", Style::default().fg(Color::Yellow)),
        Span::raw("Delete  "),
        Span::styled("[q] ", Style::default().fg(Color::Yellow)),
        Span::raw("Quit"),
    ]))
    .style(Style::default().fg(Color::White))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(help, area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title(" Add Todo (Enter to confirm, Esc to cancel) ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(input, area);

    // 显示光标
    f.set_cursor_position((area.x + app.input.len() as u16 + 1, area.y + 1));
}

/// 辅助函数：居中显示区域
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
