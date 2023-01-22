use inquire::{
  list_option::ListOption, validator::Validation, MultiSelect,
};

mod config;

fn main() {
    let options = vec![
        "OS",
        "Git",
        "VSCode",
        "Shell",
    ];

    let validator = |a: &[ListOption<&&str>]| {
        if a.len() < 1 {
          return Ok(Validation::Invalid("You must select at least one option.".into()));
        }

        return Ok(Validation::Valid);
    };

    let ans = MultiSelect::new("Select what you would like to configure:", options)
        .with_validator(validator)
        .prompt();


    match ans {
        Ok(answers) => {
            let entries = answers.iter();

            for entry in entries {
                match entry.as_ref() {
                    "Git" => config::git::configure(),
                    "VSCode" => config::vscode::configure(),
                    "OS" => config::os::configure(),
                    "Shell" => config::shell::configure(),
                    _ => ()
                }
            }
        }
        Err(_) => println!("Something went wrong buddy."),
    }
}