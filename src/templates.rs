use handlebars::Handlebars;

pub fn create_templates() -> Handlebars<'static> {
    let mut reg = Handlebars::new();

    reg.register_template_string("standard", "Good afternoon, {{name}}")
        .expect("Unable to register standard template");

    reg
}
