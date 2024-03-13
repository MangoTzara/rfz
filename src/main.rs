use async_walkdir::WalkDir;
use clap::{arg, command};
use crossterm::ExecutableCommand;

use futures_util::{StreamExt, TryStreamExt};
use nucleo::{Injector, Utf32String};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use rfz::app::{App, AppResult};
use rfz::event::{Event, EventHandler};
use rfz::handler::handle_key_events;
use rfz::tui::Tui;
use std::fmt::Error;
use std::fs::FileType;
use std::io::{BufRead, IsTerminal};
use std::path::Path;
use std::process;

use std::{env, io};
use tokio::main;

fn get_os_path(injector: Injector<String>) -> Result<(), Error> {
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
    let matches = args.clone().get_matches().clone();
    if let Some(file) = matches.get_one::<String>("file") {
        tokio::spawn(crawl_directory(
            file.clone(),
            injector,
            |entry: FileType| entry.is_file(),
        ));
        return Ok(());
    }

    if let Some(directory) = matches.get_one::<String>("directory") {
        tokio::spawn(crawl_directory(
            directory.clone(),
            injector,
            |entry: FileType| entry.is_file(),
        ));
        return Ok(());
    }

    if let Some(working_dir) = matches.get_one::<String>("working-dir") {
        tokio::spawn(crawl_directory(
            working_dir.clone(),
            injector,
            |_: FileType| true,
        ));
        return Ok(());
    }
    args.print_help().ok();
    Err(std::fmt::Error::default())
}

async fn crawl_directory<P: AsRef<Path>>(
    path: P,
    injector: Injector<String>,
    predicate: fn(FileType) -> bool,
) {
    let entries: WalkDir = async_walkdir::WalkDir::new(path);
    let mut stream = entries.into_stream();
    while let Some(item) = stream.next().await {
        match item {
            Ok(ok) => {
                let c = ok.path().to_string_lossy().to_string();

                if predicate(ok.file_type().await.expect("!!!")) {
                    injector.push(c.clone(), |s| {
                        s[0] = Utf32String::Ascii(c.to_string().into());
                    });
                }
            }
            Err(_) => {}
        }
    }
}

#[main]
async fn main() -> AppResult<()> {
    // Create an application.
    // let mut app = App::new(path.as_slice());

    let mut app = App::default();
    let injector = app.injector();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;
    terminal
        .backend_mut()
        .execute(crossterm::terminal::SetTitle("rfz"))
        .expect("Couldn't change title");
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    let input = io::stdin();
    if input.is_terminal() {
        // no input available
        if let Err(_) = get_os_path(injector) {
            process::exit(0);
        }
    } else {
        // input available
        input.lock().lines().for_each(|c| {
            if let Ok(to_push) = c {
                app.add_item(to_push)
            }
        });
    }
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
