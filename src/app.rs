use libpobsd::{Game, GameDataBase, GameFilter, SearchType};
use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{ListItem, ListState},
};
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Search,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameItem {
    pub uid: u32,
    pub name: String,
    pub steam_owned: bool,
}

impl GameItem {
    pub fn new(uid: u32, name: String, steam_owned: bool) -> Self {
        Self {
            uid,
            name,
            steam_owned,
        }
    }
}

impl GameItem {
    fn from_game(game: &Game, steam_ids: &Option<Vec<usize>>) -> Self {
        match steam_ids {
            Some(ids) => match game.get_steam_id() {
                Some(id) => GameItem::new(game.uid, game.name.to_owned(), ids.contains(&id)),
                None => GameItem::new(game.uid, game.name.to_owned(), false),
            },
            None => GameItem::new(game.uid, game.name.to_owned(), false),
        }
    }
}

impl<'a> From<GameItem> for ListItem<'a> {
    fn from(val: GameItem) -> Self {
        let line = if val.steam_owned {
            Line::styled(
                format!("{} 🎮", val.name),
                Style::default().fg(Color::Green),
            )
        } else {
            Line::styled(val.name, Style::default())
        };
        let item = ListItem::new(line);
        item
    }
}

/// Application.
pub struct App {
    /// mode
    pub mode: InputMode,
    /// Is the application running?
    pub running: bool,
    /// game database
    pub game_db: GameDataBase,
    /// game to display
    pub games: Vec<GameItem>,
    /// list state
    pub state: ListState,
    /// search text
    pub search_text: Option<String>,
    /// list of Steam ids (owned game)
    pub steam_ids: Option<Vec<usize>>,
    /// if true, only list owned game
    pub owned_only: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: InputMode::default(),
            running: true,
            game_db: GameDataBase::new(vec![]),
            games: vec![],
            state: ListState::default(),
            search_text: None,
            steam_ids: None,
            owned_only: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game_db: GameDataBase, steam_ids: Option<Vec<usize>>) -> Self {
        Self {
            mode: InputMode::default(),
            running: true,
            games: game_db
                .get_all_games()
                .into_iter()
                .map(|g| GameItem::from_game(g, &steam_ids))
                .collect(),
            game_db,
            state: ListState::default(),
            search_text: None,
            steam_ids,
            owned_only: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_next(&mut self) {
        self.state.select_next()
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous()
    }

    pub fn select_first(&mut self) {
        self.state.select_first()
    }

    pub fn select_last(&mut self) {
        self.state.select_last()
    }

    pub fn push_search(&mut self, l: char) {
        self.search_text = match &self.search_text {
            Some(text) => {
                let mut text = text.to_owned();
                text.push(l);
                Some(text)
            }
            None => Some(String::from(l)),
        };
        self.update_game_list();
    }
    pub fn pull_search(&mut self) {
        self.search_text = match &self.search_text {
            Some(text) => {
                let mut text = text.to_owned();
                text.pop();
                Some(text)
            }
            None => None,
        };
        self.update_game_list();
    }

    pub fn update_game_list(&mut self) {
        self.select_first();
        let games = match &self.search_text {
            Some(text) => {
                let mut game_filter = GameFilter::default();
                game_filter.set_name(text);
                game_filter.set_engine(text);
                game_filter.set_runtime(text);
                game_filter.set_tag(text);
                game_filter.set_genre(text);
                let search_type = SearchType::NotCaseSensitive;
                self.game_db
                    .search_game_by_filter(&search_type, &game_filter)
            }
            None => self.game_db.get_all_games(),
        };
        let games: Vec<GameItem> = games
            .into_iter()
            .map(|g| match &self.steam_ids {
                Some(ids) => match g.get_steam_id() {
                    Some(id) => GameItem::new(g.uid, g.name.to_owned(), ids.contains(&id)),
                    None => GameItem::new(g.uid, g.name.to_owned(), false),
                },
                None => GameItem::new(g.uid, g.name.to_owned(), false),
            })
            .collect();
        if self.owned_only {
            self.games = games.into_iter().filter(|x| x.steam_owned).collect();
        } else {
            self.games = games
        }
    }

    pub fn clear_search(&mut self) {
        self.search_text = None;
        self.update_game_list();
    }

    pub fn switch_mode(&mut self) {
        match &self.mode {
            InputMode::Normal => self.mode = InputMode::Search,
            InputMode::Search => self.mode = InputMode::Normal,
        }
    }
    pub fn switch_owned_only(&mut self) {
        self.owned_only = !self.owned_only;
        self.update_game_list();
    }
}
