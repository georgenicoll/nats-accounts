use tide::Request;
use tide::prelude::*;
use tide::log;

#[macro_use]
extern crate clap;

const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8989;

struct Config {
    port: u16,
    address: String,
}

async fn account(req: Request<()>) -> tide::Result<String> {
    let public_key = req.param("public_key")?;

    let res = format!("You sent {}", public_key);

    Ok(res)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    log::start();

    let config = get_config();
    let address_port = format!("{}:{}", config.address, config.port);
    log::info!("Starting Server on {}", address_port);

    let mut app = tide::new();
    app.at("/jwt/v1/accounts/:public_key").get(account);
    app.listen(address_port).await?;

    Ok(())
}

fn get_config() -> Box<Config> {
    let matches = clap_app!(myapp =>
        (@arg address: -a --address +takes_value "Address of interface on which to listen")
        (@arg port: -p --port +takes_value "Port on which to listen")
    ).get_matches();

    let address = match matches.value_of("address") {
        Some(str) => str,
        None => DEFAULT_ADDRESS,
    };
    let port = match matches.value_of("port") {
        Some(str) => match str.parse::<u16>() {
            Ok(p) => p,
            Err(err) => {
                log::error!("Invalid port supplied '{}' will use default: error={}", str, err);
                DEFAULT_PORT
            }
        }
        None => DEFAULT_PORT,
    };

    Box::new(Config {
        address: String::from(address),
        port: port
    })
}
