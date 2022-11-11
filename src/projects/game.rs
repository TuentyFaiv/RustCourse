use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::{fs};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

// Type, Tag, Text, Life
#[derive(Debug)]
struct DataHistory {
	type_data: String,
	tag: String,
	text: String,
	life: i32,
	options: Vec<DataHistory>
}

impl DataHistory {
	fn new(row: StringRecord) -> DataHistory {
		return DataHistory {
			type_data: row.get(0).unwrap().trim().to_string(),
			tag: row.get(1).unwrap().trim().to_string(),
			text: row.get(2).unwrap().trim().to_string(),
			life: row.get(3).unwrap().trim().parse().unwrap_or(0),
			options: vec![]
		};
	}
}

pub fn start() {
  let mut life = 100;
	let mut current_tag = FIRST_TAG.to_string();

	let mut last_record: String = "".to_string();

	let mut history: HashMap<String, DataHistory> = HashMap::new();

	let content = fs::read_to_string(FILENAME).unwrap();
	let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

	for result in rdr.records() {
		let result = result.unwrap();
		let step = DataHistory::new(result);

		if step.type_data == "SITUACION" {
			let record_tag = step.tag.clone();

			history.insert(record_tag.clone(), step);
			last_record = record_tag;
		} else if step.type_data == "OPCION" {
			if let Some(data) = history.get_mut(&last_record) {
				(*data).options.push(step);
			}
		}
	}

	// Game loop
	loop {
		println!("Tienes {} de vida", life);

		if let Some(data) = history.get(&current_tag) {
			// println!("{}", data.life);
			println!("{}", data.text);
			
			for (index, option) in data.options.iter().enumerate() {
				println!("[{}] {}", index, option.text);
			}

			let mut selection = String::new();

			std::io::stdin().read_line(&mut selection).unwrap();

			let selection = selection.trim().parse().unwrap_or(99);

			if let Some(selected_option) = &data.options.get(selection) {
				current_tag = selected_option.tag.clone();
			} else {
				println!("Comando no valido")
			}

			life += data.life;
			println!("");	
		} else {
			println!("You won");
			break;
		}
		
		// If life is <= 0 game over
		if life <= 0 {
			println!("Game over!");
			break;
		}
	}
}