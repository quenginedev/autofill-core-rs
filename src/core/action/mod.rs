mod click;
mod wait;

use async_std::task::spawn;
use async_trait::async_trait;
use chromiumoxide::{
    cdp::browser_protocol::page::EventFrameNavigated, error::CdpError, types::Error, Browser,
    BrowserConfig, Page,
};
use futures::StreamExt;
use regex::Regex;

use super::instruction::{EntryPoint, Instruction};

use click::Click;
use wait::Wait;

pub struct Context<'a> {
    page: &'a Page,
}

#[async_trait]
pub trait Action {
    async fn execute(&self) -> Result<(), CdpError>;
}

pub async fn autofill(entires: Vec<EntryPoint>) -> Result<(), Box<dyn std::error::Error>> {
    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    for entry in entires {
        let page = browser.new_page(&entry.url).await?;
        spawn(async move {
            let context = Context { page: &page };
            match page.event_listener::<EventFrameNavigated>().await {
                Ok(mut events) => {
                    while let Some(event) = events.next().await {
                        let url: &String = &event.frame.url;
                        let frames = &entry.page_frames;
                        for frame in frames.iter() {
                            if let Ok(matcher) = Regex::new(&frame.url_matcher) {
                                if matcher.is_match(url) {
                                    let instructions = &frame.instructions;
                                    _run_instructions(&context, instructions).await;
                                }
                            };
                        }
                    }
                }
                _ => todo!(),
            };
        });
    }

    handle.await;
    Ok(())
}

async fn _run_instructions(context: &Context<'_>, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        let result = match instruction {
            Instruction::Click { selector, .. } => Click { context, selector }.execute().await,
            Instruction::Wait { timeout } => Wait { context, timeout }.execute().await,
        };

        if let Err(error) = result {
            match error {
                CdpError::Chrome(Error { message, code }) => println!("{code} - {message}"),
                _ => todo!(),
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::core::instruction::{EntryPoint, Frame, Instruction, ENTRY_POINT};

    use super::autofill;

    #[async_std::test]
    async fn test_autofill() {
        let entries = vec![EntryPoint {
            url: "https://www.bing.com".to_string(),
            page_frames: vec![
                Frame {
                    url_matcher: String::from(ENTRY_POINT),
                    instructions: vec![Instruction::Click {
                        selector: "#codex > a".to_string(),
                    }],
                },
                Frame {
                    url_matcher: "^https://www\\.bing\\.com/search\\?q=".to_string(),
                    instructions: vec![
                        Instruction::Wait {
                            timeout: Duration::from_secs(5),
                        },
                        Instruction::Click {
                            selector: "#b_sydConvCont > cib-serp div > div:nth-child(5)"
                                .to_string(),
                        },
                    ],
                },
            ],
        }];
        let res = autofill(entries).await;
        if let Err(err) = res {
            println!("{err:?}");
            assert!(false);
        } else {
            assert!(true);
        };
    }
}
