use std::error::Error;
use colour::{dark_green, yellow};
use newsapi::{Articles, get_articles};

fn render_artcles(articles: &Articles){
    for a in &articles.articles{
        dark_green!("> {}\n", a.title);
        yellow!("> {}\n\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let url: &str = "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=025d8c08ac5b4b6a910618e1b1575584";
    let articles: Articles = get_articles(url)?;

    render_artcles(&articles);

    Ok(())
}