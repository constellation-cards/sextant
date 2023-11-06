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
    let cards: Value = serde_json::from_str(body).unwrap();

    let result = service_state
        .registry
        .template(template)
        .render(&cards)
        .to_string();

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
