// extern crate aggregator;

// use aggregator::Summarizable;

pub trait Summarizable {
    fn summary(&self) -> String;
}



struct WeatherForecast {
    high_temp : f64,
    low_temp : f64,
    chance_of_precipitation : f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%", self.high_temp, self.low_temp, self.chance_of_precipitation)
    }
}

pub struct NewsArticle {
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    } 
}


// let tweet = Tweet {
//     username : String::from("horse_ebooks"),
//     content : String::from("of course, as you probably already know, people"),
//     reply : false,
//     retweet : false,
// };

// println!("1 new tweet: {}", tweet.summary());