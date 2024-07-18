use std::env;
use std::fs::{create_dir_all, metadata, write, File};
use std::io::Read;
use std::path::Path;
use wthor;

fn pos_to_str(p: &wthor::Position) -> String {
    let a0: u8 = 97;
    let n0: u8 = 49;
    format!(
        "{}{}",
        (a0 + p.file as u8) as char,
        (n0 + p.rank as u8) as char
    )
}

fn game_to_str(g: &wthor::Game) -> String {
    let ms = g.moves
        .iter()
        .map(|m| pos_to_str(m))
        .collect::<Vec<String>>()
        .join(" ");
    format!("{} {}", ms, g.real_score)
}

fn main() -> Result<(), wthor::WthorError> {
    let args: Vec<String> = env::args().collect();
    let f = &args[1];
    // let f = "WTH_1977";
    create_dir_all("../data").expect("cannot create directory");
    let wtb_filename = format!("./wthor-database/WTH_{f}.wtb");
    let wtb_path = Path::new(&wtb_filename);
    print!("Exporting {}...", wtb_filename);
    let mut wtb_file = File::open(&wtb_path).expect("no file found");
    let wtb_metadata = metadata(&wtb_path).expect("unable to read metadata");
    let mut wtb_buffer = vec![0; wtb_metadata.len() as usize];
    wtb_file.read(&mut wtb_buffer).expect("buffer overflow");
    let games = wthor::parse(&wtb_buffer)?
        .games
        .expect("Unexpected parsing error");

    let txt = games.iter().map(game_to_str).collect::<Vec<String>>();
    let txt_filename = format!("./data/WTH_{f}.txt");
    let path = Path::new(&txt_filename);
    write(path, txt.join("\n")).expect("unable to write file");
    Ok(())
}
