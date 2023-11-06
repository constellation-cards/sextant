#[macro_use]
extern crate rocket;

use rocket::State;
use serde_json::Value;
use upon::Engine;

mod templates;

struct ServiceState<'a> {
    registry: Engine<'a>,
}

#[post("/latex/<template>", format = "json", data = "<body>")]
fn get_latex(service_state: &State<ServiceState>, template: &str, body: &str) -> String {
    let cards: Value = match serde_json::from_str(body) {
        Ok(json) => json,
        Err(e) => return e.to_string(),
    };

    let template = match service_state.registry.get_template(template) {
        None => return String::from("No such template"),
        Some(template) => template,
    };

    let result = template.render(&cards).to_string();

    // TODO: return 400 type status on error

    match result {
        Ok(s) => String::from(s),
        Err(e) => e.to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    let engine = templates::create_templates();

    rocket::build()
        .manage(ServiceState { registry: engine })
        .mount("/", routes![get_latex])
}
