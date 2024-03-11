use clap::{arg, command};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rfz::app::{App, AppResult};
use rfz::event::{Event, EventHandler};
use rfz::handler::handle_key_events;
use rfz::tui::Tui;
use std::fs::FileType;
use std::io::{BufRead, IsTerminal};
use std::path::Path;
use std::process;
use std::{env, io};
use tokio::main;

fn get_os_path() -> Option<Vec<String>> {
    let mut args = command!().args([
        arg!(--file <PATH>)
            .short('f')
            .help("Search files from the given PATH")
            .exclusive(true),
        arg!(--directory <PATH>)
            .short('d')
            .help("Search directories from the given PATH ")
            .exclusive(true),
        arg!(--"working-dir" <PATH>)
            .short('w')
            .help("Search  directories and files from the given PATH ")
            .exclusive(true),
    ]);
    let matches = &args.clone().get_matches();
    if let Some(file) = matches.get_one::<String>("file") {
        return Some(crawl_directory(file, |entry: FileType| entry.is_file()));
    }

    if let Some(directory) = matches.get_one::<String>("directory") {
        return Some(crawl_directory(directory, |entry: FileType| entry.is_dir()));
    }

    if let Some(working_dir) = matches.get_one::<String>("working-dir") {
        return Some(crawl_directory(working_dir, |_: FileType| true));
    }
    args.print_help().ok();
    None
}

fn crawl_directory<P: AsRef<Path>>(path: P, predicate: fn(FileType) -> bool) -> Vec<String> {
    jwalk::WalkDir::new(path)
        .skip_hidden(false)
        .into_iter()
        .filter_map(|path| match path {
            Ok(p) if predicate(p.file_type) => Some(p.path().to_string_lossy().to_string()),
            Ok(_) | Err(_) => None,
        })
        .collect()
}

#[main]
async fn main() -> AppResult<()> {
    let mut path: Vec<String> = Vec::new();
    let input = io::stdin();
    if input.is_terminal() {
        // no input available
        match get_os_path() {
            Some(p) => path = p,
            None => process::exit(0),
        }
    } else {
        // input available
        input.lock().lines().for_each(|c| {
            if let Ok(to_push) = c {
                path.push(to_push)
            }
        });
    }
    // Create an application.
    let mut app = App::new(path.as_slice());

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;
    terminal
        .backend_mut()
        .execute(crossterm::terminal::SetTitle("rfz"))
        .expect("Couldn't change title");
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

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

    if let Some(selected) = app.list_state.selected() {
        println!(
            "{}",
            app.snapshot()
                .get_matched_item(selected.try_into().unwrap())
                .unwrap()
                .data
        )
    };

    Ok(())
}
