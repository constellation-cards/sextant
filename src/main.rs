#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
use rocket::State;

mod templates;

struct ServiceState<'a> {
    registry: Handlebars<'a>,
}

#[post("/latex/<template>", format = "json")]
fn get_latex(service_state: &State<ServiceState>, template: &str) -> String {
    let result = service_state
        .registry
        .render(template, &json!({"name": "foo"}));
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
