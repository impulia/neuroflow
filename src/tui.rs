use crate::report::{format_duration, ReportData, Reporter};
use anyhow::Result;
use chrono::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub fn show_tui(reporter: Reporter) -> Result<()> {
    let data = reporter.get_data()?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, data);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, data: ReportData) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &data))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') | KeyCode::Esc = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, data: &ReportData) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let header = Paragraph::new("Neflo Report - Press 'q' to exit")
        .block(Block::default().borders(Borders::ALL).title("Neflo"));
    f.render_widget(header, chunks[0]);

    // Prepare data for BarChart
    let days: Vec<_> = (0..7)
        .map(|i| data.week_start + Duration::days(i))
        .collect();

    let chart_labels: Vec<String> = days.iter().map(|d| d.format("%a %d").to_string()).collect();

    let focus_minutes: Vec<u64> = days
        .iter()
        .map(|d| {
            data.daily_stats
                .get(d)
                .map(|s| s.total_focus.num_minutes() as u64)
                .unwrap_or(0)
        })
        .collect();

    let bar_data: Vec<(&str, u64)> = chart_labels
        .iter()
        .zip(focus_minutes.iter())
        .map(|(l, v)| (l.as_str(), *v))
        .collect();

    let chart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Weekly Focus Time (minutes)"),
        )
        .data(&bar_data)
        .bar_width(10)
        .bar_gap(2)
        .style(Style::default().fg(Color::Green))
        .value_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(chart, chunks[1]);

    // Summary
    let mut week_focus = Duration::zero();
    let mut week_idle = Duration::zero();
    let mut week_interruptions = 0;

    for (date, stats) in &data.daily_stats {
        if *date >= data.week_start {
            week_focus = week_focus + stats.total_focus;
            week_idle = week_idle + stats.total_idle;
            week_interruptions += stats.idle_sessions;
        }
    }

    let summary_lines = vec![
        Line::from(vec![
            "Weekly Total Focus: ".into(),
            format_duration(week_focus).into(),
        ]),
        Line::from(vec![
            "Weekly Total Idle:  ".into(),
            format_duration(week_idle).into(),
        ]),
        Line::from(vec![
            "Total Interruptions: ".into(),
            week_interruptions.to_string().into(),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Daily Details:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ];

    let mut details = summary_lines;
    for (date, stats) in data.daily_stats.iter().rev() {
        if *date < data.week_start {
            continue;
        }
        let date_str = if *date == data.today {
            format!("{} (Today)", date)
        } else {
            date.to_string()
        };
        details.push(Line::from(format!(
            "{}: Focus {}, Idle {}, Interruptions {}",
            date_str,
            format_duration(stats.total_focus),
            format_duration(stats.total_idle),
            stats.idle_sessions
        )));
    }

    let summary = Paragraph::new(details).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Weekly Summary"),
    );
    f.render_widget(summary, chunks[2]);

    let footer = Paragraph::new(format!("Today is {}", data.today))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}
