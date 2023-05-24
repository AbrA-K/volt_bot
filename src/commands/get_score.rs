use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::User;
use crate::commands::get_db_client;

pub async fn get_score(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(user, _member) = option {
        let db = get_db_client().await;

        let their_game: User = db.select(("VoltGame", &user.id.0.to_string())).await.unwrap();

        format!(
            "name: {} \nscore: {} \ncurrent score: {}",
            user.name, their_game.score, their_game.current_score
        )
    } else {
        "Please provide a valid user".to_string()
    }
}

pub fn register_score(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("score")
        .description("Get the score of a user")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to lookup")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
