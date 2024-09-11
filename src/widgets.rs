use libpobsd::Game;
use ratatui::prelude::*;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, List, Paragraph, Wrap};

use crate::app::{App, GameItem, InputMode};

fn add_field(text: &mut Vec<Line>, field_name: &str, field: &Option<String>) {
    if let Some(field) = field {
        let line = Line::from(vec![
            Span::styled(format!("{}: ", field_name), Style::new().blue().bold()),
            Span::raw(field.to_owned()),
        ]);
        text.push(line);
    }
}

fn add_vec_field(text: &mut Vec<Line>, field_name: &str, field: &Option<Vec<String>>) {
    if let Some(field) = field {
        let line = Line::from(vec![
            Span::styled(format!("{}: ", field_name), Style::new().blue().bold()),
            Span::raw(field.join(", ").to_owned()),
        ]);
        text.push(line);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GameDetailsWidget {
    game: Option<Game>,
}

impl GameDetailsWidget {
    pub fn new(game: Option<Game>) -> Self {
        Self { game }
    }
}

impl Widget for GameDetailsWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.game {
            Some(game) => {
                let mut text: Vec<Line> = Vec::new();
                let name = Line::from(Span::styled(
                    game.name.to_uppercase(),
                    Style::new().underlined().bold().red(),
                ));
                text.push(name);
                text.push(Line::default());

                add_field(&mut text, "Engine", &game.engine);
                add_field(&mut text, "Runtime", &game.runtime);
                add_field(&mut text, "Setup", &game.setup);
                add_field(&mut text, "Hints", &game.hints);
                add_field(&mut text, "Year", &game.year);
                add_vec_field(&mut text, "Genres", &game.genres);
                add_vec_field(&mut text, "Tags", &game.tags);
                add_vec_field(&mut text, "Developers", &game.devs);
                add_vec_field(&mut text, "Publishers", &game.publis);

                if let Some(stores) = &game.stores {
                    let mut store_names: Vec<String> = Vec::new();
                    for store in &stores.0 {
                        store_names.push(store.store.to_string())
                    }
                    let line = Line::from(vec![
                        Span::styled("Stores: ", Style::new().blue().bold()),
                        Span::raw(store_names.join(", ")),
                    ]);
                    text.push(line);
                }
                let p = Paragraph::new(text)
                    .block(
                        Block::bordered()
                            .title("Game Details")
                            .title_alignment(Alignment::Center)
                            .border_type(BorderType::Rounded),
                    )
                    .wrap(Wrap { trim: false });
                p.render(area, buf)
            }
            None => {
                let mut text: Vec<Line> = Vec::new();
                let name = Line::from(Span::styled(
                    "select a game".to_uppercase(),
                    Style::new().underlined().bold().red(),
                ));
                text.push(name);
                let p = Paragraph::new(text).block(
                    Block::bordered()
                        .title("Game Details")
                        .title_alignment(Alignment::Center)
                        .border_type(BorderType::Rounded),
                );
                p.render(area, buf)
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SearchBoxWidget {
    search_text: Option<String>,
}

impl SearchBoxWidget {
    pub fn new(search_text: Option<String>) -> Self {
        Self { search_text }
    }
}

impl StatefulWidget for SearchBoxWidget {
    type State = InputMode;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let border_type = match state {
            InputMode::Normal => BorderType::Plain,
            InputMode::Search => BorderType::Double,
        };
        let search_box = match &self.search_text {
            Some(text) => Paragraph::new(text.to_owned()).block(
                Block::bordered()
                    .title("Search")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded)
                    .border_type(border_type),
            ),
            None => Paragraph::new("".to_owned()).block(
                Block::bordered()
                    .title("Search")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded)
                    .border_type(border_type),
            ),
        };
        match state {
            InputMode::Normal => search_box.render(area, buf),
            InputMode::Search => search_box.red().bold().render(area, buf),
        };
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LeftPanelWidget<'a> {
    pub game_list: List<'a>,
    pub search_box: SearchBoxWidget,
}

impl<'a> LeftPanelWidget<'a> {
    pub fn new(search_text: Option<String>, items: Vec<GameItem>) -> Self {
        let game_list = List::new(items)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .style(Style::default().fg(Color::Cyan));
        Self {
            game_list,
            search_box: SearchBoxWidget::new(search_text),
        }
    }
}

impl<'a> StatefulWidget for LeftPanelWidget<'a> {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list_title = if state.owned_only {
            "Game List (owned only)"
        } else {
            "Game List"
        };
        let list_block = Block::bordered()
            .title(list_title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        list_block.render(area, buf);

        let list_chunk = Layout::default()
            .horizontal_margin(2)
            .vertical_margin(1)
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        StatefulWidget::render(self.game_list, list_chunk[1], buf, &mut state.state);
        StatefulWidget::render(self.search_box, list_chunk[0], buf, &mut state.mode);
    }
}
