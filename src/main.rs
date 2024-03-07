use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rfz::app::{App, AppResult};
use rfz::event::{Event, EventHandler};
use rfz::handler::handle_key_events;
use rfz::tui::Tui;
use std::fs::FileType;
use std::io::{BufRead, Error, ErrorKind, IsTerminal};
use std::path::Path;
use std::{env, io, process};
use tokio::main;

const HELP: &str = "Usage: rfz [OPTION] [PATH] \n
    Options: \n
        -h Show this help message \n
    Path mode option:  \n
        -f Search only between files from the given PATH \n
        -d Search only between directory from the given PATH         \n
        -w Search between both files and directory  from the given PATH \n
    ";
#[inline]
fn get_os_path() -> Result<Vec<String>, Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => crawl_directory(env::current_dir().unwrap(), |_| true),
        2 => {
            if args[1] == "-h" {
                println!("{}", HELP);
                process::exit(0)
            };
            input_error(&format!("Error: {} needs a starting directory", args[1]))
        }
        3 => {
            let mode: Result<fn(FileType) -> bool, Error> = match args[1].as_str() {
                "-d" => Ok(|entry: FileType| entry.is_dir()),
                "-f" => Ok(|entry: FileType| entry.is_file()),
                "-w" => Ok(|_: FileType| true),
                _ => input_error("No such option"),
            };
            crawl_directory(&args[2], mode.expect("Error"))
        }
        _ => input_error("Error"),
    }
}
#[inline(always)]
fn input_error<T>(msg: &str) -> Result<T, Error> {
    Err(Error::new(ErrorKind::InvalidInput, msg))
}
#[inline]
fn crawl_directory<P: AsRef<Path>>(
    path: P,
    predicate: fn(FileType) -> bool,
) -> Result<Vec<String>, Error> {
    match jwalk::WalkDir::new(path).try_into_iter() {
        Ok(c) => Ok(c
            .filter_map(|path| match path {
                Ok(p) if predicate(p.file_type) => Some(p.path().to_string_lossy().to_string()),
                Ok(_) | Err(_) => None,
            })
            .collect()),
        Err(_) => input_error("Error: starting directory does not exist"),
    }
}

#[main]
async fn main() -> AppResult<()> {
    let mut path: Vec<String> = Vec::new();
    let input = io::stdin();
    if input.is_terminal() {
        // no input available
        match get_os_path() {
            Ok(p) => path = p,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
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
