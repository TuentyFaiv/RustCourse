fn what_is_array() {
  let mut names: Vec<String> = Vec::new();

	for i in 0..15 {
		// Choose a name
		println!("Please enter a name: ");
		let mut name = String::new();
		std::io::stdin().read_line(&mut name).unwrap();
		names.push(name.trim().to_string());

		// Stop iteration
		println!("Do you want to add other name? (Y) yes / (N) no");
		let mut confirm = String::new();
		std::io::stdin().read_line(&mut confirm).unwrap();

		if confirm.trim().to_lowercase() != "y" {
			break;
		}
	}

	println!("{:?}", names);

	for name in names {
		println!("Hello, {}", name);
	}
}