use eyre::Result;

use crate::signature::structure::SignTransferTrait;

mod definition;
mod utils;
mod signature;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let parameters = match utils::args::parse_args() {
        Ok(v) => v,
        Err(e) => {
            utils::show::show_error(&e);
            return Ok(());
        }
    };

    if parameters.contains_key("help") {
        utils::usage::usage();
        return Ok(());
    }

    let signature_struct = match signature::structure::SignatureStruct::from(&parameters) {
        Ok(v) => v,
        Err(e) => {
            utils::show::show_error(&e);
            return Ok(());
        }
    };

    let tx_hex = signature_struct.sign_transfer().await?;
    println!("{}", tx_hex);

    Ok(())
}
