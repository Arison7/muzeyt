use rodio::decoder::DecoderBuilder;
use rodio::source::SeekError;
use rodio::{OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use symphonia::core::meta::MetadataOptions;
//use tokio::sync::Mutex;
use std::collections::VecDeque;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::{get_codecs, get_probe};

pub fn initialize_stream() -> (Sink, OutputStream) {
    // _stream must live as long as the sink
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    (sink, stream_handle)
}
// Appned song from a file to the sink
pub fn append_song_from_file(
    path: &str,
    sink: &Sink,
    buffer: &Arc<Mutex<VecDeque<f32>>>,
) -> Duration {
    // I am not gonna pretend I understand fully how the symphonia works here but bierfly
    // Open file and covert it to Media source stream
    // TODO: Custom error handling instead of panic
    let file = File::open(path).expect("Incorrect file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Attempts to read file as any of the featured formats in Cargo.toml
    let probed = get_probe()
        .format(
            &Default::default(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .expect("Unsupported format");

    // Gets the file format
    let mut format = probed.format;
    let track = format.default_track().expect("No default track");

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .expect("Unsupported codec");

    let channels = track
        .codec_params
        .channels
        .as_ref()
        .map(|c| c.count() as u16)
        .unwrap_or(2);

    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);

    // Vec::<f32> is how radio is representing audio
    let mut samples = Vec::<f32>::new();

    // We go through each package and attempt to convert it to the sample format that radio knows
    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet).unwrap();
        let mut sample_buf =
            SampleBuffer::<f32>::new(decoded.frames().try_into().unwrap(), *decoded.spec());
        sample_buf.copy_interleaved_ref(decoded);
        samples.extend_from_slice(sample_buf.samples());
    }
    // Create a radio SamplesBuffer with all the necessary information
    let source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, samples);

    // Wrap up in the visualizer
    let vis_source = VisualizingSource::new(source, buffer.clone());
    // Update duration
    let duration = vis_source.total_duration().unwrap_or_default();

    // Play the song with visualizer
    sink.append(vis_source);
    // Return duration
    duration
}

/// A wrapper around a Source that stores f32 samples for visualization.
pub struct VisualizingSource<S>
where
    S: Source<Item = f32>,
{
    inner: S,
    buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl<S> VisualizingSource<S>
where
    S: Source,
{
    pub fn new(inner: S, buffer: Arc<Mutex<VecDeque<S::Item>>>) -> Self {
        Self { inner, buffer }
    }
}

impl<S> Iterator for VisualizingSource<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next()?;
        if let Ok(mut buf) = self.buffer.lock() {
            if buf.len() > 2048 {
                buf.pop_front();
            }
            buf.push_back(sample);
        }
        Some(sample)
    }
}

impl<S> Source for VisualizingSource<S>
where
    S: Source<Item = f32>,
{
    fn channels(&self) -> u16 {
        self.inner.channels()
    }
    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }
    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.inner.try_seek(pos)
    }
}
