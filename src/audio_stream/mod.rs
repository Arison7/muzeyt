use std::fs::File;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source, Sample};
use std::io::BufReader;
use std::sync::{Arc,Mutex};
//use tokio::sync::Mutex;
use std::collections::VecDeque;

const BLOCK_SIZE: usize = 1024;

pub fn initialize_stream() -> (Sink, OutputStream) {
    // _stream must live as long as the sink
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
   
    (sink, stream_handle)

}


pub fn append_song_from_file(path : &str, sink : &Sink, buffer : &Arc<Mutex<VecDeque<f32>>>) {
    let file = BufReader::new(File::open(path).unwrap());

    let source = Decoder::try_from(file).unwrap();
    


    // Wrap the audio source in our visualizer
    let vis_source = VisualizingSource::new(source, buffer.clone());


    sink.append(vis_source);

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
}
