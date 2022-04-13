pub fn get_contexts() -> anyhow::Result<Vec<String>> {
    // get kubectl contexts by running command
    let command = std::process::Command::new("kubectl")
        .args(&["config", "view", "-o=jsonpath={.contexts[*].name}"])
        .output()?;
    let output = String::from_utf8_lossy(&command.stdout).to_string();
    Ok(output.split_whitespace().map(|s| s.to_string()).collect())
}

pub fn get_current_contexts() -> anyhow::Result<String> {
    // get kubectl contexts by running command
    let command = std::process::Command::new("kubectl")
        .args(&["config", "current-context"])
        .output()?
        .stdout;
    Ok(String::from_utf8_lossy(&command).to_string())
}

pub fn update_current_context(choice: &String) -> anyhow::Result<()> {
    let command = std::process::Command::new("kubectl")
        .args(&["config", "use-context", choice])
        .output()?;
    println!("{}", String::from_utf8_lossy(&command.stdout));
    Ok(())
}
