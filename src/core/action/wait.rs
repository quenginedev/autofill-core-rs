use std::time::Duration;

use async_trait::async_trait;
use chromiumoxide::error::CdpError;

use super::{Action, Context};

pub struct Wait<'a> {
  pub context: &'a Context<'a>,
  pub timeout: &'a Duration,
}

#[async_trait]
impl<'a> Action for Wait<'a> {
    async fn execute(&self) -> Result<(), CdpError> {
      let mut counter = Duration::ZERO;
      let waiting = 100;
      loop {
        if counter == *self.timeout { break; }
        counter += Duration::from_millis(waiting);
        std::thread::sleep(Duration::from_millis(waiting));
      } 
      Ok(())
    }
}