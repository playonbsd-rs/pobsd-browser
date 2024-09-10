use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    app::App,
    widgets::{GameDetailsWidget, LeftPanelWidget},
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    // Render left panel
    let left_panel = LeftPanelWidget::new(app.search_text.clone(), app.games.clone());
    frame.render_stateful_widget(left_panel, layout[0], app);

    // Let's grab the selected game and display it
    let game = match app.state.selected() {
        Some(i) => {
            let uid = app.games.get(i).unwrap().uid;
            app.game_db.get_game_by_id(uid).cloned()
        }
        None => None,
    };
    let game_widget = GameDetailsWidget::new(game);
    frame.render_widget(game_widget, layout[1])
}
