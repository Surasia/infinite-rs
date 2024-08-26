use infinite_rs::ModuleFile;

fn main() -> std::io::Result<()> {
    let mut modules = Vec::new();
    let path = "C:/XboxGames/Halo Infinite/Content/deploy/";

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap();
            if file_path.ends_with(".module") {
                let mut module = ModuleFile::default();
                match module.read(String::from(file_path)) {
                    Ok(_) => {
                        modules.push(module);
                        println!("Read module: {}", file_path);
                    }
                    Err(err) => {
                        println!("Failed on file: {}", file_path);
                        return Err(err);
                    }
                };
            }
        }
    }

    for module in &mut modules {
        for index in 0..module.header.file_count {
            module.read_tag(index as usize)?;
        }
    }
    Ok(())
}
