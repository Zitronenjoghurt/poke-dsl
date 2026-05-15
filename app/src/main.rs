use poke_dsl::data::dex::Dex;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from("./data");
    let mut dex = Dex::new();
    dex.load_ron_from_dir(&dir).unwrap();
    println!("{:#?}", dex);
}
