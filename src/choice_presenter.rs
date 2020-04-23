use crate::error_box::ErrorBox;

pub struct ChoicePresenter<'c> {
	choices: Vec<&'c String>,
}

pub type ChoiceResult<'c> = Result<Option<&'c String>, ErrorBox>;

impl<'c> ChoicePresenter<'c> {
	pub fn new(choices: Vec<&'c String>) -> ChoicePresenter<'c> {
		ChoicePresenter { choices }
	}

	pub fn present_choices(&self) -> String {
		let mut list = String::new();

		for (element, choice) in self.choices.iter().enumerate() {
			list.push_str(&format!("[{}] {}", element + 1, choice));
			if element < self.choices.len() - 1 {
				list.push('\n');
			}
		}
		list
	}

	pub fn select_choice(&'c self, input: &str) -> ChoiceResult {
		let input = input.trim().parse()?;
		match input {
			0 => Ok(None),
			i if i > self.choices.len() => Err("".into()),
			_ => Ok(Some(&self.choices[input - 1])),
		}
	}

	pub fn len(&self) -> usize {
		self.choices.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_present_choices() {
		let jank = &"jank.meme".to_owned();
		let funky = &"funky.time".to_owned();
		let choices = vec![jank, funky];

		let presenter = ChoicePresenter::new(choices);

		let list = presenter.present_choices();
		assert_eq!(list, "[1] jank.meme\n[2] funky.time");
	}

	#[test]
	fn can_select_choice() {
		let jank = &"jank.meme".to_owned();
		let funky = &"funky.time".to_owned();
		let choices = vec![jank, funky];

		let presenter = ChoicePresenter::new(choices);

		let choice = presenter.select_choice("2").unwrap();
		assert_eq!(choice, Some(&"funky.time".into()));
	}

	#[test]
	fn can_select_untrimmed_choice() {
		let jank = &"jank.meme".to_owned();
		let funky = &"funky.time".to_owned();
		let choices = vec![jank, funky];

		let presenter = ChoicePresenter::new(choices);

		let choice = presenter.select_choice(" 2   \n    ").unwrap();
		assert_eq!(choice, Some(&"funky.time".into()));
	}

	#[test]
	fn can_select_none_choice() {
		let jank = &"jank.meme".to_owned();
		let funky = &"funky.time".to_owned();
		let choices = vec![jank, funky];

		let presenter = ChoicePresenter::new(choices);

		let choice = presenter.select_choice("0").unwrap();
		assert_eq!(choice, None);
	}

	#[test]
	fn can_select_invalid_choice() {
		let jank = &"jank.meme".to_owned();
		let funky = &"funky.time".to_owned();
		let choices = vec![jank, funky];

		let presenter = ChoicePresenter::new(choices);

		let choice_out_of_bounds = presenter.select_choice("3");
		let choice_not_a_number = presenter.select_choice("sasafrass");

		assert!(choice_out_of_bounds.is_err());
		assert!(choice_not_a_number.is_err());
	}
}
