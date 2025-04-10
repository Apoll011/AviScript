use std::error::Error;
use rhai::{Array, Scope};
use crate::engine::create_avi_script_engine;
use crate::modules::register_modules;

mod modules;
mod engine;



pub fn main() -> Result<(), Box<dyn Error>>
{
    let mut scope = Scope::new();

    let supported_languages: Array = vec!["pt".into(), "en".into()];

    scope.push_constant("SKILL_NAME", "test")
        .push_constant("CURRENT_LANGUAGE", "pt")
        .push_constant("SUPPORTED_LANGUAGES", supported_languages);

    let mut engine = create_avi_script_engine(register_modules);

    engine?.run_file_with_scope(&mut scope, "script.avi".into())?;

    // Done!
    Ok(())
}