#![allow(unused)]

use generic_traits_lifetime::NewsArticle;
use generic_traits_lifetime::Summary;
use generic_traits_lifetime::Tweet;
use generic_traits_lifetime::notify;
use generic_traits_lifetime::largest;
use generic_traits_lifetime::Pair;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());    

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
    notify(tweet);
    notify(article);


    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let pair = Pair{x: 5, y: 10};
    pair.cmp_display();

    
    
}
