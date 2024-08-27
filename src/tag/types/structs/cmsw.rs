//! CoatingSwatchPODTag definition dumped from the game.
//!
//! Source: <https://github.com/Codename-Atriox/TagStructs/blob/main/Structs/cmsw.xml>

use crate::{
    common::extensions::BufReaderExt,
    tag::types::common_types::{
        AnyTag, FieldReal, FieldRealRGBColor, FieldRealVector2D, FieldReference,
    },
};
use std::io::{BufRead, Seek};

#[derive(Default, Debug)]
/// This type (cmsw) defines a "coating swatch" which is a collection of parameters making up a procedural material for coatings.
///
/// Some of these fields are overwritten by the "style" (aka coating).
pub struct CoatingSwatchPODTag {
    /// VTable space, global tag id and local handle.
    pub any_tag: AnyTag,
    /// Parent of the swatch to inherit from.
    /// Seems to be unused.
    pub parent: FieldReference,
    // Base
    /// The X and Y scaling of the gradient map texture.
    pub color_and_roughness_texture_transform: FieldRealVector2D,
    /// The X and Y scaling of the normal map texture.
    pub normal_texture_transform: FieldRealVector2D,
    /// Bitmap reference to the gradient.
    /// Has to be of type "bitm".
    pub color_gradient_map: FieldReference,
    /// Main gradient value.
    pub gradient_top_color: FieldRealRGBColor,
    /// Secondary gradient value.
    pub gradient_mid_color: FieldRealRGBColor,
    /// Tertiary gradient value.
    pub gradient_bot_color: FieldRealRGBColor,
    /// Upper bound of roughness value (before calculation).
    pub roughness_white: FieldReal,
    /// Lower bound of roughness value (before calculation).
    pub roughness_black: FieldReal,
    /// Bitmap reference to the normal map.
    /// Has to be of type "bitm".
    pub normal_detail_map: FieldReference,
    /// Metallic value used in PBR workflow.
    pub metallic: FieldReal,
    /// Index of Refraction (unused?)
    pub ior: FieldReal,
    /// Unused, leftover from Halo 5.
    pub albedo_tint_spec: FieldReal,
    // Scratches
    /// Color of the scratch masked by the ASG texture.
    pub scratch_color: FieldRealRGBColor,
    /// Unused: Brightness of the scratch color.
    pub scratch_brightness: FieldReal,
    /// Roughness of the scratch layer in PBR workflow.
    pub scratch_roughness: FieldReal,
    /// Metallic of the scratch layer in PBR workflow.
    pub scratch_metallic: FieldReal,
    /// Index of Refraction of scratch layer (unused?)
    pub scratch_ior: FieldReal,
    /// Unused, leftover from Halo 5.
    pub scratch_albedo_tint_spec: FieldReal,
    // Subsurface
    /// Intensity of sub surface scattering.
    /// What this value is used for is unknown.
    pub sss_intensity: FieldReal,
    // Emissive
    /// Intensity of emissives controlling post processing.
    pub emissive_intensity: FieldReal,
    /// "Amount" of emissive controlling the actual shader.
    pub emissive_amount: FieldReal,
}

impl CoatingSwatchPODTag {
    /// Allocate new CoatingSwatchPODTag and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the cnsw structure type from the given readers implementing Read, BufReaderExt and Seek.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `Read + BufReaderExt + Seek` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read<R: BufRead + Seek + BufReaderExt>(
        &mut self,
        reader: &mut R,
    ) -> std::io::Result<()> {
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
