use rocket::Shutdown;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn index() -> Template {
    Template::render(
        "index/main",
        context! {author:"ZHENG Robert", js:"create", html:"nix"},
    )
}

#[get("/shutdown")]
pub async fn shutdown(shutdown: Shutdown) -> &'static str {
    // suppose this variable is from function which produces irrecoverable error
    let result: Result<&str, &str> = Err("err");
    if result.is_err() {
        shutdown.notify();
        return "Shuting down the application.";
    }
    "Not doing anything."
}

// user
#[get("/user/login")]
pub async fn login_form() -> Template {
    Template::render("user/login", context! {author:"ZHENG Robert"})
}
