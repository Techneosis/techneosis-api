mod newsletter;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(newsletter::stage())
        .mount("/api/newsletter/", newsletter::routes())
}
