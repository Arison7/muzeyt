mod app;
mod audio_stream;
mod ui;


use app::App;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::Frame;
use tokio::sync::mpsc::{self, Receiver};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (sink, stream) = audio_stream::initialize_stream();


    let mut app = App::new(sink, stream).await?;

    let mut rx = spawn_input_task().await;


    // main loop
    loop {
        // Handle keys
        while let Ok(key) = rx.try_recv() {   // <-- non-blocking
            app.handle_event(key).await?;
        }


        // Handle updates
        if !app.running {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

async fn spawn_input_task() -> Receiver<KeyEvent> {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(async move {
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if tx.send(key).await.is_err() {
                    break;
                }
            }
        }
    });

    rx
}
