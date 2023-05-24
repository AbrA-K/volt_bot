use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root};

pub mod delete_me;
pub mod init_me;
pub mod reveal;
pub mod get_score;
pub mod end_game;
pub mod print_game;

// get's the db already logged in with the config I want
async fn get_db_client() -> Surreal<Client> {
    let surr_user = std::env::var("SURR_USER").unwrap();
    let surr_pass = std::env::var("SURR_PASS").unwrap();
    let surr_url = std::env::var("SURR_URL").unwrap();

    let db = Surreal::new::<Ws>(surr_url).await.unwrap();

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: &surr_user,
        password: &surr_pass,
    })
    .await
    .unwrap();

    db.use_ns("VoltBoard").use_db("VolBoardDB").await.unwrap();
    db
}
