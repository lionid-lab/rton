#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constructor<Ty, Fi> {
    pub variant: Ty,
    pub tl_id: Option<u32>,
    pub type_parameters: Vec<Fi>,
    pub fields: Vec<Fi>,
    pub output: Ty,
    pub original_variant: String,
    pub original_output: String,
    pub is_function: bool,
}
