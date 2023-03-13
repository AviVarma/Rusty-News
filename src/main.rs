mod theme;

use std::error::Error;
use dotenv::dotenv;

use newsapi::{Articles, get_articles};

fn render_artcles(articles: &Articles){
    let theme = theme::default();
    theme.print_text("# Top Headlines For the Day\n\n");
    for a in &articles.articles{
        theme.print_text(&format!("` {}`", a.title));
        theme.print_text(&format!("> *{}*", a.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url: &str = 
    "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=";

    let url = format!("{}{}", url, api_key);


    let articles: Articles = get_articles(&url)?;

    render_artcles(&articles);

    Ok(())
}