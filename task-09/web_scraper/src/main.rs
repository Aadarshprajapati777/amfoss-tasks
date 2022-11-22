
use scraper::{Html, Selector};

fn main() {


    let response = reqwest::blocking::get(
        "https://crypto.com/price",
    )
    .unwrap()
    .text()
    .unwrap();



    let document = scraper::Html::parse_document(&response);

    let rowselector = Selector::parse(".css-1cxc880").unwrap();

    let mut wtr = csv::Writer::from_path("data.csv").expect("Could not create file.");

    wtr.write_record(&["Crypto Name", "Crypto Price", "24hrChange","marketcap","Volume"]).expect("Could not write header.");

    let crypto = scraper::Selector::parse("p.chakra-text.css-rkws3").unwrap();
    let price = scraper::Selector::parse(".css-0>.css-1cxc880>.css-1vyy4qg>.css-b1ilzc").unwrap();
    let change = scraper::Selector::parse("td.css-1b7j986>p").unwrap();
    let marketcap= scraper::Selector::parse(".css-1nh9lk8+.css-1nh9lk8").unwrap();
    let volume = scraper::Selector::parse("td.css-1nh9lk8").unwrap();

    for element in document.select(&rowselector) {
 

    let cryptoName = element.select(&crypto).next().unwrap().inner_html();
    let cryptoPrice = element.select(&price).next().unwrap().inner_html();
    let cryptoChange = element.select(&change).next().unwrap().inner_html();
    let cryptoVolume = element.select(&volume).next().unwrap().inner_html();
    let cryptoMarketcap = element.select(&marketcap).next().unwrap().inner_html();

    wtr.write_record(&[cryptoName, cryptoPrice, cryptoChange,cryptoMarketcap,cryptoVolume]).expect("Could not write record.");
    
}
wtr.flush().expect("Could not close file");
println!("Done");

}
