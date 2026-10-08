use jiff::Zoned;

fn main() {
	// Calendar service
	let now = Zoned::now();
	let today = now.date();
  let time = now.time();

	println!("Today is: {}", today);
	println!("The current time is: {}", time);
}

struct Calendar {
  date: Zoned,
  time: Zoned,
  title: String,
  fields: Vec<String>,
}

