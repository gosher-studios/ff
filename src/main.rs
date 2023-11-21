use std::process::Command;

fn main() {
    let pattern = std::env::args().nth(1).expect("no command specified");
    let mut filenames: Vec<String> = std::env::args().collect();
    filenames.remove(0);
    filenames.remove(0);
    if pattern == "upload" || pattern == "u" {
        filenames.iter().for_each(|i| {
            let out = Command::new("curl")
                .args(["-T", i, "https://files.colon3.lol"])
                .output()
                .unwrap();
            let meow = format!(
                "https://files.colon3.lol/{}",
                std::str::from_utf8(&out.stdout).unwrap()
            );
            println!("{}", meow);
            Command::new("wl-copy").arg(meow).spawn().unwrap();
        });
    }
}
