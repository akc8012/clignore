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

	// #[test]
	// fn can_select_option() {}
}
