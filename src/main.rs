use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use rzf_async::app::{App, AppResult};
use rzf_async::event::{Event, EventHandler};
use rzf_async::handler::handle_key_events;
use rzf_async::tui::Tui;
use std::{env, io};
use tui_textarea::{Input, Key, TextArea};

#[tokio::main]
async fn main() -> AppResult<()> {
    let path = match env::args().nth(1) {
        Some(res) => res,
        None => env::current_dir().unwrap().to_str().unwrap().to_string(),
    };
    // Create an application.
    let mut app = App::new(path);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    app.start();
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    match app.list_state.selected() {
        Some(i) => println!(
            "{}",
            app.snapshot()
                .get_matched_item(i.try_into().unwrap())
                .unwrap()
                .data
        ),
        None => {}
    }
    Ok(())
}
