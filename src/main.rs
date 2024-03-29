mod theme;

use std::error::Error;
use dotenv::dotenv;
use newsapi::{Country, Endpoint, NewsAPI, Article};
use tokio;

// use newsapi::{Articles, get_articles};

fn render_artcles(articles: &Vec<Article>){
    let theme = theme::default();
    theme.print_text("# Top Headlines For the Day\n\n");
    for a in articles{
        theme.print_text(&format!("` {}`", a.title()));
        theme.print_text(&format!("> *{}*", a.url()));
        theme.print_text("---");
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv();

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);

    let newsapi_response = newsapi.fetch_async().await?;

    render_artcles(&newsapi_response.articles());

    Ok(())
}
