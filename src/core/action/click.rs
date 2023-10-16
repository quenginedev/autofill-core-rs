use async_trait::async_trait;
use chromiumoxide::error::CdpError;

use super::{Context, Action};

pub struct Click<'a > {
  pub context: &'a Context<'a>,
  pub selector: &'a String,
}


#[async_trait]
impl <'a> Action for Click<'a> {
    async fn execute(&self) -> Result<(), CdpError> {
      let page = self.context.page;
      let selector = self.selector;

      page
        .find_element(selector).await?
        .click().await?;
      Ok(())
    }
}