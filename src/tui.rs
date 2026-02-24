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
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
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
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => tracker.reset()?,
                    _ => {}
                }
            }
        }

        let now = chrono::Utc::now();
        if tracker.should_stop(now) {
            return Ok(());
        }

        if tracker.should_track(now) {
            let idle_time = get_idle_time();
            tracker.tick(idle_time, now)?;
        }
    }
}

pub fn draw(frame: &mut Frame, tracker: &Tracker) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Stats
            Constraint::Min(0),     // Chart
            Constraint::Length(3),  // Footer
        ])
        .split(frame.size());

    draw_header(frame, chunks[0], tracker);
    draw_stats(frame, chunks[1], tracker);
    draw_chart(frame, chunks[2], tracker);
    draw_footer(frame, chunks[3]);
}

fn draw_header(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let now_utc = chrono::Utc::now();
    let now_local = Local::now();

    let status_text = if !tracker.should_track(now_utc) {
        Span::styled(
            format!(
                "WAITING (starts at {})",
                tracker.start_time.unwrap().format("%H:%M")
            ),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    } else if let Some(kind) = tracker.last_kind_seen {
        match kind {
            IntervalType::Focus => Span::styled(
                "IN FLOW",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            IntervalType::Idle => Span::styled(
                "IDLE",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        }
    } else {
        Span::raw("STARTING...")
    };

    let mut header_spans = vec![
        Span::styled(
            " Neflo ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        status_text,
        Span::raw(" | "),
        Span::raw(now_local.format("%Y-%m-%d %H:%M:%S").to_string()),
    ];

    if let Some(timeout) = tracker.timeout {
        let elapsed = now_utc - tracker.run_start_time;
        let remaining = timeout - elapsed;
        if remaining.num_seconds() > 0 {
            header_spans.push(Span::raw(" | Timeout: "));
            header_spans.push(Span::styled(
                format_duration(remaining.num_seconds()),
                Style::default().fg(Color::Magenta),
            ));
        }
    } else if let Some(end_time) = tracker.end_time {
        header_spans.push(Span::raw(" | End time: "));
        header_spans.push(Span::styled(
            end_time.format("%H:%M").to_string(),
            Style::default().fg(Color::Magenta),
        ));
    }

    let header_content = Line::from(header_spans);

    let header = Paragraph::new(header_content).block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn draw_stats(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let stats = calculate_stats(&tracker.db);
    let today_stats = stats
        .daily_stats
        .get(&stats.today)
        .cloned()
        .unwrap_or_default();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left side: Today's Totals
    let mut total_lines = vec![
        Line::from(Span::styled(
            "Today's Totals",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(vec![
            Span::raw("  Focus Time:    "),
            Span::styled(
                format_duration(today_stats.total_focus.num_seconds()),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Idle Time:     "),
            Span::styled(
                format_duration(today_stats.total_idle.num_seconds()),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Interruptions: "),
            Span::raw(today_stats.idle_sessions.to_string()),
        ]),
        Line::raw(""),
    ];

    // Current Session
    if tracker.last_kind_seen.is_some() {
        let session_duration = chrono::Utc::now() - tracker.state_start;
        total_lines.push(Line::from(vec![
            Span::raw("Current Session: "),
            Span::styled(
                format_duration(session_duration.num_seconds()),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let totals_para =
        Paragraph::new(total_lines).block(Block::default().title(" Totals ").borders(Borders::ALL));
    frame.render_widget(totals_para, chunks[0]);

    // Right side: Averages
    let mut avg_lines = vec![Line::from(Span::styled(
        "Averages",
        Style::default().add_modifier(Modifier::UNDERLINED),
    ))];
    if today_stats.focus_sessions > 0 {
        let avg_focus = today_stats.total_focus / (today_stats.focus_sessions as i32);
        avg_lines.push(Line::from(vec![
            Span::raw("  Avg Focus:     "),
            Span::raw(format_duration(avg_focus.num_seconds())),
        ]));
    }
    if today_stats.idle_sessions > 0 {
        let avg_idle = today_stats.total_idle / (today_stats.idle_sessions as i32);
        avg_lines.push(Line::from(vec![
            Span::raw("  Avg Idle:      "),
            Span::raw(format_duration(avg_idle.num_seconds())),
        ]));
    }

    let averages_para =
        Paragraph::new(avg_lines).block(Block::default().title(" Averages ").borders(Borders::ALL));
    frame.render_widget(averages_para, chunks[1]);
}

fn draw_chart(frame: &mut Frame, area: Rect, tracker: &Tracker) {
    let stats = calculate_stats(&tracker.db);

    let chart_block = Block::default()
        .title(" Activity - Current Week (Focus: Green, Idle: Yellow) ")
        .borders(Borders::ALL);
    let inner_area = chart_block.inner(area);
    frame.render_widget(chart_block, area);

    if inner_area.height < 2 || inner_area.width < 14 {
        return;
    }

    // Get current week (Monday to Sunday)
    let mut days_data = Vec::new();
    let mut max_total_secs = 1;

    for i in 0..7 {
        let date = stats.week_start + chrono::Duration::days(i);
        let day_stats = stats.daily_stats.get(&date).cloned().unwrap_or_default();
        let focus_secs = day_stats.total_focus.num_seconds();
        let idle_secs = day_stats.total_idle.num_seconds();
        let total_secs = focus_secs + idle_secs;
        if total_secs > max_total_secs {
            max_total_secs = total_secs;
        }
        days_data.push((date.format("%a").to_string(), focus_secs, idle_secs));
    }

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
        ])
        .split(inner_area);

    for (i, (label, focus, idle)) in days_data.into_iter().enumerate() {
        let col_area = columns[i];

        let bar_label_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(col_area);

        let bar_area = bar_label_split[0];
        let label_area = bar_label_split[1];

        // Center the bar horizontally within the column
        let bar_width = 5.min(bar_area.width);
        let bar_x_offset = (bar_area.width - bar_width) / 2;
        let centered_bar_area = Rect::new(
            bar_area.x + bar_x_offset,
            bar_area.y,
            bar_width,
            bar_area.height,
        );

        // Draw label
        frame.render_widget(
            Paragraph::new(label).alignment(ratatui::layout::Alignment::Center),
            label_area,
        );

        // Draw bar
        if centered_bar_area.height > 0 {
            let total_height = centered_bar_area.height as i64;
            let focus_height = (focus * total_height / max_total_secs) as u16;
            let idle_height = (idle * total_height / max_total_secs) as u16;

            let remaining_height = centered_bar_area
                .height
                .saturating_sub(focus_height + idle_height);

            let bar_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(remaining_height),
                    Constraint::Length(idle_height),
                    Constraint::Length(focus_height),
                ])
                .split(centered_bar_area);

            if idle_height > 0 {
                frame.render_widget(Block::default().bg(Color::Yellow), bar_chunks[1]);
            }
            if focus_height > 0 {
                frame.render_widget(Block::default().bg(Color::Green), bar_chunks[2]);
            }
        }
    }
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new("Press 'q' to quit | 'r' to reset | Neflo TUI v0.1.0")
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(help, area);
}
