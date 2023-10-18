#[macro_use]
extern crate rocket;
use rocket::tokio::task;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[get("/")]
async fn index() -> String {
     return "send some number in the url".to_string();
}

#[get("/<n>")]
async fn calc_fibonacci(n: u64) -> String {
    let result = task::spawn_blocking(move || {
        fibonacci(n).to_string()
    })
    .await;

    return result.unwrap_or_default();
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, calc_fibonacci])
}
