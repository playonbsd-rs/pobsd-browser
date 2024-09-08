use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod cmd;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;
pub mod utils;
pub mod widgets;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cmd_args = cmd::get_args().get_matches();
    // Get game database
    let game_db = match cmd_args.get_one::<String>("file") {
        Some(filename) => utils::get_db_from_file(filename)?,
        None => match cmd_args.get_one::<String>("url") {
            Some(url) => utils::get_db_from_url(url).await?,
            None => match cmd_args.get_one::<bool>("official") {
                Some(off) => {
                    if *off {
                        utils::get_db_from_url(utils::DB_URL).await?
                    } else {
                        unreachable!("At least one argument is required")
                    }
                }
                None => unreachable!("At least one argument is required"),
            },
        },
    };
    let steam_ids = match cmd_args.get_one::<String>("steam_ids") {
        Some(file) => utils::get_steam_ids(file)?,
        None => None,
    };
    // Create an application.
    let mut app = App::new(game_db, steam_ids);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
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
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
