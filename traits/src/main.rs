#![allow(unused)]
use traits::{NewsArticle, Summary, Tweet, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("saroj"),
        content: String::from("This is some content of the tweet out there!"),
        reply: false,
        retweet: false
    };

    println!("Tweet: {}", tweet.summarize());
    println!();

    let news_article = NewsArticle {
        content: String::from("This is the content of a news article."),
        author: String::from("saroj"),
        location: String::from("Rampur"),
        headline: String::from("Nothing happened")
    };

    println!("NewsArticle: {}", news_article.summarize());

    println!();
    notify(&tweet);
}

// Returning type that implements Traits (here a type that implements Summary)
fn return_types_implementing_traits() -> impl Summary {
    NewsArticle {
        content: String::from("Some content"),
        headline: String::from("Heading Unavailable!"),
        location: String::from("Antartica"),
        author: String::from("unknown")
    }
}

// Using impl Trait is only allowed if we are returning a single type
// returning one or other based on a condition like this doesn't compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}