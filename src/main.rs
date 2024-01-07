#[macro_use]
extern crate rocket;

use log::LevelFilter;

use rocket::fs::{relative, FileServer};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

use rocket_eval::catchers;
use rocket_eval::fairings::cors::CORS;
use rocket_eval::fairings::csrf::Csrf;
use rocket_eval::routes::{self};

#[launch]
async fn rocket() -> Rocket<Build> {
    setup_logger();

    rocket::build()
        .mount(
            "/",
            routes![routes::shutdown, routes::index, routes::login_form],
        )
        .mount("/assets", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .attach(Csrf::new())
        .attach(CORS)
        .manage(CORS)
        .register(
            "/",
            catchers![
                catchers::bad_request,
                catchers::not_found,
                catchers::unprocessable_entity,
                catchers::internal_server_error
            ],
        )
}

fn setup_logger() {
    let (level, logger) = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}] [{level}][{target}] [{message}]",
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            fern::log_file("logs/application.log")
                .unwrap_or_else(|_| panic!("Cannot open logs/application.log")),
        )
        .into_log();
    async_log::Logger::wrap(logger, || 0).start(level).unwrap();
}
