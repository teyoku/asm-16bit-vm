#[derive(Debug, Default, Clone, Copy)]
pub struct Flags {
    pub zero: bool,
    pub negative: bool,
    pub overflow: bool,
}
