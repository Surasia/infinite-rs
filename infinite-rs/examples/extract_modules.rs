use std::{
    fs::{File, create_dir_all},
    io::{BufWriter, Read, Seek, Write},
    path::{Path, PathBuf},
};

use argh::FromArgs;
use infinite_rs::{ModuleFile, Result};

#[derive(FromArgs, Debug)]
/// Tool that extracts files from modules of any version from Halo Infinite
struct InfiniteExtract {
    /// path to where modules are stored (deploy folder)
    #[argh(option)]
    deploy_path: PathBuf,
    /// path to folder to output files to.
    #[argh(option)]
    output_path: PathBuf,
}

fn load_modules<R: AsRef<Path>>(deploy_path: R) -> Result<Vec<ModuleFile>> {
    let mut modules = Vec::new();
    for entry in walkdir::WalkDir::new(deploy_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap();
            if file_path.ends_with(".module") {
                let module = ModuleFile::from_path(file_path)?;
                modules.push(module);
            }
        }
    }
    Ok(modules)
}

fn main() -> Result<()> {
    let args: InfiniteExtract = argh::from_env();
    let mut modules = load_modules(args.deploy_path)?;
    for module in &mut modules {
        for idx in 0..module.files.len() {
            module.read_tag(idx as u32)?;
        }

        for file in &mut module.files {
            let mut buffer = Vec::with_capacity(file.total_uncompressed_size as usize);
            if let Some(stream) = file.data_stream.as_mut() {
                stream.rewind()?;
                stream.read_to_end(&mut buffer)?;
            }
            let tag_path = file
                .tag_name
                .replace(" ", "_")
                .replace("*", "_")
                .replace(r"\", "/")
                .replace(":", "_");
            let path = PathBuf::from(&args.output_path).join(tag_path);
            create_dir_all(path.parent().unwrap())?;
            let filee = File::create(path)?;
            let mut bw = BufWriter::new(&filee);
            bw.write_all(&buffer)?;
        }
    }
    Ok(())
}
