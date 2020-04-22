pub struct ChoicePresenter<'c> {
	choices: &'c [String],
}

#[derive(Debug, PartialEq)]
pub enum ChoiceResult<'c> {
	Some(&'c str),
	Invalid(usize),
	None,
}

impl<'c> ChoicePresenter<'c> {
	pub fn new(choices: &'c [String]) -> ChoicePresenter<'c> {
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

	pub fn select_choice(&self, input: usize) -> ChoiceResult {
		match input {
			0 => ChoiceResult::None,
			o if o > self.choices.len() => ChoiceResult::Invalid(o),
			_ => ChoiceResult::Some(&self.choices[input - 1]),
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
		let choices = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = ChoicePresenter::new(&choices);

		let list = presenter.present_choices();
		assert_eq!(list, "[1] jank.meme\n[2] funky.time");
	}

	#[test]
	fn can_select_choice() {
		let choices = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = ChoicePresenter::new(&choices);

		let choice = presenter.select_choice(2);
		assert_eq!(choice, ChoiceResult::Some("funky.time"));
	}

	#[test]
	fn can_select_none_choice() {
		let choices = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = ChoicePresenter::new(&choices);

		let choice = presenter.select_choice(0);
		assert_eq!(choice, ChoiceResult::None);
	}

	#[test]
	fn can_select_invalid_choice() {
		let choices = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = ChoicePresenter::new(&choices);

		let choice = presenter.select_choice(3);
		assert_eq!(choice, ChoiceResult::Invalid(3));
	}
}
