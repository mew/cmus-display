use std::process::Command;

fn main() {
    let cmus_remote = String::from_utf8(Command::new("cmus-remote").arg("-Q").output().unwrap().stdout).unwrap();
    let x: Vec<&str> = cmus_remote.split("\n").collect();
    let mut artist = "";
    let mut track = "";
    for i in 0..x.len() {
        let a = x[i];
        if a.starts_with("tag ") {
            if a.starts_with("tag artist") {
                artist = a.split("artist ").collect::<Vec<&str>>()[1];
            } else if a.starts_with("tag title") {
                track = a.split("title ").collect::<Vec<&str>>()[1];
            }
        }
    }

    println!("{} - {}", artist, track)
}
