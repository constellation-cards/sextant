use handlebars::Handlebars;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

#[get("/")]
fn index() -> String {
    let reg = Handlebars::new();
    let result = reg.render_template("Hello {{name}}", &json!({"name": "foo"}));
    match result {
        Ok(s) => String::from(s),
        Err(_) => "Error".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}