use super::get_db_client;
use crate::User;
use serenity::builder::CreateApplicationCommand;

pub async fn end_game(id: &str) -> String {
    let db = get_db_client().await;

    let their_state: Result<User, surrealdb::Error> = db.select(("VoltGame", id)).await;
    if let Ok(mut user) = their_state {
        let curr_score = user.current_score;

        user.game.new_game();

        user.current_score = 0;
        user.score += curr_score;
        let updated: Result<User, surrealdb::Error> =
            db.update(("VoltGame", id)).content(user).await;
        if let Err(e) = updated {
            return format!("something went wrong:\n `{:#?}`", e);
        }
        format!("no fun. added {} to score", curr_score)
    } else {
        "you don't seem to have a game - try `/init`".to_string()
    }
}

pub fn register_end_game(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("end").description("end your game early")
}
