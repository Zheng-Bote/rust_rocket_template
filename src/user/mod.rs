use rocket_dyn_templates::{context, Template};

#[get("/user/login")]
pub async fn login_form() -> Template {
    Template::render("user/login", context! {author:"ZHENG Robert"})
}
