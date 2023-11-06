use upon::{Engine, Syntax};

pub fn create_templates() -> Engine<'static> {
    let syntax = Syntax::builder().expr("<=", "=>").block("<<", ">>").build();
    let mut engine = Engine::with_syntax(syntax);

    engine
        .add_template("standard", include_str!("standard.txt"))
        .expect("Unable to add standard.txt template");

    engine.add_filter("replace", |a: &str, b: &str, c: &str| str::replace(a, b, c));

    engine
}
