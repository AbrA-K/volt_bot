use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::User;

use super::get_db_client;

pub async fn reveal(options: &[CommandDataOption], id: &str) -> String {
    let x_pos = options
        .get(0)
        .expect("expected x position")
        .resolved
        .as_ref()
        .expect("?? second expect");

    let y_pos = options
        .get(1)
        .expect("expected y position")
        .resolved
        .as_ref()
        .expect("?? second expect");

    if let (CommandDataOptionValue::Integer(x), CommandDataOptionValue::Integer(y)) = (x_pos, y_pos)
    {
        if !((1..6).contains(x) && (1..6).contains(y)) {
            return "enter a range (inclusive) from 1..5".to_string();
        }
        let db = get_db_client().await;

        let mut their_game: User = db.select(("VoltGame", id)).await.unwrap();
        let reveal_value = their_game.game.reveal(*x as u8 - 1, *y as u8 - 1);

        // loosing
        if reveal_value == 0 {
            their_game.current_score = 0;
            their_game.game.new_game();
            let _updated: Option<User> = db // type must be known, that's why the 'let _updated'
                .update(("VoltGame", id))
                .content(their_game)
                .await
                .unwrap();
            "You lost.".to_string()
        } else {
            // checking won or survived
            let mut state = "survived";
            if their_game.current_score == 0 {
                their_game.current_score = 1;
            }
            their_game.current_score *= reveal_value as usize;
            if their_game.game.check_won() {
                their_game.score += their_game.current_score;
                their_game.current_score = 0;
                their_game.game.new_game();
                state = "won!!!";
            }
            // survived
            let response = format!(
                "{}. \n ```{}``` \n score: {} \n current_score: {}",
                state,
                their_game.game.pretty_string(),
                their_game.score,
                their_game.current_score
            );
            let _updated: Option<User> = db
                .update(("VoltGame", id))
                .content(their_game)
                .await
                .unwrap();
            response
        }
    } else {
        "Please provide a valid integers".to_string()
    }
}

pub fn register_reveal(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("reveal")
        .description("reveal a square")
        .create_option(|option| {
            option
                .name("x")
                .description("the x position")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("y")
                .description("the y position")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
