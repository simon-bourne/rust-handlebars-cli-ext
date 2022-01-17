use std::{
    error::Error,
    fs,
    io::{self, Read},
    process::Output,
};

use execute::shell;
use handlebars::{handlebars_helper, Handlebars, RenderError};

handlebars_helper!(include: |file: str| { fs::read_to_string(file)? });
handlebars_helper!(command: |cmd: str| { run_process(cmd)? });

fn run_process(cmd: &str) -> Result<String, RenderError> {
    let mut shell_cmd = shell(cmd);

    let Output {
        status,
        stdout,
        stderr,
    } = shell_cmd.output()?;

    let output = String::from_utf8(stdout)?;

    if !stderr.is_empty() {
        return Err(RenderError::new(format!(
            "Stderr is not empty:\n\n{}",
            String::from_utf8(stderr)?
        )));
    }

    if !status.success() {
        return Err(RenderError::new("Process failed"));
    }

    Ok(output)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();
    reg.register_helper("include", Box::new(include));
    reg.register_helper("command", Box::new(command));

    let mut template = String::new();
    io::stdin().read_to_string(&mut template)?;

    reg.render_template_to_write(&template, &"{}", io::stdout())?;

    Ok(())
}