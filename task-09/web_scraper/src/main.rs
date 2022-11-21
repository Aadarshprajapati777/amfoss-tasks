use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {

    let mut file = fs::File::create("Top50Cryptodata.txt")?;
    file.write_all(b"{}")?;
    Ok(())
    let response = reqwest::blocking::get(
        "https://crypto.com/price",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
 
    let crypto = scraper::Selector::parse("p.chakra-text.css-rkws3").unwrap();

    let crypto_curency = document.select(&crypto).map(|x| x.inner_html());

    

    let price = scraper::Selector::parse(".css-0>.css-1cxc880>.css-1vyy4qg>.css-b1ilzc").unwrap();

    let crypto_price= document.select(&price).map(|x| x.inner_html());




    let change = scraper::Selector::parse("td.css-1b7j986>p").unwrap();

    let crypto_change= document.select(&change).map(|x| x.inner_html());


    let marketcap= scraper::Selector::parse(".css-1nh9lk8~.css-1nh9lk8").unwrap();

    let crypto_marketcap= document.select(&marketcap).map(|x| x.inner_html());
    

    
    let volume = scraper::Selector::parse("td.css-1b7j986+.css-1nh9lk8").unwrap();
    let crypto_volume= document.select(&volume).map(|x| x.inner_html());


 crypto_curency
.zip(1..51)
.for_each(|(item, number)| println!("top {}. crypto curriency name:{}", number, item));

crypto_volume
.zip(1..51)
.for_each(|(item, number)| println!("top {}. crypto volume:{}", number, item));

crypto_price
.zip(1..51)
.for_each(|(item, number)| println!("Top {}. crypto curiencies Price:{}", number, item));

crypto_change
.zip(1..51)
.for_each(|(item, number)| println!("Top  {}. crypto curiencies 24hr Change:{}", number, item));
           
crypto_marketcap
.zip(1..51)
.for_each(|(item, number)| println!("Top  {}. crypto market cap:{}", number, item));
 

//print crypto_curency,crypto_price,crypto_change,cryp
}
