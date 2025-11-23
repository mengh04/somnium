use somnium::find_songs;

fn main() {
    let dir_path = "D:/Music";
    dbg!(find_songs(dir_path).unwrap());
}
