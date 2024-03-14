use hound::SampleFormat;

pub trait TypeToFormat {
    const FORMAT: SampleFormat;
}
impl TypeToFormat for i8 {
    const FORMAT: SampleFormat = SampleFormat::Int;
}
impl TypeToFormat for i16 {
    const FORMAT: SampleFormat = SampleFormat::Int;
}
impl TypeToFormat for i32 {
    const FORMAT: SampleFormat = SampleFormat::Int;
}
impl TypeToFormat for f32 {
    const FORMAT: SampleFormat = SampleFormat::Float;
}
