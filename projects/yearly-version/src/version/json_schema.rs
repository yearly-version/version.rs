use super::*;
use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::string::String;
use schemars::_private::apply_metadata;
use schemars::_serde_json::Value;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::{InstanceType, Metadata, Schema, SchemaObject, SingleOrVec, StringValidation};

impl JsonSchema for Version {
    fn schema_name() -> String {
        "Version".to_owned()
    }
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed(concat!(module_path!(), "::", "Version"))
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        let metadata = Metadata { default: Some(Value::String("0.0.0.0".to_string())), ..Default::default() };
        let string = StringValidation {
            max_length: None,
            min_length: None,
            pattern: Some(r"[0-9]{1,10}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,5}".to_string()),
        };
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(metadata)),
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: Some(Box::new(string)),
            array: None,
            object: None,
            reference: None,
            extensions: Default::default(),
        })
    }
}
