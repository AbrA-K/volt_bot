mod commands;
mod volt_flip;

use dotenv::dotenv;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::GuildId;
use volt_flip::VoltGame;

use std::env;
use std::u8;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    current_score: usize,
    score: usize,
    game: VoltGame,
}

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let user_id = interaction
            .clone()
            .application_command()
            .unwrap()
            .user
            .id
            .to_string();

        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "id" => commands::get_score::get_score(&command.data.options).await,
                "score" => commands::get_score::get_score(&command.data.options).await,
                "vanish" => commands::delete_me::delete_me(&user_id).await,
                "init" => commands::init_me::init_me(&user_id).await,
                "end" => commands::end_game::end_game(&user_id).await,
                "print" => commands::print_game::print_game(&user_id).await,
                "reveal" => commands::reveal::reveal(&command.data.options, &user_id).await,
                _ => "not implemented :(".to_string(),
            };
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::get_score::register_score(command))
                .create_application_command(|command| commands::delete_me::register_delete_me(command))
                .create_application_command(|command| commands::init_me::register_init_me(command))
                .create_application_command(|command| commands::end_game::register_end_game(command))
                .create_application_command(|command| commands::print_game::register_print_game(command))
                .create_application_command(|command| commands::reveal::register_reveal(command))
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
