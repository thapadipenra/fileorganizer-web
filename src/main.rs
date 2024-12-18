#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

mod lib;

#[derive(FromForm)]
struct OrganizeForm {
    directory: String,
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("tool_link", "/static/fileorganizer-cli.zip");
    Template::render("index", &context)
}

#[get("/contact")]
fn contact() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("contact", &context)
}

#[post("/organize", data = "<form>")]
fn organize(form: Form<OrganizeForm>) -> Redirect {
    let directory = &form.directory;
    lib::organize_files(directory);
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, contact, organize])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
