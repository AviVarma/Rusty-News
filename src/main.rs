use std::error::Error;

struct Articles {
    articles: Vec<Article>
}

struct Article{
    title: String,
    url: String
    // implement stuff here...
}


fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    let response = ureq::get(url).call()?.into_string()?;

    dbg!(response);

    todo!()
}

fn main(){
    let url: &str = "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=025d8c08ac5b4b6a910618e1b1575584";
    let articles: Result<Articles, Box<dyn Error>> = get_articles(url);
}