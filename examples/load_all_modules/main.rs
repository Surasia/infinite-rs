use anyhow::Result;
use infinite_rs::{
    common::extensions::BufReaderExt,
    tag::types::common_types::{
        AnyTag, FieldReal, FieldRealRGBColor, FieldRealVector2D, FieldReference,
    },
    ModuleFile,
};
use std::io::{BufRead, Seek};

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct CoatingSwatchPODTag {
    any_tag: AnyTag,
    parent: FieldReference,
    color_and_roughness_texture_transform: FieldRealVector2D,
    normal_texture_transform: FieldRealVector2D,
    color_gradient_map: FieldReference,
    gradient_top_color: FieldRealRGBColor,
    gradient_mid_color: FieldRealRGBColor,
    gradient_bot_color: FieldRealRGBColor,
    roughness_white: FieldReal,
    roughness_black: FieldReal,
    normal_detail_map: FieldReference,
    metallic: FieldReal,
    ior: FieldReal,
    albedo_tint_spec: FieldReal,
    scratch_color: FieldRealRGBColor,
    scratch_brightness: FieldReal,
    scratch_roughness: FieldReal,
    scratch_metallic: FieldReal,
    scratch_ior: FieldReal,
    scratch_albedo_tint_spec: FieldReal,
    sss_intensity: FieldReal,
    emissive_intensity: FieldReal,
    emissive_amount: FieldReal,
}

impl CoatingSwatchPODTag {
    fn read<R: BufRead + Seek + BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.any_tag.read(reader)?;
        self.parent.read(reader)?;
        self.color_and_roughness_texture_transform.read(reader)?;
        self.normal_texture_transform.read(reader)?;
        self.color_gradient_map.read(reader)?;
        self.gradient_top_color.read(reader)?;
        self.gradient_mid_color.read(reader)?;
        self.gradient_bot_color.read(reader)?;
        self.roughness_white.read(reader)?;
        self.roughness_black.read(reader)?;
        self.normal_detail_map.read(reader)?;
        self.metallic.read(reader)?;
        self.ior.read(reader)?;
        self.albedo_tint_spec.read(reader)?;
        self.scratch_color.read(reader)?;
        self.scratch_brightness.read(reader)?;
        self.scratch_roughness.read(reader)?;
        self.scratch_metallic.read(reader)?;
        self.scratch_ior.read(reader)?;
        self.scratch_albedo_tint_spec.read(reader)?;
        self.sss_intensity.read(reader)?;
        self.emissive_intensity.read(reader)?;
        self.emissive_amount.read(reader)?;
        Ok(())
    }
}

fn load_modules(deploy_path: String) -> Result<Vec<ModuleFile>> {
    let mut modules = Vec::new();
    for entry in walkdir::WalkDir::new(deploy_path)
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
                        return Err(err.into());
                    }
                };
            }
        }
    }
    Ok(modules)
}

fn load_cmsw(module: &mut ModuleFile, index: usize) -> Result<()> {
    if module.files[index].tag_group == "cmsw" {
        let mut coating_swatch = CoatingSwatchPODTag::default();
        if let Some(ref mut data_stream) = module.files[index].data_stream {
            coating_swatch.read(data_stream)?;
        } else {
            return Err(anyhow::anyhow!(
                "Data stream is missing for file at index {}",
                index
            ));
        }

        #[cfg(feature = "serde")]
        {
            let json = serde_json::to_string_pretty(&coating_swatch)?;
            println!("{}", json);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut pc_modules =
        load_modules(String::from("C:/XboxGames/Halo Infinite/Content/deploy/pc"))?;

    let mut any_modules = load_modules(String::from(
        "C:/XboxGames/Halo Infinite/Content/deploy/any",
    ))?;

    let mut modules = pc_modules.iter_mut().chain(any_modules.iter_mut());

    for module in &mut modules {
        for index in 0..module.files.len() {
            module.read_tag(index as u32)?;
            module.read_resources(index as u32)?;
            load_cmsw(module, index)?;
        }
    }
    Ok(())
}
