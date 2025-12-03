mod app;
mod audio_stream;
mod file;
mod ui;
mod utility;

use app::App;
use app::AppUpdate;
use crossterm::event::{self, Event, KeyEvent};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::app::UiUpdate;
use crate::ui::start_ui_loop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sink is initialize here to ensure it's lifetime
    let (sink, _stream) = audio_stream::initialize_stream();
    // Shared buffer (Arc + Mutex so both threads can see it)
    let buffer = Arc::new(Mutex::new(VecDeque::new()));
    // Creates a new pointer to a sink so all threads can access it
    let sink = Arc::new(sink);

    let (update_sender, mut update_receiver) = tokio::sync::mpsc::channel::<AppUpdate>(32);

    // Channel for updating Ui
    let (ui_update_sender, ui_update_receiver): (Sender<UiUpdate>, Receiver<UiUpdate>) =
        mpsc::channel(32);

    // has to run before the initialization of the app so that app can own the variables
    start_ui_loop(&buffer, &sink, ui_update_receiver);

    let mut app = App::new(
        sink,
        update_sender,
        "audio".to_string(),
        buffer,
        ui_update_sender,
    )
    .await?;

    let mut input_receiver = spawn_input_task().await;

    // main loop
    loop {
        // Handle keys
        while let Ok(key) = input_receiver.try_recv() {
            // non-blocking
            app.handle_event(key).await;
        }

        // Handle updates
        while let Ok(update) = update_receiver.try_recv() {
            app.handle_updates(update).await;
        }
        if !app.running {
            break;
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
