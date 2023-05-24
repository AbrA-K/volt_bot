use super::get_db_client;
use crate::User;
use serenity::builder::CreateApplicationCommand;

pub async fn print_game(id: &str) -> String {
    let db = get_db_client().await;

    let their_state: Result<User, surrealdb::Error> = db.select(("VoltGame", id)).await;
    let mut response = "you don't see to have a game. try: `/init`".to_string();
    if let Ok(user) = their_state {
        response = format!("here you go \n ```{}```", user.game.pretty_string());
    }
    response
}

pub fn register_print_game(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command.name("print").description("prints your board")
}
