/// _0
pub struct FieldString {
    pub string: String, // 32 bytes
}

/// _1
pub struct FieldLongString {
    pub long_string: String, // 256 bytes
}

/// _2
pub struct FieldStringId {
    pub string_id: u32,
}

/// _3
pub struct FieldUnused1 {
    pub unused: [u8; 4],
}

/// _4
pub struct FieldCharInteger {
    pub char_integer: i8,
}

/// _5
pub struct FieldShortInteger {
    pub short_integer: i16,
}

/// _6
pub struct FieldLongInteger {
    pub long_integer: i32,
}

/// _7
pub struct FieldInt64Integer {
    pub int64_integer: i64,
}

/// _8
pub struct FieldAngle {
    pub angle: f32,
}

/// _9
pub struct FieldTag {
    pub tag: i32,
}

/// _A
pub struct FieldCharEnum {
    pub char_enum: u8,
}

/// _B
pub struct FieldShortEnum {
    pub short_enum: u16,
}

/// _C
pub struct FieldLongEnum {
    pub long_enum: u32,
}

/// _D
pub struct FieldLongFlags {
    pub long_flags: u32,
}

/// _E
pub struct FieldWordFlags {
    pub word_flags: u16,
}

/// _F
pub struct FieldByteFlags {
    pub byte_flags: u8,
}

/// _10
pub struct FieldPoint2D {
    pub x: u16,
    pub y: u16,
}

/// _11
pub struct FieldRectangle2D {
    pub x: u16,
    pub y: u16,
}

/// _12
pub struct FieldRGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // UNUSED
}

/// _13
pub struct FieldARGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// _14
pub struct FieldReal {
    pub real: f32,
}

/// _15
pub struct FieldRealFraction {
    pub fraction: f32,
}

/// _16
pub struct FieldRealPoint2D {
    pub x: f32,
    pub y: f32,
}

/// _17
pub struct FieldRealPoint3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// _18
pub struct FieldRealVector2D {
    pub x: f32,
    pub y: f32,
}

/// _19
pub struct FieldRealVector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// _1A
pub struct FieldRealQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// _1B
pub struct FieldRealEulerAngles2D {
    pub x: f32,
    pub y: f32,
}

/// _1C
pub struct FieldRealEularAngles3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// _1D
pub struct FieldRealPlane2D {
    pub x: f32,
    pub y: f32,
    pub d: f32,
}

/// _1E
pub struct FieldRealPlane3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub d: f32,
}

/// _1F
pub struct FieldRealRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// _20
pub struct FieldRealARGBColor {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// _21
pub struct FieldRealHSVColor {
    pub hsv: f32,
}

/// _22
pub struct FieldRealAHSVColor {
    pub ahsv: f32,
}

/// _23
pub struct FieldShortBounds {
    pub min: u16,
    pub max: u16,
}

/// _24
pub struct FieldAngleBounds {
    pub min: f32,
    pub max: f32,
}

/// _25
pub struct FieldRealBounds {
    pub min: f32,
    pub max: f32,
}

/// _26
pub struct FieldRealFractionBounds {
    pub min: f32,
    pub max: f32,
}

/// _27
pub struct FieldUnused2 {
    pub unused: u32,
}

/// _28
pub struct FieldUnused3 {
    pub unused: u32,
}

/// _29
pub struct FieldLongBlockFlags {
    pub flags: u32,
}

/// _2A
pub struct FieldWordBlockFlags {
    pub flags: u32,
}

/// _2B
pub struct FieldByteBlockFlags {
    pub flags: u32,
}

/// _2C
pub struct FieldCharBlockIndex {
    pub index: u8,
}

/// _2D
pub struct FieldCustomCharBlockIndex {
    pub index: u8,
}

/// _2E
pub struct FieldShortBlockIndex {
    pub index: u16,
}

/// _2F
pub struct FieldCustomShortBlockIndex {
    pub index: u16,
}

/// _30
pub struct FieldLongBlockIndex {
    pub index: u32,
}
/// _31
pub struct FieldCustomLongBlockIndex {
    pub index: u32,
}

/// _32
pub struct FieldUnused4 {
    pub unused: u32,
}

/// _33
pub struct FieldUnused5 {
    pub unused: u32,
}

/// _34
pub struct FieldPad;

/// _35
pub struct FieldSkip;

/// _36
pub struct FieldExplanation;

/// _37
pub struct FieldCustom;

/// _38
pub struct FieldStruct;

/// _39
pub struct FieldArray;

/// _3A
pub struct FieldUnused6 {
    pub unused: u32,
}

/// _3B
pub struct FieldEndOfStruct;

/// _3C
pub struct FieldByteInteger {
    pub byte_integer: u8,
}

/// _3D
pub struct FieldWordInteger {
    pub word_integer: u16,
}

/// _3E
pub struct FieldDwordInteger {
    pub dword_integer: u32,
}

/// _3F
pub struct FieldQwordInteger {
    pub qword_integer: u64,
}

/// _40
pub struct FieldBlock {
    pub block: [u8; 20],
}

/// _41
pub struct FieldReference {
    pub type_info: u64, // uintptr at runtime
    pub global_id: i32,
    pub asset_id: u64,
    pub class_id: String,
    pub local_handle: i32,
}

/// _42
pub struct FieldData {
    pub block: u64, // uintptr at runtime
    pub handle: u32,
    pub unknown: u32,
}

/// _43
pub struct FieldResource {
    pub data: u64,      // uintptr at runtime
    pub type_info: u64, // uintptr at runtime
    pub unknown: u32,
    pub size: u32,
}

/// _44
pub struct FieldUnused7 {
    pub unknown: u32,
}

/// _45
pub struct FieldUnused8 {
    pub unknown: u32,
}
