mod app;
mod audio_stream;
mod ui;
mod file;


use app::App;
use app::AppUpdate;
use crossterm::event::{self, Event, KeyEvent};
use tokio::sync::mpsc::{self, Receiver};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (sink, stream) = audio_stream::initialize_stream();

    let (update_sender, mut update_receiver) = tokio::sync::mpsc::channel::<AppUpdate>(32);

    let mut app = App::new(sink, stream,update_sender).await?;

    let mut input_receiver = spawn_input_task().await;



    // main loop
    loop {
        // Handle keys
        while let Ok(key) = input_receiver.try_recv() {   // <-- non-blocking
            app.handle_event(key).await;
        }


        // Handle updates
        if !app.running {
            break;
        }

        while let Ok(update) = update_receiver.try_recv() {
            app.handle_updates(update);
        }
    }

    ratatui::restore();
    Ok(())
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

