#[macro_use]
extern crate rocket;

use handlebars::Handlebars;
use rocket::State;
use serde_json::Value;

mod templates;

struct ServiceState<'a> {
    registry: Handlebars<'a>,
}

#[post("/latex/<template>", format = "json", data = "<body>")]
fn get_latex(service_state: &State<ServiceState>, template: &str, body: &str) -> String {
    let cards: Value = serde_json::from_str(body).unwrap();

    let result = service_state.registry.render(template, &cards);

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
