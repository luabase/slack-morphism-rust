use slack_morphism_client as slack_client;
use slack_morphism_models as slack_models;

use slack_client::*;
use slack_models::blocks::kit::*;
use slack_models::*;
use slack_morphism_client::test::{SlackApiTestResponse, SlackApiTestRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sb: SlackSectionBlock = SlackSectionBlock::new().with_block_id("test".into());
    let sb_ser = serde_json::to_string_pretty(&sb).unwrap();
    let sb_des: SlackSectionBlock = serde_json::from_str(&sb_ser).unwrap();
    println!("{} {:?}", sb_ser, sb_des);

    let section_block = SlackSectionBlock::new()
        .with_text(md!("hey, {}", 10))
        .with_fields(slack_items! [
            some(md!("hey1")),
            some(pt!("hey2")),
            optionally( sb_ser.is_empty() => md!("hey"))
        ])
        .with_accessory(
            SlackBlockButtonElement::from(SlackBlockButtonElementInit {
                action_id: "-".into(),
                text: pt!("ddd"),
            })
            .into(),
        );

    let context_block: SlackContextBlock = SlackContextBlock::new(slack_blocks![
        some(SlackBlockImageElement::new(
            "http://example.net/img1".into(),
            "text 1".into()
        )),
        some(SlackBlockImageElement::new(
            "http://example.net/img2".into(),
            "text 2".into()
        ))
    ]);

    let blocks: Vec<SlackBlock> = slack_blocks! [
       some ( section_block ),
       optionally( !sb_ser.is_empty() => context_block)
    ];

    println!("{:#?}", blocks);

    let client = SlackClient::new();
    let token_value : String = std::env::var("SLACK_TEST_TOKEN")?;
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);
    println!("{:#?}", session);

    let test: SlackApiTestResponse =
        session
            .api_test(
                &SlackApiTestRequest::new()
                    .with_foo("Test".into())
            ).await?;

    println!("{:#?}", test);

    Ok(())
}