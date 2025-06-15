
mod definition;
mod utils;

fn main() {

    let parameters = match utils::args::parse_args() {
        Ok(v) => v,
        Err(e) => {
            utils::show::show_error(&e);
            return;
        }
    };

    if parameters.contains_key("help") {
        utils::usage::usage();
        return;
    }
}
