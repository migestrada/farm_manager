pub struct Farm {
  pub current_sotarge: i32,
}

impl Farm {
  pub fn show_info(&self) {
    println!("\nActual Chickens: {}", self.current_sotarge)
  }
  pub fn add_ckicken(&mut self) {
    self.current_sotarge += 1;
  }

  pub fn sub_ckicken(&mut self) {
    if self.current_sotarge > 0 {
      self.current_sotarge -= 1;
    }
  }
}