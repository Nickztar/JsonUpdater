use std::fmt::format;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let start_path = "models/";
    let models = fs::read_dir(start_path)?;
    for file in models.into_iter().filter(|c| {
        c.as_ref().is_ok_and(|n| {
            n.file_name()
                .to_str()
                .is_some_and(|df| !df.starts_with("new_"))
        })
    }) {
        let file = file?;
        let file_name = file.file_name();
        let model = File::open(file.path())?;
        let reader = BufReader::new(model);
        let mut new_file =
            File::create(format!("{}{}", start_path, "new_") + file_name.to_str().unwrap())?;
        let mut prev_line = None;
        for line in reader.lines() {
            let line = line?;
            if !line.contains("get;") && !line.starts_with("//") && !line.contains("JsonProperty") {
                new_file.write(line.as_bytes())?;
                new_file.write(&[b'\n'])?;
                prev_line = Some(line);
                continue;
            }

            if let Some(identifier) = line.split_whitespace().nth(2) {
                let new_line =
                    line.replace(identifier, make_ascii_titlecase(&mut identifier.to_owned()));
                println!("new line? {}, ident: {}", new_line, identifier);
                println!("prev line? {:?}", prev_line);
                if prev_line
                    .clone()
                    .and_then(|prev| {
                        if prev.contains("JsonProperty") {
                            Some(prev)
                        } else {
                            None
                        }
                    })
                    .is_none()
                {
                    let mut whitespace = 0;
                    let chars = line.chars();
                    for char in chars {
                        if char::is_whitespace(char) {
                            whitespace += 1
                        } else {
                            break;
                        }
                    }
                    new_file.write(
                        format!(
                            "{:indent$}[JsonProperty(\"{}\")]",
                            "",
                            identifier,
                            indent = whitespace,
                        )
                        .as_bytes(),
                    )?;
                    new_file.write(&[b'\n'])?;
                }
                new_file.write(new_line.as_bytes())?;
                new_file.write(&[b'\n'])?;
            }
        }
        new_file.flush()?;
    }

    Ok(())
}

fn make_ascii_titlecase(s: &mut str) -> &str {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    s
}
