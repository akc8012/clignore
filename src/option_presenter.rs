struct OptionPresenter<'o> {
	options: &'o [String],
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

	pub fn select_option(&self, option: usize) -> Option<&str> {
		if option == 0 {
			return None;
		}

		Some(&self.options[option - 1])
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

		let option = presenter.select_option(2).unwrap();
		assert_eq!(option, "funky.time");
	}

	#[test]
	fn can_select_none_option() {
		let options = vec![String::from("jank.meme"), String::from("funky.time")];
		let presenter = OptionPresenter::new(&options);

		let option = presenter.select_option(0);
		assert_eq!(option, None);
	}
}
