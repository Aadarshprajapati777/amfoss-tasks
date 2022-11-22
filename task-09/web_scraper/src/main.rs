
use scraper::{Html, Selector};

fn main() {


    let response = reqwest::blocking::get(
        "https://crypto.com/price",
    )
    .unwrap()
    .text()
    .unwrap();



    let document = scraper::Html::parse_document(&response);


    // let mut wtr = csv::Writer::from_path("data.csv").expect("Could not create file.");
    // wtr.write_record(&["Crypto Name", "Crypto Price", "24hrChange","marketcap","Volume"]).expect("Could not write header.");

    // for element in document.select(&body) {
 
    let crypto = scraper::Selector::parse("p.chakra-text.css-rkws3").unwrap();

    let crypto_curency = document.select(&crypto).map(|x| x.inner_html());

    

    let price = scraper::Selector::parse(".css-0>.css-1cxc880>.css-1vyy4qg>.css-b1ilzc").unwrap();

    let crypto_price= document.select(&price).map(|x| x.inner_html());




    let change = scraper::Selector::parse("td.css-1b7j986>p").unwrap();

    let crypto_change= document.select(&change).map(|x| x.inner_html());


    let marketcap= scraper::Selector::parse(".css-1nh9lk8+.css-1nh9lk8").unwrap();

    let crypto_marketcap= document.select(&marketcap).map(|x| x.inner_html());
    

    
    let volume = scraper::Selector::parse("td:nth-child(6)").unwrap();
    let crypto_volume= document.select(&volume).map(|x| x.inner_html());

//     wtr.write_record([crypto_curency, &crypto_price, &crypto_change, &crypto_marketcap, &crypto_volume]).expect("Could not create selector.");
// // }
// wtr.flush().expect("Could not close file");
// println!("Done");


//  crypto_curency
// .zip(1..51)
// .zip(crypto_price)
// .zip(crypto_change)
// .zip(crypto_volume)
// .zip(crypto_marketcap)
// .for_each(|(((((crypto_curency, _), crypto_price), crypto_change),crypto_volume ),crypto_marketcap )| {
//     println!(
//         "{}, {}, {}, {}, {}",
//         crypto_curency, crypto_price, crypto_change, crypto_volume, crypto_marketcap
//     );
// });

println('{}',  crypto_curency
.zip(1..51)
.zip(crypto_price)
.zip(crypto_change)
.zip(crypto_volume)
.zip(crypto_marketcap).len());

}
// .for_each(|(item, number)| println!("top {}. crypto curriency name:{}", number, item));

// crypto_volume
// .zip(1..51)
// .for_each(|(item, number)| println!("top {}. crypto volume:{}", number, item));

// crypto_price
// .zip(1..51)
// .for_each(|(item, number)| println!("Top {}. crypto curiencies Price:{}", number, item));

// crypto_change
// .zip(1..51)
// .for_each(|(item, number)| println!("Top  {}. crypto curiencies 24hr Change:{}", number, item));
           
// crypto_marketcap
// .zip(1..51)
// .for_each(|(item, number)| println!("Top  {}. crypto market cap:{}", number, item));



