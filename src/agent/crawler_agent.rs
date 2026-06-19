use crate::agent::{AutonomousAgent, Message};
use crate::agent::crawler::ContentScraper;
use async_trait::async_trait;

pub struct SocialMediaCrawlerAgent;

#[async_trait]
impl AutonomousAgent for SocialMediaCrawlerAgent {
    fn name(&self) -> &str { "Social Media Crawler" }
    fn role_permissions(&self) -> Vec<String> { vec!["scrape_external_feeds".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let raw_feed_data = msg.payload.get("raw_text").and_then(|v| v.as_str()).unwrap_or("");
        
        // Clean and bundle external input strings safely inside untrusted tags
        let scraper_tool = ContentScraper;
        let safe_bundle = scraper_tool.bundle_scraped_text(raw_feed_data);

        Ok(Some(Message {
            id: format!("{}-parsed", msg.id),
            sender: self.name().to_string(),
            recipient: "Sentiment_Analyst".to_string(),
            payload: serde_json::json!({ "processed_content": safe_bundle }),
        }))
    }
}
