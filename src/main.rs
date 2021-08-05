// extern crate vader_sentiment;

// fn main() {
//     vader_sentiment::demo::run_demo();
// }

extern crate spider;

use spider::website::Website;

fn main() {
    let mut website: Website = Website::new("https://reddit.com/r/all");
    website.crawl();

    for page in website.get_pages() {
        println!("- {}", page.get_url());
    }
}   