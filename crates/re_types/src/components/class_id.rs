// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#[doc = "A 16-bit ID representing a type of semantic class."]
#[doc = ""]
#[doc = "Used to look up a `crate::components::ClassDescription` within the `crate::components::AnnotationContext`."]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassId(pub u16);

impl crate::Component for ClassId {
    fn name() -> crate::ComponentName {
        crate::ComponentName::Borrowed("rerun.class_id")
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Extension(
            "rerun.components.ClassId".to_owned(),
            Box::new(DataType::UInt16),
            None,
        )
    }
}
