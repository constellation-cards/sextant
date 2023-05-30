#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

mod templates;

#[post("/", format = "json")]
fn get_latex() -> String {
    let reg = templates::create_templates();
    let result = reg.render("standard", &json!({"name": "foo"}));
    match result {
        Ok(s) => String::from(s),
        Err(_) => "Error".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_latex])
}
