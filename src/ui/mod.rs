use ratatui::{
    prelude::*,
    widgets::{BarChart, Block, Borders},
    Terminal,
};
use std::collections::VecDeque;
use std::io::Stdout;
use std::sync::{Arc, Mutex};

pub const BAR_COUNT: usize = 40;

/// Collapse raw samples into N bars by grouping + averaging energy.
pub fn compute_bars(samples: &[f32]) -> Vec<f32> {
    if samples.is_empty() {
        return vec![0.0; BAR_COUNT];
    }

    (0..BAR_COUNT)
        .map(|i| {
            let start = i * samples.len() / BAR_COUNT;
            let end = (i + 1) * samples.len() / BAR_COUNT;
            let chunk = &samples[start..end];
            let sum: f32 = chunk.iter().map(|s| s * s).sum();
            (sum / chunk.len().max(1) as f32).sqrt()
        })
        .collect()
}
pub fn draw_visualization(
    buffer: &Arc<Mutex<VecDeque<f32>>>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    labels: &Vec<String>,
) {
    let bars = {
        let buf = buffer.lock().unwrap();
        let samples: Vec<f32> = buf.iter().cloned().collect();
        compute_bars(&samples)
    };

    terminal
        .draw(|frame| {
            let area = frame.area();
            let data: Vec<(&str, u64)> = bars
                .iter()
                .enumerate()
                .map(|(i, v)| (labels[i].as_str(), (v * 100.0) as u64))
                .collect();

            let chart = BarChart::default()
                .block(Block::default().title("Now Playing").borders(Borders::ALL))
                .data(&data)
                .bar_width(1);

            frame.render_widget(chart, area);
        })
        .unwrap();
}
