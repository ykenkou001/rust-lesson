trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;
// fn priceの中身を方の中で定義する
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
// fn priceの中身を方の中で定義する
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

// trait Summaryを実装する。返り値はString
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

// 構造体NewsArticleを定義する
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// NewsArticleにSummaryを実装する
impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
}
impl Message for NewsArticle {}

// 構造体Tweetを定義する
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// TweetにSummaryを実装する
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
    // tweetのインスタンスを作成する
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // articleのインスタンスを作成する
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify(&tweet);
    notify_another(&article);
}

// priceを取得する関数
fn get_price<T: Fruits>(fruits: T) {
    println!("price: {}", fruits.price());
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());    
}
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message: {}", item.message());
}