
mod utils;

fn main() {
    utils::usage::usage();

    let str = String::from("Invalid to Address");
    utils::show::show_error(&str);
}
