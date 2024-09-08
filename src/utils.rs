use libpobsd::{GameDataBase, Parser, ParserResult};
use std::{error::Error, fs};

pub const DB_URL: &str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";

pub async fn get_db_from_url(db_url: &str) -> Result<GameDataBase, Box<dyn Error>> {
    let req = reqwest::get(db_url).await?;
    let content = req.text().await?;
    let games = match Parser::default().load_from_string(&content) {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => games,
    };
    let db = GameDataBase::new(games);
    Ok(db)
}

pub fn get_db_from_file(file: &str) -> Result<GameDataBase, Box<dyn Error>> {
    let games = match Parser::default().load_from_file(file)? {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => games,
    };
    let db = GameDataBase::new(games);
    Ok(db)
}

pub fn get_steam_ids(file: &str) -> Result<Option<Vec<usize>>, Box<dyn Error>> {
    let mut ids: Vec<usize> = vec![];
    let data = fs::read_to_string(file)?;
    for line in data.lines() {
        let id: usize = line.split_whitespace().collect::<Vec<&str>>()[0].parse()?;
        ids.push(id);
    }
    if ids.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ids))
    }
}
