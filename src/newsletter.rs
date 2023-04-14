use rocket::{serde::json::{json, Value}, form::Form, fairing, Rocket, Build, Route};
use rocket_db_pools::{Database, Connection, sqlx};


#[derive(FromForm)]
struct NewsletterForm<'r> {
    email: &'r str,
}

#[derive(Database)]
#[database("techneosis")]
struct TechneosisDB(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match TechneosisDB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => {println!("OKed"); Ok(rocket)},
            Err(e) => {
                println!("Migration Error :: {e}");
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => {
            println!("None Error");
            Err(rocket)
        }
    }

}

#[post("/subscribe", data="<newsletter_form>")]
async fn subscribe_to_newsletter(mut db: Connection<TechneosisDB>, newsletter_form: Form<NewsletterForm<'_>>) -> Value {
    let email = String::from(newsletter_form.email).to_lowercase();

    let query = sqlx::query("INSERT OR IGNORE INTO newsletter (email) VALUES (?)").bind(email.as_str());
    match query.execute(&mut* db).await {
        Ok(_) => json!({"success": "Subscribed to newsletter"}),
        Err(err) => {
            println!("{err}");
            json!({"error": "An Error Occurred"})}
    }
}

pub fn routes() -> Vec<Route> {
    routes![subscribe_to_newsletter]
}

pub fn stage() -> fairing::AdHoc {
    fairing::AdHoc::on_ignite("Newsletter Staging", |rocket| async {
        rocket.attach(TechneosisDB::init())
            .attach(fairing::AdHoc::try_on_ignite("DB Migrations", run_migrations))
    })
}
