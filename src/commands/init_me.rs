use super::get_db_client;
use crate::volt_flip::VoltGame;
use crate::User;
use serenity::builder::CreateApplicationCommand;

pub async fn init_me(id: &str) -> String {
    let db = get_db_client().await;

    let their_game_maybe: Result<User, surrealdb::Error> = db.select(("VoltGame", id)).await;
    if let Ok(user) = their_game_maybe {
        return format!(
            "man, you already have a board: \n ```{}```",
            user.game.pretty_string()
        );
    }

    let mut game = VoltGame::get_new_game();
    game.new_game();

    let _new_game: User = db
        .create(("VoltGame", id))
        .content(User {
            game,
            current_score: 1,
            score: 0,
        })
        .await
        .unwrap();
    "done! :)".to_string()
}

pub fn register_init_me(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("init").description("add yourself to my db")
}
