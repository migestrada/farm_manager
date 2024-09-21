pub fn show_menu() {

  let list: [&str; 3] = [
    "1. Add Chicken",
    "2. Subtract Chicken",
    "3. Exit"
  ];


  for text in list {
    println!("{text}");
    println!("\n");
  }
}