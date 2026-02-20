use crate::models::IntervalType;
use crate::stats::calculate_stats;
use crate::system::get_idle_time;
use crate::tracker::Tracker;
use crate::utils::format_duration;
use anyhow::Result;
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

pub fn run_tui(tracker: &mut Tracker) -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_loop(&mut terminal, tracker);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tracker: &mut Tracker,
) -> Result<()> {
    loop {
        terminal.draw(|f| draw(f, tracker))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        let idle_time = get_idle_time();
        let now = chrono::Utc::now();
        tracker.tick(idle_time, now)?;
    }
}

pub fn draw(frame: &mut Frame, tracker: &Tracker) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(frame.size());

    draw_header(frame, chunks[0], tracker);
    draw_main(frame, chunks[1], tracker);
    draw_footer(frame, chunks[2]);
}

fn draw_header(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let now = Local::now();
    let status_text = if let Some(kind) = tracker.last_kind_seen {
        match kind {
            IntervalType::Focus => Span::styled("IN FLOW", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            IntervalType::Idle => Span::styled("IDLE", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        }
    } else {
        Span::raw("STARTING...")
    };

    let header_content = Line::from(vec![
        Span::styled(" Neflo ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        status_text,
        Span::raw(" | "),
        Span::raw(now.format("%Y-%m-%d %H:%M:%S").to_string()),
    ]);

    let header = Paragraph::new(header_content)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn draw_main(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Stats
            Constraint::Percentage(60), // Chart
        ])
        .split(area);

    draw_stats(frame, chunks[0], tracker);
    draw_chart(frame, chunks[1], tracker);
}

fn draw_stats(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let stats = calculate_stats(&tracker.db);
    let today_stats = stats.daily_stats.get(&stats.today).cloned().unwrap_or_default();

    let mut lines = Vec::new();

    // Current Session
    if tracker.last_kind_seen.is_some() {
        let session_duration = chrono::Utc::now() - tracker.state_start;
        lines.push(Line::from(vec![
            Span::raw("Current Session: "),
            Span::styled(
                format_duration(session_duration.num_seconds()),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            ),
        ]));
        lines.push(Line::raw(""));
    }

    // Today's Totals
    lines.push(Line::from(Span::styled("Today's Totals", Style::default().add_modifier(Modifier::UNDERLINED))));
    lines.push(Line::from(vec![
        Span::raw("  Focus Time:    "),
        Span::styled(format_duration(today_stats.total_focus.num_seconds()), Style::default().fg(Color::Green)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Idle Time:     "),
        Span::styled(format_duration(today_stats.total_idle.num_seconds()), Style::default().fg(Color::Yellow)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Interruptions: "),
        Span::raw(today_stats.idle_sessions.to_string()),
    ]));
    lines.push(Line::raw(""));

    // Averages
    lines.push(Line::from(Span::styled("Averages", Style::default().add_modifier(Modifier::UNDERLINED))));
    if today_stats.focus_sessions > 0 {
        let avg_focus = today_stats.total_focus / (today_stats.focus_sessions as i32);
        lines.push(Line::from(vec![
            Span::raw("  Avg Focus:     "),
            Span::raw(format_duration(avg_focus.num_seconds())),
        ]));
    }
    if today_stats.idle_sessions > 0 {
        let avg_idle = today_stats.total_idle / (today_stats.idle_sessions as i32);
        lines.push(Line::from(vec![
            Span::raw("  Avg Idle:      "),
            Span::raw(format_duration(avg_idle.num_seconds())),
        ]));
    }

    let stats_para = Paragraph::new(lines)
        .block(Block::default().title(" Summary ").borders(Borders::ALL));
    frame.render_widget(stats_para, area);
}

fn draw_chart(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let stats = calculate_stats(&tracker.db);

    // Get last 7 days of focus time
    let mut data = Vec::new();
    let mut labels = Vec::new();

    for i in (0..7).rev() {
        let date = stats.today - chrono::Duration::days(i);
        let label = date.format("%a").to_string();
        let focus_mins = stats.daily_stats.get(&date)
            .map(|s| s.total_focus.num_minutes() as u64)
            .unwrap_or(0);

        // We need to keep strings alive if we use &str, or use Bar
        labels.push(label);
        data.push(focus_mins);
    }

    // Ratatui BarChart takes &[(&str, u64)]
    // We need to construct this.
    let bar_data: Vec<(&str, u64)> = labels.iter().zip(data.iter())
        .map(|(l, d)| (l.as_str(), *d))
        .collect();

    let chart = BarChart::default()
        .block(Block::default().title(" Focus Time (min) - Last 7 Days ").borders(Borders::ALL))
        .data(&bar_data)
        .bar_width(7)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD));

    frame.render_widget(chart, area);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new("Press 'q' to quit | Neflo TUI v0.1.0")
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(help, area);
}
