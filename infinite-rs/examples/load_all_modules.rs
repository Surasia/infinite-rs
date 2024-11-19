use infinite_rs::tag::types::common_types::{
    AnyTag, FieldBlock, FieldByteFlags, FieldLongFlags, FieldReference, FieldStringId,
};
use infinite_rs::ModuleFile;
use infinite_rs_derive::TagStructure;

fn load_modules(deploy_path: String) -> infinite_rs::Result<Vec<ModuleFile>> {
    let mut modules = Vec::new();
    for entry in walkdir::WalkDir::new(deploy_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_str().unwrap();
            if file_path.ends_with(".module") {
                let mut module = ModuleFile::new();
                match module.read(file_path) {
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
    Ok(modules)
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x30))]
struct MaterialShaderFunctionParameter {
    #[data(offset(0x00))]
    input_type: FieldLongFlags,
    #[data(offset(0x04))]
    input_name: FieldStringId,
    #[data(offset(0x0C))]
    output_modifier: FieldByteFlags,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x9c))]
struct MaterialParameter {
    #[data(offset(0x8))]
    bitmap: FieldReference,
    #[data(offset(0x80))]
    function_parameters: FieldBlock<MaterialShaderFunctionParameter>,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x38))]
struct MaterialPostprocessTexture {
    #[data(offset(0x00))]
    texture: FieldReference,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0xA0))]
struct PostProcessDefinition {
    #[data(offset(0x00))]
    textures: FieldBlock<MaterialPostprocessTexture>,
}

#[derive(Default, Debug, TagStructure)]
#[data(size(0x88))]
struct MaterialTag {
    #[data(offset(0x00))]
    any_tag: AnyTag,
    #[data(offset(0x10))]
    material_shader: FieldReference,
    #[data(offset(0x2C))]
    material_parameters: FieldBlock<MaterialParameter>,
    #[data(offset(0x40))]
    postprocess_definition: FieldBlock<PostProcessDefinition>,
}

fn main() -> infinite_rs::Result<()> {
    let mut pc_modules =
        load_modules(String::from("C:/XboxGames/Halo Infinite/Content/deploy/pc"))?;

    let mut any_modules = load_modules(String::from(
        "C:/XboxGames/Halo Infinite/Content/deploy/any",
    ))?;

    let mut modules = pc_modules.iter_mut().chain(any_modules.iter_mut());

    for module in &mut modules {
        for index in 0..module.files.len() {
            module.read_tag(index as u32)?;
            let tag = &mut module.files[index];
            if tag.tag_group == "mat " {
                let mut mat = MaterialTag::default();
                tag.read_metadata(&mut mat)?;
                for param in mat.postprocess_definition.elements.iter() {
                    if param.textures.elements.is_empty() {
                        continue;
                    }
                    println!("{:#?}", param.textures);
                }
            }
        }
    }
    Ok(())
}
