pub struct ContentScraper;

impl ContentScraper {
    pub fn bundle_scraped_text(&self, raw_html: &str) -> String {
        // Enforces structural boundaries around raw untrusted data inputs
        format!("<untrusted_data>\n{}\n</untrusted_data>", raw_html)
    }
}
