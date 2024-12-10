use std::{
    fs::File,
    io::{BufWriter, Read, Seek, SeekFrom, Write},
};

use infinite_rs::{tag::types::common_types::FieldData, ModuleFile, Result};
use infinite_rs_derive::TagStructure;

const DEPLOY_PATH: &str =
    "C:/XboxGames/Halo Infinite/Content/deploy/any/globals/globals-rtx-new.module";
const SAVE_PATH: &str = "./scripts";
const SCRIPT_GROUP: &str = "hsc*";

#[derive(Default, Debug, TagStructure)]
#[data(size(0x2D8))]
struct HsSourceFileTag {
    #[data(offset(0x294))]
    server: FieldData,
    #[data(offset(0x2AC))]
    client: FieldData,
}

fn main() -> Result<()> {
    let mut module = ModuleFile::from_path(DEPLOY_PATH)?;
    for idx in 0..module.files.len() {
        if module.files[idx].tag_group == SCRIPT_GROUP {
            module.read_tag(idx as u32)?;
            let mut source = HsSourceFileTag::default();
            module.files[idx].read_metadata(&mut source)?;

            let size = module.files[idx].uncompressed_header_size + 0x2D8;
            let tag_id = module.files[idx].tag_id;
            let mut server_buf = vec![0; source.server.size as usize];
            let mut client_buf = vec![0; source.client.size as usize];

            if let Some(stream) = module.files[idx].data_stream.as_mut() {
                stream.seek(SeekFrom::Start(size as u64))?;
                stream.read_exact(&mut server_buf)?;
                stream.read_exact(&mut client_buf)?;
            }

            let server_file = File::create(format!("{SAVE_PATH}/{}_server.luac", tag_id))?;
            let mut bw = BufWriter::new(server_file);
            bw.write_all(&server_buf)?;

            let client_file = File::create(format!("{SAVE_PATH}/{}_client.luac", tag_id))?;
            let mut bw = BufWriter::new(client_file);
            bw.write_all(&client_buf)?;
        }
    }
    Ok(())
}
