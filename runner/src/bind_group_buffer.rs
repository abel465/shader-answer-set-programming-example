#[allow(dead_code)]
pub enum BindGroupBufferType<'a> {
    Uniform(Uniform<'a>),
    SSBO(SSBO<'a>),
}

pub struct SSBO<'a> {
    pub data: &'a [u8],
    pub read_only: bool,
}

pub struct Uniform<'a> {
    pub data: &'a [u8],
}

#[derive(Default)]
pub struct BufferData<'a> {
    pub bind_group_buffers: Vec<BindGroupBufferType<'a>>,
}
