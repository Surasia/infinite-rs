use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

use crate::common::extensions::BufReaderExt;

#[derive(Default, Debug)]
/// _0: 32 Byte strings that usually store some sort of short name.
pub struct FieldString {
    pub string: String, // 32 bytes
}

impl FieldString {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.string = reader.read_fixed_string(32)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1: 256 byte long string usually used to store paths.
pub struct FieldLongString {
    pub long_string: String, // 256 bytes
}

impl FieldLongString {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.long_string = reader.read_fixed_string(256)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2: 32 bit unsigned integer containing a MurmurHash3_x86_64 32 bit value.
pub struct FieldStringId {
    pub string_id: u32,
}

impl FieldStringId {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.string_id = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3: Unused
pub struct FieldUnused1 {
    pub unused: [u8; 4],
}

impl FieldUnused1 {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        reader.read_exact(&mut self.unused)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _4: Signed integer type "char" in C.
pub struct FieldCharInteger {
    pub char_integer: i8,
}

impl FieldCharInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.char_integer = reader.read_i8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _5: Signed integer type "short" in C.
pub struct FieldShortInteger {
    pub short_integer: i16,
}

impl FieldShortInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.short_integer = reader.read_i16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _6: Signed integer type "long" in C.
pub struct FieldLongInteger {
    pub long_integer: i32,
}

impl FieldLongInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.long_integer = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _7: Signed integer type "__int64 (long long)" in C.
pub struct FieldInt64Integer {
    pub int64_integer: i64,
}

impl FieldInt64Integer {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.int64_integer = reader.read_i64::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _8: IEE 754 floating point number that stores an angle.
pub struct FieldAngle {
    pub angle: f32,
}

impl FieldAngle {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.angle = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _9: Global ID of a tag. Seems to be unused.
pub struct FieldTag {
    pub tag: i32,
}

impl FieldTag {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.tag = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _A: An unsigned "char" value in C used to calculate enums.
pub struct FieldCharEnum {
    pub char_enum: u8,
}

impl FieldCharEnum {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.char_enum = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _B: An unsigned "short" value in C used to calculate enums.
pub struct FieldShortEnum {
    pub short_enum: u16,
}

impl FieldShortEnum {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.short_enum = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _C: An unsigned "long" value in C used to calculate enums.
pub struct FieldLongEnum {
    pub long_enum: u32,
}

impl FieldLongEnum {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.long_enum = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _D: An unsigned "long" value in C used to calculate bitflags.
pub struct FieldLongFlags {
    pub long_flags: u32,
}

impl FieldLongFlags {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.long_flags = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _E: An unsigned "word (short)" value in C used to calculate bitflags.
pub struct FieldWordFlags {
    pub word_flags: u16,
}

impl FieldWordFlags {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.word_flags = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _F: An unsigned "byte (char)" value in C used to calculate bitflags.
pub struct FieldByteFlags {
    pub byte_flags: u8,
}

impl FieldByteFlags {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.byte_flags = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _10: X and Y coordinates of a point in 2D.
pub struct FieldPoint2D {
    pub x: u16,
    pub y: u16,
}

impl FieldPoint2D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_u16::<LE>()?;
        self.y = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _11:  X and Y coordinates of a rectangle in 2D.
pub struct FieldRectangle2D {
    pub x: u16,
    pub y: u16,
}

impl FieldRectangle2D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_u16::<LE>()?;
        self.y = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _12: RGBA values of a color represented in u8.
/// Alpha value is unused.
pub struct FieldRGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // UNUSED
}

impl FieldRGBColor {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.r = reader.read_u8()?;
        self.g = reader.read_u8()?;
        self.b = reader.read_u8()?;
        self.a = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _13: RGBA values of a color represented in u8.
pub struct FieldARGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl FieldARGBColor {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.r = reader.read_u8()?;
        self.g = reader.read_u8()?;
        self.b = reader.read_u8()?;
        self.a = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _14: Real number represented as a float.
pub struct FieldReal {
    pub real: f32,
}

impl FieldReal {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.real = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _15: Real "fraction" value represented as a float.
pub struct FieldRealFraction {
    pub fraction: f32,
}

impl FieldRealFraction {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.fraction = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _16: X and Y coordinates of point in 2D stored as two floats.
pub struct FieldRealPoint2D {
    pub x: f32,
    pub y: f32,
}

impl FieldRealPoint2D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _17: X, Y and Z coordinates of point in 3D stored as three floats.
pub struct FieldRealPoint3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FieldRealPoint3D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _18: X and Y coordinates of a vector in 2D stored as two floats.
pub struct FieldRealVector2D {
    pub x: f32,
    pub y: f32,
}

impl FieldRealVector2D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _19: X, Y and Z coordinates of a vector in 3D stored as three floats.
pub struct FieldRealVector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FieldRealVector3D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1A: X, Y, Z and W values of a quaternion stored as four floats.
/// Used for rotation math.
pub struct FieldRealQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl FieldRealQuaternion {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        self.w = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1B: X and Y coordinates of a eular angle in 2D stored as two floats.
pub struct FieldRealEulerAngles2D {
    pub x: f32,
    pub y: f32,
}

impl FieldRealEulerAngles2D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1C: X, Y and Z coordinates of a eular angle in 3D stored as two floats.
pub struct FieldRealEularAngles3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FieldRealEularAngles3D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1D: X, Y and D values of a plane in 2D stored as three floats.
pub struct FieldRealPlane2D {
    pub x: f32,
    pub y: f32,
    pub d: f32,
}

impl FieldRealPlane2D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.d = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1E: X, Y, Z and D values of a plane in 3D stored as four floats.
pub struct FieldRealPlane3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub d: f32,
}

impl FieldRealPlane3D {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.x = reader.read_f32::<LE>()?;
        self.y = reader.read_f32::<LE>()?;
        self.z = reader.read_f32::<LE>()?;
        self.d = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _1F: RGB values of a color stored as three floats.
pub struct FieldRealRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FieldRealRGBColor {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.r = reader.read_f32::<LE>()?;
        self.g = reader.read_f32::<LE>()?;
        self.b = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _20: RGBA values of a color stored as four floats.
pub struct FieldRealARGBColor {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FieldRealARGBColor {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.a = reader.read_f32::<LE>()?;
        self.r = reader.read_f32::<LE>()?;
        self.g = reader.read_f32::<LE>()?;
        self.b = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _21: HSV values of a color stored as a single float.
/// Unknown how the actual color is calculated
pub struct FieldRealHSVColor {
    pub hsv: f32,
}

impl FieldRealHSVColor {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.hsv = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _22: AHSV values of a color stored as a single float.
/// Unknown how the actual color is calculated
pub struct FieldRealAHSVColor {
    pub ahsv: f32,
}

impl FieldRealAHSVColor {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.ahsv = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _23: Minimum and Maximum bounds stored as two unsigned shorts in C (u16).
pub struct FieldShortBounds {
    pub min: u16,
    pub max: u16,
}

impl FieldShortBounds {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.min = reader.read_u16::<LE>()?;
        self.max = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _24: Minimum and Maximum angles stored as two floats.
pub struct FieldAngleBounds {
    pub min: f32,
    pub max: f32,
}

impl FieldAngleBounds {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.min = reader.read_f32::<LE>()?;
        self.max = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _25: Minimum and Maximum real values stored as two floats.
pub struct FieldRealBounds {
    pub min: f32,
    pub max: f32,
}

impl FieldRealBounds {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.min = reader.read_f32::<LE>()?;
        self.max = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _26: Minimum and Maximum real fraction values stored as two floats.
pub struct FieldRealFractionBounds {
    pub min: f32,
    pub max: f32,
}

impl FieldRealFractionBounds {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.min = reader.read_f32::<LE>()?;
        self.max = reader.read_f32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _27: Unused field, stores a 32-bit unsigned integer.
pub struct FieldUnused2 {
    pub unused: u32,
}

impl FieldUnused2 {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.unused = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _28: Unused field, stores a 32-bit unsigned integer.
pub struct FieldUnused3 {
    pub unused: u32,
}

impl FieldUnused3 {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.unused = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _29: Long block flags, stores a 32-bit unsigned integer.
pub struct FieldLongBlockFlags {
    pub flags: u32,
}

impl FieldLongBlockFlags {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.flags = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2A: Word block flags, stores a 32-bit unsigned integer.
pub struct FieldWordBlockFlags {
    pub flags: u32,
}

impl FieldWordBlockFlags {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.flags = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2B: Byte block flags, stores a 32-bit unsigned integer.
pub struct FieldByteBlockFlags {
    pub flags: u32,
}

impl FieldByteBlockFlags {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.flags = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2C: Char block index, stores an 8-bit unsigned integer.
pub struct FieldCharBlockIndex {
    pub index: u8,
}

impl FieldCharBlockIndex {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.index = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2D: Custom char block index, stores an 8-bit unsigned integer.
pub struct FieldCustomCharBlockIndex {
    pub index: u8,
}

impl FieldCustomCharBlockIndex {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.index = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2E: Short block index, stores a 16-bit unsigned integer.
pub struct FieldShortBlockIndex {
    pub index: u16,
}

impl FieldShortBlockIndex {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.index = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _2F: Custom short block index, stores a 16-bit unsigned integer.
pub struct FieldCustomShortBlockIndex {
    pub index: u16,
}

impl FieldCustomShortBlockIndex {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.index = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _30: Long block index, stores a 32-bit unsigned integer.
pub struct FieldLongBlockIndex {
    pub index: u32,
}

impl FieldLongBlockIndex {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.index = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _31: Custom long block index, stores a 32-bit unsigned integer.
pub struct FieldCustomLongBlockIndex {
    pub index: u32,
}

impl FieldCustomLongBlockIndex {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.index = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _32: Unused field, stores a 32-bit unsigned integer.
pub struct FieldUnused4 {
    pub unused: u32,
}

impl FieldUnused4 {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.unused = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _33: Unused field, stores a 32-bit unsigned integer.
pub struct FieldUnused5 {
    pub unused: u32,
}

impl FieldUnused5 {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.unused = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _34: Padding field, no data stored.
pub struct FieldPad;

#[derive(Default, Debug)]
/// _35: Skip field, no data stored.
pub struct FieldSkip;

#[derive(Default, Debug)]
/// _36: Explanation field, no data stored.
pub struct FieldExplanation;

#[derive(Default, Debug)]
/// _37: Custom field, no data stored.
pub struct FieldCustom;

#[derive(Default, Debug)]
/// _38: Struct field, no data stored.
pub struct FieldStruct;

#[derive(Default, Debug)]
/// _39: Array field, no data stored.
pub struct FieldArray;

#[derive(Default, Debug)]
/// _3A: Unused field, stores a 32-bit unsigned integer.
pub struct FieldUnused6 {
    pub unused: u32,
}

impl FieldUnused6 {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.unused = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3B: End of struct marker, no data stored.
pub struct FieldEndOfStruct;

#[derive(Default, Debug)]
/// _3C: Byte integer field, stores an 8-bit unsigned integer.
pub struct FieldByteInteger {
    pub byte_integer: u8,
}

impl FieldByteInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.byte_integer = reader.read_u8()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3D: Word integer field, stores a 16-bit unsigned integer.
pub struct FieldWordInteger {
    pub word_integer: u16,
}

impl FieldWordInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.word_integer = reader.read_u16::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3E: Dword integer field, stores a 32-bit unsigned integer.
pub struct FieldDwordInteger {
    pub dword_integer: u32,
}

impl FieldDwordInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.dword_integer = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _3F: Qword integer field, stores a 64-bit unsigned integer.
pub struct FieldQwordInteger {
    pub qword_integer: u64,
}

impl FieldQwordInteger {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.qword_integer = reader.read_u64::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _40: Block field, stores a fixed-size array of 20 bytes.
pub struct FieldBlock {
    pub block: [u8; 20],
}

impl FieldBlock {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        reader.read_exact(&mut self.block)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _41: Reference to an external tag.
pub struct FieldReference {
    pub type_info: u64, // uintptr at runtime
    pub global_id: i32,
    pub asset_id: u64,
    pub group: String,
    pub local_handle: i32,
}

impl FieldReference {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.type_info = reader.read_u64::<LE>()?;
        self.global_id = reader.read_i32::<LE>()?;
        self.asset_id = reader.read_u64::<LE>()?;
        self.group = reader.read_fixed_string(4)?;
        self.local_handle = reader.read_i32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _42: "External" resource inside tag.
pub struct FieldData {
    pub data: u64,      // uintptr at runtime
    pub type_info: u64, // uintptr at runtime
    pub unknown: u32,
    pub size: u32,
}

impl FieldData {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.data = reader.read_u64::<LE>()?;
        self.type_info = reader.read_u64::<LE>()?;
        self.unknown = reader.read_u32::<LE>()?;
        self.size = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// _43: Reference to tag resource, only useful at runtime.
pub struct FieldTagResource {
    pub block: u64, // uintptr at runtime
    pub handle: u32,
    pub unknown: u32,
}

impl FieldTagResource {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.block = reader.read_u64::<LE>()?;
        self.handle = reader.read_u32::<LE>()?;
        self.unknown = reader.read_u32::<LE>()?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// "Internal struct" of AnyTag field.
pub struct AnyTagGuts {
    pub tag_id: FieldLongInteger,
    pub local_tag_handle: FieldLongInteger,
}

impl AnyTagGuts {
    pub fn read<R: Read + Seek>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.tag_id.read(reader)?;
        self.local_tag_handle.read(reader)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
/// "AnyTag" is present in all non-resource tags.
/// Is used at runtime to calculate locations of tags in memory.
pub struct AnyTag {
    pub vtable_space: FieldInt64Integer,
    pub internal_struct: AnyTagGuts,
}

impl AnyTag {
    pub fn read<R: Read + Seek>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.vtable_space.read(reader)?;
        self.internal_struct.read(reader)?;
        Ok(())
    }
}
