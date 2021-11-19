pub mod clients;

#[macro_use]
pub mod clone_from_java;
#[macro_use]
pub mod clone_to_java;
#[macro_use]
pub mod java_stored_object;

pub mod common;

macro_rules! clone_to_from_java_for_struct {
    ($struct_name:ty, $class_name:literal) => {
        clone_to_java!($struct_name, $class_name);
        clone_from_java!($struct_name, $class_name);
        from_jobject!($struct_name, $class_name);
    };
}
// TODO: split
pub(crate) use clone_to_from_java_for_struct;
