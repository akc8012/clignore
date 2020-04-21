struct OptionPresenter;

#[allow(dead_code)] // TODO: Remove me please
impl OptionPresenter {
	pub fn new() -> OptionPresenter {
		OptionPresenter
	}

	pub fn present_list(&self, options: &[String]) -> String {
		let mut list = String::new();
		for (element, option) in options.iter().enumerate() {
			list.push_str(&format!("[{}] {}", element + 1, option));

			if element < options.len() - 1 {
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
	fn can_present_list() {
		let presenter = OptionPresenter::new();

		let options = vec![String::from("jank.meme"), String::from("funky.time")];
		let list = presenter.present_list(&options);

		assert_eq!(list, "[1] jank.meme\n[2] funky.time");
	}
}
