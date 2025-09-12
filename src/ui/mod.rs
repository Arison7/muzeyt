use ratatui::{
    prelude::*,
    widgets::{BarChart, Block, Borders},
    Terminal,
};
use std::collections::VecDeque;
use std::io::Stdout;
use std::sync::{Arc, Mutex};
use rustfft::{FftPlanner, num_complex::Complex};

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
    let mut buffer: Vec<Complex<f32>> =
        samples.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();

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


pub fn draw_visualization(
    buffer: &Arc<Mutex<VecDeque<f32>>>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    let bars = {
        let buf = buffer.lock().unwrap();
        let samples: Vec<f32> = buf.iter().cloned().collect();
        compute_spectrum(&samples)
    };

    terminal
        .draw(|frame| {
            let area = frame.area();
            let data: Vec<(&str, u64)> = bars.iter().map(|v| ("", (v * 100.0) as u64)).collect();

            let chart = BarChart::default()
                .block(Block::default().title("Now Playing").borders(Borders::ALL))
                .data(&data)
                .value_style(Style::reset().bg(Color::White))
                .bar_width(1);

            frame.render_widget(chart, area);
        })
        .unwrap();
}
