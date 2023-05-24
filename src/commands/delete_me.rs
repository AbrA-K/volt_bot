use serenity::builder::CreateApplicationCommand;

use crate::User;

use super::get_db_client;

pub async fn delete_me(id: &str) -> String {
    let db = get_db_client().await;

    let _: Result<User, surrealdb::Error> = db.delete(("VoltGame", id)).await;

    "done! :(".to_string()
}

pub fn register_delete_me(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("vanish")
        .description("delete your entry in my db")
}

