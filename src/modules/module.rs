/// Trait used for modules, so that they have the same base functions
///
/// # Functions
/// * `get_id` - Gets the id of the module
/// * `get_name` - Gets the name of the module
/// * `get_description` - Gets the description of the module
/// * `get_data` - Gets data from the module
pub trait Module {
    fn get_id(&self) -> i16;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_data(&self) -> Vec<(String, i16)>;
}