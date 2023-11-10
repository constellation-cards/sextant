use upon::{Engine, Syntax};

pub fn create_templates() -> Engine<'static> {
    let syntax = Syntax::builder().expr("<=", "=>").block("<<", ">>").build();
    let mut engine = Engine::with_syntax(syntax);

    engine
        .add_template("standard", include_str!("standard.txt"))
        .expect("Unable to add standard.txt template");
    engine
        .add_template("tarot", include_str!("tarot.txt"))
        .expect("Unable to add tarot.txt template");

    engine.add_filter("replace", |a: &str, b: &str, c: &str| str::replace(a, b, c));

    engine.add_filter("description", |desc: &str| {
        let mut s = String::with_capacity(desc.len());
        for line in desc.lines() {
            match line.strip_prefix("//") {
                Some(line) => {
                    s.push_str("\\constellationquote{");
                    s.push_str(line);
                    s.push('}');
                }
                None => s.push_str(line),
            }
            s.push('\n');
            s.push('\n');
        }
        s
    });

    engine
}
