use rocket::{get, launch, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![hello])
}

fn main() {
    rocket().launch();
}
