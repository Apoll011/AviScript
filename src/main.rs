mod modules;

use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Scope};
use crate::modules::register_modules;
// TODO:
/*
-----------
on_intent "get_weather" |intent| {
    let city = intent.slots.get("city") ?? "your location";
    say("Let me check the weather in " + city + ".");
}
-----------

*/

pub fn main() -> Result<(), Box<EvalAltResult>>
{
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    scope.push_constant("SKILL_NAME", "test")
        .push_constant("CURRENT_LANGUAGE", "pt")
        .push_constant("SUPPORTED_LANGUAGES", [ "pt", "en" ]);

    engine.register_custom_syntax(["whenis", "$expr$", "?", "$expr$", ":", "$expr$"], false, |context, inputs| match context.eval_expression_tree(&inputs[0])?.as_bool() {
            Ok(true) => context.eval_expression_tree(&inputs[1]),
            Ok(false) => context.eval_expression_tree(&inputs[2]),
            Err(typ) => Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                "bool".to_string(), typ.to_string(), inputs[0].position()
            ))),
        }, )?;

    engine.register_custom_operator("or", 160)?
        .register_fn("or", |a: Dynamic, b: Dynamic| {
            if a.to_string() == "" {
                b
            }
            else {
                a
            }
        });
    engine.register_custom_operator("@@", 160)?
        .register_fn("@@", |a: ImmutableString, b: ImmutableString| format!("{}{}", a, b));
    
    register_modules(&mut engine).expect("A module did not load successfully!!");
    
    engine.run_file_with_scope(&mut scope, "script.avi".into())?;

    // Done!
    Ok(())
}