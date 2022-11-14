fn main() {
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




    let change = scraper::Selector::parse(".chakra-text css-dg4gux").unwrap();

    let crypto_change= document.select(&change).map(|x| x.inner_html());


    let volume = scraper::Selector::parse(".css-1nh9lk8").unwrap();

    let crypto_volume= document.select(&volume).map(|x| x.inner_html());


    crypto_curency
        .zip(1..51)
        .for_each(|(item, number)| println!("top {}. crypto curriency name:{}", number, item));



        crypto_price
            .zip(1..51)
            .for_each(|(item, number)| println!("Top {}. crypto curiencies Price:{}", number, item));

            crypto_change
                .zip(1..51)
                .for_each(|(item, number)| println!("Top  {}. crypto curiencies 24hr Change:{}", number, item));
           
    let mut n = 0;
    for element in crypto_volume {
    
        if n%2==0 {
            println!("Top 50 crypto curiencies 24hr volume:{}", element);
        }
        n += 1;
    }
}