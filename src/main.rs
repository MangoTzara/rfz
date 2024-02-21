use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rzf_async::app::{App, AppResult};
use rzf_async::event::{Event, EventHandler};
use rzf_async::handler::handle_key_events;
use rzf_async::tui::Tui;

use std::io::BufRead;
use std::{env, io};

fn get_os_path() -> Vec<String> {
    match env::args().count() {
        1 => jwalk::WalkDir::new(env::current_dir().unwrap())
            .into_iter()
            .filter_map(|path| match path {
                Ok(p) => Some(p.path().to_string_lossy().to_string()),
                Err(_) => None,
            })
            .collect(),
        _ => {
            let args: Vec<String> = env::args().collect();
            if args.contains(&"-w".to_string()) {
                jwalk::WalkDir::new(&args[2])
                    .into_iter()
                    .filter_map(|path| match path {
                        Ok(p) => Some(p.path().to_string_lossy().to_string()),
                        Err(_) => None,
                    })
                    .collect()
            } else {
                args.into_iter().skip(1).collect()
            }
        }
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut path: Vec<String> = io::stdin()
        .lock()
        .lines()
        .filter_map(|c| match c {
            Ok(string) => Some(string),
            Err(_) => None,
        })
        .collect::<Vec<String>>();
    if path.is_empty() {
        path = get_os_path();
    }
    // Create an application.
    let mut app = App::new(path);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    // app.start();
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
            Event::Paste(c) => c.chars().for_each(|c| app.update_query(c)),
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
