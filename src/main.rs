#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
use rocket::{serde::json::Json, State};
use serde_json::Value;

mod templates;

struct ServiceState<'a> {
    registry: Handlebars<'a>,
}

// TODO: send JSON data to Handlebars
#[post("/latex/<template>", format = "json", data = "<cards>")]
fn get_latex(service_state: &State<ServiceState>, template: &str, cards: Json<Value>) -> String {
    let result = service_state
        .registry
        .render(template, &json!({"name": "foo"}));

    dbg!(cards);
    // TODO: return 400 type status on error

    match result {
        Ok(s) => String::from(s),
        Err(_) => "Error".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    let reg: handlebars::Handlebars = templates::create_templates();

    rocket::build()
        .manage(ServiceState { registry: reg })
        .mount("/", routes![get_latex])
}
