#[macro_use]
extern crate rocket;
use rocket::tokio::task;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[get("/<n>")]
async fn hello(n: u32) -> String {
    let result = task::spawn_blocking(move || {
        fibonacci(n).to_string()
    })
    .await;

    return result.unwrap_or_default();
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
