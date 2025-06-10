



pub trait EnumReflect {
    fn get_fields(&self) -> Vec<&dyn std::any::Any>;
    fn get_fields_mut(&mut self) -> Vec<&mut dyn std::any::Any>;
    fn get_named_fields(&self) -> Vec<(&'static str, &dyn std::any::Any)>;
    fn get_named_fields_mut(&mut self) -> Vec<(&'static str, &mut dyn std::any::Any)>;
}
