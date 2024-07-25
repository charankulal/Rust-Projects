use error_chain::error_chain;
error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let response=reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}\n",response.status());
    println!("Headers:\n{:#?}\n",response.headers());
    let body=response.text().await?;
    println!("Body: \n{}",body);
    Ok(())
}