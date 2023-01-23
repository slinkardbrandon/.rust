use inquire::{
  list_option::ListOption, validator::Validation, MultiSelect,
};

use crate::config;

pub fn exec() {
	let options = vec![
		"Git",
		"NeoVim",
		"OS",
		"Shell",
		"VSCode",
	];

	let validator = |a: &[ListOption<&&str>]| {
			if a.len() < 1 {
				return Ok(Validation::Invalid("You must select at least one option.".into()));
			}

			return Ok(Validation::Valid);
	};

	let ans = MultiSelect::new("Select what you would like to configure:", options)
			.with_validator(validator)
			.with_default(&[0,1,2,3,4])
			.prompt();


	match ans {
			Ok(answers) => {
					let entries = answers.iter();

					for entry in entries {
							match entry.as_ref() {
									"Git" => config::git::configure(),
									"NeoVim" => config::nvim::configure(),
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