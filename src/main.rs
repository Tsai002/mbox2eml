use std::{fs, path::Path};

#[derive(Debug, serde::Deserialize)]
struct Config {
    mbox_path: String,
    save_dir: String,
}

fn main() {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    let config: Config = {
        let config_str = fs::read_to_string(config_path).unwrap();
        basic_toml::from_str(&config_str).unwrap()
    };

    let time = std::time::Instant::now();

    let mut count = 0;
    for i in walkdir::WalkDir::new(&config.mbox_path) {
        let entry = i.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "dat" || ext == "msf" {
                    continue;
                }
            }

            if path.metadata().unwrap().len() == 0 {
                continue;
            }

            let mbox = mbox_reader::MboxFile::from_file(path).unwrap();
            let new_path = path.strip_prefix(&config.mbox_path).unwrap();
            let path_str = new_path.to_str().unwrap().replace(".sbd", "");
            println!("{}, {}ms", path_str, time.elapsed().as_millis());

            let save_path = Path::new(&config.save_dir).join(path_str);
            if !save_path.exists() {
                fs::create_dir_all(&save_path).unwrap();
            }
            for (i, e) in mbox.iter().enumerate() {
                count += 1;
                let msg = e.message().unwrap();
                let msg = String::from_utf8_lossy(msg);
                let file_path = save_path.join(format!("{}.eml", i));
                println!("  total: {count} {}", file_path.display());
                fs::write(file_path, msg.as_bytes()).unwrap();
            }
        }
    }

    println!("Finished. Press to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
