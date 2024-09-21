pub struct Farm {
  pub current_storage: i32,
}

impl Farm {
  pub fn show_info(&self) {
    println!("\nActual Chickens: {}", self.current_storage)
  }
  pub fn add_ckicken(&mut self) {
    self.current_storage += 1;
  }

  pub fn sub_ckicken(&mut self) {
    if self.current_storage > 0 {
      self.current_storage -= 1;
    }
  }
}