use clap::{App, Arg};
use csv::Writer;
use log::{debug, error, info};
use log4rs;
use std::collections::HashMap;

mod cmc;
mod error;
mod email;
mod etfs;

use cmc::CMCResponse;
use error::OneError;
use etfs::ETFS;
use email::{ Email, HTML };

// dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
// vscode 列编辑，鼠标放到行首，alt + shift
#[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
async fn main() -> Result<(), OneError> {
    dotenv::dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let matches = App::new("onetutorial-rust")
        .version("1.0")
        .author("wangtao")
        .about("Learn rust in one project")
        .arg(
            Arg::new("currency_list")
                .long("currencies")
                .min_values(1)
                .required(true),
        )
        .get_matches();

    let currency_list = matches
        .value_of("currency_list")
        .expect("No currencies were being passed");
    debug!("Querying the following currencies: {:?}", currency_list);

    let cmc_pro_api_key = dotenv::var("CMC_PRO_API_KEY").expect("CMC key not set");
    if cmc_pro_api_key.is_empty() {
        error!("Empty CMC API KEY provided! Please set one via the .env file!");
        return Err(OneError::NoAPIKey);
    }

    let mut params = HashMap::new();
    params.insert("symbol", currency_list.to_string());

    let client = reqwest::Client::new();
    let _resp = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", cmc_pro_api_key)
        .query(&params)
        .send();

    let resp = "{\"status\":{\"timestamp\":\"2023-01-10T13:05:55.475Z\",\"error_code\":0,\"error_message\":null,\"elapsed\":32,\"credit_count\":1,\"notice\":null},\"data\":{\"BTC\":{\"id\":1,\"name\":\"Bitcoin\",\"symbol\":\"BTC\",\"slug\":\"bitcoin\",\"num_market_pairs\":9924,\"date_added\":\"2013-04-28T00:00:00.000Z\",\"tags\":[\"mineable\",\"pow\",\"sha-256\",\"store-of-value\",\"state-channel\",\"coinbase-ventures-portfolio\",\"three-arrows-capital-portfolio\",\"polychain-capital-portfolio\",\"binance-labs-portfolio\",\"blockchain-capital-portfolio\",\"boostvc-portfolio\",\"cms-holdings-portfolio\",\"dcg-portfolio\",\"dragonfly-capital-portfolio\",\"electric-capital-portfolio\",\"fabric-ventures-portfolio\",\"framework-ventures-portfolio\",\"galaxy-digital-portfolio\",\"huobi-capital-portfolio\",\"alameda-research-portfolio\",\"a16z-portfolio\",\"1confirmation-portfolio\",\"winklevoss-capital-portfolio\",\"usv-portfolio\",\"placeholder-ventures-portfolio\",\"pantera-capital-portfolio\",\"multicoin-capital-portfolio\",\"paradigm-portfolio\"],\"max_supply\":21000000,\"circulating_supply\":19257925,\"total_supply\":19257925,\"is_active\":1,\"platform\":null,\"cmc_rank\":1,\"is_fiat\":0,\"self_reported_circulating_supply\":null,\"self_reported_market_cap\":null,\"tvl_ratio\":null,\"last_updated\":\"2023-01-10T13:04:00.000Z\",\"quote\":{\"USD\":{\"price\":17234.697542410468,\"volume_24h\":15792788540.058117,\"volume_change_24h\":4.0069,\"percent_change_1h\":-0.08779297,\"percent_change_24h\":-0.10579356,\"percent_change_7d\":3.06964306,\"percent_change_30d\":0.38884383,\"percent_change_60d\":-1.09497017,\"percent_change_90d\":-9.63506852,\"market_cap\":331904512669.4251,\"market_cap_dominance\":39.0686,\"fully_diluted_market_cap\":361928648390.62,\"tvl\":null,\"last_updated\":\"2023-01-10T13:04:00.000Z\"}}}}}";
    let currencies: CMCResponse = serde_json::from_str(resp).unwrap();

    // if let Some(bitcoin) = currencies.get_currency("BTC") {
    //     println!("{}", bitcoin);
    // } else {
    //     println!("Bitcoin is not in the list");
    // }

    let mut csv = Writer::from_path("prices.csv")?;
    csv.write_record(&["Name", "Symbol", "Price", "7DayChange"])?;

    for (symbol, currency) in currencies.data.into_iter() {
        csv.write_record(&[
            currency.name,
            symbol.to_owned(),
            currency.quote.0.get("USD").unwrap().price.to_string(),
            currency
                .quote
                .0
                .get("USD")
                .unwrap()
                .percent_change_7d
                .to_string(),
        ])?;
    }
    csv.flush()?;
    info!("Queried {} and wrote CSV file", currency_list);

    let etfs = ETFS::new(vec![String::from("3.14")]);
    let mut components : Vec<&dyn HTML>= Vec::new();

    components.push(&etfs);

    let email = Email::new(components);

    match email.send() {
        Ok(_) => info!("{} E-Mail sent", chrono::offset::Utc::now()),
        Err(e) => error!("Error sending E-Mail {}", e)
    }

    Ok(())
}
