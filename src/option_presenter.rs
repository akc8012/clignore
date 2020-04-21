struct OptionPresenter<'o> {
	options: &'o [String],
}

// TODO: Please for the love of god pick a better name
#[derive(Debug, PartialEq)]
enum OptionResult<'o> {
	Some(&'o str),
	Invalid(usize),
	None,
}

#[allow(dead_code)] // TODO: Remove me please
impl<'o> OptionPresenter<'o> {
	pub fn new(options: &'o [String]) -> OptionPresenter<'o> {
		OptionPresenter { options }
	}

	pub fn present_options(&self) -> String {
		let mut list = String::new();

		for (element, option) in self.options.iter().enumerate() {
			list.push_str(&format!("[{}] {}", element + 1, option));
			if element < self.options.len() - 1 {
				list.push('\n');
			}
		}
		list
	}

	pub fn select_option(&self, option: usize) -> OptionResult {
		if option == 0 {
			return OptionResult::None;
		}
		OptionResult::Some(&self.options[option - 1])
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_present_options() {
		let options = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = OptionPresenter::new(&options);

		let list = presenter.present_options();
		assert_eq!(list, "[1] jank.meme\n[2] funky.time");
	}

	#[test]
	fn can_select_option() {
		let options = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = OptionPresenter::new(&options);

		let option = presenter.select_option(2);
		assert_eq!(option, OptionResult::Some("funky.time"));
	}

	#[test]
	fn can_select_none_option() {
		let options = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = OptionPresenter::new(&options);

		let option = presenter.select_option(0);
		assert_eq!(option, OptionResult::None);
	}
}
