use anyhow;
use inquire::Select;

mod commands;

fn main() -> anyhow::Result<()> {
    let contexts = commands::get_contexts()?;
    let current_context = commands::get_current_contexts()?;

    let mut context_index = 0;
    let mut final_contexts: Vec<String> = vec![];

    for (index, context) in contexts.iter().enumerate() {
        if context.trim() == current_context.to_string().trim() {
            context_index = index;
            let format = format!("✅ {}", context);
            final_contexts.push(format);
        } else {
            final_contexts.push(context.to_string())
        }
    }

    // iterate over contexts
    let ans = Select::new("Switch kubernetes context", final_contexts.clone())
        .with_starting_cursor(context_index)
        .with_page_size(final_contexts.len())
        .prompt();
    let choice = ans?;
    // update kubectl context
    commands::update_current_context(&choice)?;
    Ok(())
}
