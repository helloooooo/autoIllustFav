
extern crate dotenv;
extern crate egg_mode;
extern crate tokio_core;
extern crate futures;
extern crate chrono;
extern crate twitter_stream;



use dotenv::dotenv;
use std::env;
use tokio_core::reactor::Core;
use std::time::{Duration, Instant};
use std::thread::sleep;
use twitter_stream::{Token, TwitterStream};
use twitter_stream::message::*;
use futures::{Future, Stream};


fn main() {
    dotenv().ok();

    let token = Token::new(
        env::var("CONSKey").unwrap(),
        env::var("CONSSECRET").unwrap(),
        env::var("ACTOKEN").unwrap(),
        env::var("ACTOKENSECRET").unwrap(),
    );
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    println!("");
    println!("Loading the user's home timeline:");
    let future = TwitterStream::user(&token, &handle)
        .flatten_stream()
        .for_each(|json| {
            if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
                print_tweet(&tweet);
                println!("");
            }
            Ok(())
        });

    core.run(future).unwrap();
}

fn print_tweet(tweet: &twitter_stream::message::Tweet) {
    println!(
        "{} (@{}) posted at {}",
        tweet.user.name,
        tweet.user.screen_name,
        tweet.created_at.with_timezone(&chrono::Local)
    );

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        println!("--> in reply to @{}", screen_name);
    }

    if let Some(ref status) = tweet.retweeted_status {
        println!("Retweeted from {}:", status.user.name);
        print_tweet(status);
        return;
    } else {
        println!("{}", tweet.text);
    }

    if let Some(ref place) = tweet.place {
        println!("--from {}", place.full_name);
    }

    if let Some(ref status) = tweet.quoted_status {
        println!("--Quoting the following status:");
        print_tweet(status);
    }

    if !tweet.entities.hashtags.is_empty() {
        println!("Hashtags contained in the tweet:");
        for tag in &tweet.entities.hashtags {
            println!("{}", tag.text);
        }
    }

    if !tweet.entities.symbols.is_empty() {
        println!("Symbols contained in the tweet:");
        for tag in &tweet.entities.symbols {
            println!("{}", tag.text);
        }
    }

    if !tweet.entities.urls.is_empty() {
        println!("URLs contained in the tweet:");
        for url in &tweet.entities.urls {
            println!("{}", url.expanded_url);
        }
    }

    if !tweet.entities.user_mentions.is_empty() {
        println!("Users mentioned in the tweet:");
        for user in &tweet.entities.user_mentions {
            println!("{}", user.screen_name);
        }
    }

    if let Some(ref media) = tweet.entities.media {
        println!("Media attached to the tweet:");
        for info in media {
            println!("A {:?}", info.media_url);
        }
    }
}
