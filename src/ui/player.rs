use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::{BarChart, Block, Borders, Gauge},
    Terminal,
};
use super::debug::build_debug_widget;
use rustfft::{num_complex::Complex, FftPlanner};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;


pub const BAR_COUNT: usize = 64;

/// Collapse raw samples into N bars by grouping + averaging energy.
/// TODO: put this in proper place
pub fn compute_spectrum(samples: &[f32]) -> Vec<f32> {
    if samples.is_empty() {
        return vec![0.0; BAR_COUNT];
    }

    // prepare FFT
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(samples.len());

    // convert samples to complex
    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();

    // run FFT
    fft.process(&mut buffer);

    // compute magnitudes
    let mags: Vec<f32> = buffer
        .iter()
        .skip(1)
        .map(|&m| (1.0 + m).log10())
        .map(|c| (c.re.powi(2) + c.im.powi(2)).sqrt())
        .collect();
    // collapse into BAR_COUNT groups (linear for now, could do log scale)
    let mut bars = Vec::with_capacity(BAR_COUNT);
    let bins_per_bar = mags.len() / BAR_COUNT;
    for i in 0..BAR_COUNT {
        let start = i * bins_per_bar;
        let end = ((i + 1) * bins_per_bar).min(mags.len());
        let chunk = &mags[start..end];
        let avg = if !chunk.is_empty() {
            chunk.iter().copied().sum::<f32>() / chunk.len() as f32
        } else {
            0.0
        };
        bars.push(avg);
    }

    bars
}

pub fn draw_player_ui(
    buffer: &Arc<Mutex<VecDeque<f32>>>,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    total_duration: Duration,
    current_progress: Duration,
    debug_lines: &Arc<Mutex<Vec<String>>>,
) {
    let debug_lines : Vec<String> = {
        let unlocked = debug_lines.lock().unwrap();
        unlocked.clone()
    };
    terminal
        .draw(|frame| {
            let constraints = if debug_lines.is_empty() {
                vec![
                    Constraint::Length(3), // progress
                    Constraint::Min(0),    // visualization
                ]
            } else {
                vec![
                    Constraint::Length(3), // progress
                    Constraint::Min(0),    // visualization
                    Constraint::Length(5), // debug panel
                ]
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(frame.area());

            // progress bar
            let gauge = build_progress_bar(total_duration, current_progress);
            frame.render_widget(gauge, chunks[0]);

            // visualization
            let chart = build_visualization(buffer);
            frame.render_widget(chart, chunks[1]);

            // only render debug if not empty
            if !debug_lines.is_empty() {
                let debug = build_debug_widget(debug_lines,frame.area());
                frame.render_widget(debug, chunks[2]);
            }

        })
        .unwrap();
}

fn build_progress_bar(
    total_duration: Duration,
    current_progress: Duration,
) -> impl ratatui::widgets::Widget {
    let percent = (current_progress.as_secs_f64() / total_duration.as_secs_f64()).min(1.0);

    Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .ratio(percent)
        .label(format!(
            "{:02}:{:02} / {:02}:{:02}",
            current_progress.as_secs() / 60,
            current_progress.as_secs() % 60,
            total_duration.as_secs() / 60,
            total_duration.as_secs() % 60
        ))
}

fn build_visualization(buffer: &Arc<Mutex<VecDeque<f32>>>) -> impl ratatui::prelude::Widget {
    let bars = {
        let buf = buffer.lock().unwrap();
        let samples: Vec<f32> = buf.iter().cloned().collect();
        compute_spectrum(&samples)
    };

    let data: Vec<(&str, u64)> = bars.iter().map(|v| ("", (v * 100.0) as u64)).collect();

    BarChart::default()
        .block(Block::default().title("Now Playing").borders(Borders::ALL))
        .data(&data)
        .value_style(Style::reset().bg(Color::White))
        .bar_width(1)
}
