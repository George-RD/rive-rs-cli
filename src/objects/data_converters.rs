use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct DataConverterRounder {
    pub name: String,
    pub decimals: u64,
}

impl RiveObject for DataConverterRounder {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_ROUNDER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.decimals != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_ROUNDER_DECIMALS,
                value: PropertyValue::UInt(self.decimals),
            });
        }
        props
    }
}

pub struct DataConverterToString {
    pub name: String,
    pub flags: u64,
    pub decimals: u64,
    pub color_format: String,
}

impl RiveObject for DataConverterToString {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_TO_STRING
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.flags != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_TO_STRING_FLAGS,
                value: PropertyValue::UInt(self.flags),
            });
        }
        if self.decimals != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_TO_STRING_DECIMALS,
                value: PropertyValue::UInt(self.decimals),
            });
        }
        if !self.color_format.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_TO_STRING_COLOR_FORMAT,
                value: PropertyValue::String(self.color_format.clone()),
            });
        }
        props
    }
}

pub struct DataConverterToNumber {
    pub name: String,
}

impl RiveObject for DataConverterToNumber {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_TO_NUMBER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        props
    }
}

pub struct DataConverterGroup {
    pub name: String,
}

impl RiveObject for DataConverterGroup {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_GROUP
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        props
    }
}

pub struct DataConverterGroupItem {
    pub converter_id: u64,
}

impl RiveObject for DataConverterGroupItem {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_GROUP_ITEM
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.converter_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_GROUP_ITEM_CONVERTER_ID,
                value: PropertyValue::UInt(self.converter_id),
            });
        }
        props
    }
}

pub struct DataConverterOperationValue {
    pub name: String,
    pub operation_type: u64,
    pub operation_value: f32,
}

impl RiveObject for DataConverterOperationValue {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_OPERATION_VALUE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.operation_type != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_OPERATION_TYPE,
                value: PropertyValue::UInt(self.operation_type),
            });
        }
        if self.operation_value != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_OPERATION_VALUE_VALUE,
                value: PropertyValue::Float(self.operation_value),
            });
        }
        props
    }
}

pub struct DataConverterTrigger {
    pub name: String,
}

impl RiveObject for DataConverterTrigger {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_TRIGGER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        props
    }
}

pub struct DataConverterOperationViewModel {
    pub name: String,
    pub operation_type: u64,
}

impl RiveObject for DataConverterOperationViewModel {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_OPERATION_VIEW_MODEL
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.operation_type != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_OPERATION_TYPE,
                value: PropertyValue::UInt(self.operation_type),
            });
        }
        props
    }
}

pub struct DataConverterStringPad {
    pub name: String,
    pub length: u64,
    pub text: String,
    pub pad_type: u64,
}

impl RiveObject for DataConverterStringPad {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_STRING_PAD
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.length != 1 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_STRING_PAD_LENGTH,
                value: PropertyValue::UInt(self.length),
            });
        }
        if !self.text.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_STRING_PAD_TEXT,
                value: PropertyValue::String(self.text.clone()),
            });
        }
        if self.pad_type != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_STRING_PAD_TYPE,
                value: PropertyValue::UInt(self.pad_type),
            });
        }
        props
    }
}

pub struct DataConverterStringRemoveZeros {
    pub name: String,
}

impl RiveObject for DataConverterStringRemoveZeros {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_STRING_REMOVE_ZEROS
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        props
    }
}

pub struct DataConverterStringTrim {
    pub name: String,
    pub trim_type: u64,
}

impl RiveObject for DataConverterStringTrim {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_STRING_TRIM
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.trim_type != 1 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_STRING_TRIM_TYPE,
                value: PropertyValue::UInt(self.trim_type),
            });
        }
        props
    }
}

pub struct DataConverterInterpolator {
    pub name: String,
    pub duration: f32,
    pub interpolation_type: u64,
    pub interpolator_id: u64,
}

impl RiveObject for DataConverterInterpolator {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_INTERPOLATOR
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.duration != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_INTERPOLATOR_DURATION,
                value: PropertyValue::Float(self.duration),
            });
        }
        if self.interpolation_type != 1 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_INTERPOLATOR_INTERPOLATION_TYPE,
                value: PropertyValue::UInt(self.interpolation_type),
            });
        }
        if self.interpolator_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_INTERPOLATOR_INTERPOLATOR_ID,
                value: PropertyValue::UInt(self.interpolator_id),
            });
        }
        props
    }
}

pub struct DataConverterBooleanNegate {
    pub name: String,
}

impl RiveObject for DataConverterBooleanNegate {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_BOOLEAN_NEGATE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        props
    }
}

pub struct DataConverterRangeMapper {
    pub name: String,
    pub interpolation_type: u64,
    pub interpolator_id: u64,
    pub flags: u64,
    pub min_input: f32,
    pub max_input: f32,
    pub min_output: f32,
    pub max_output: f32,
}

impl RiveObject for DataConverterRangeMapper {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_RANGE_MAPPER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.interpolation_type != 1 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_INTERPOLATION_TYPE,
                value: PropertyValue::UInt(self.interpolation_type),
            });
        }
        if self.interpolator_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_INTERPOLATOR_ID,
                value: PropertyValue::UInt(self.interpolator_id),
            });
        }
        if self.flags != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_FLAGS,
                value: PropertyValue::UInt(self.flags),
            });
        }
        if self.min_input != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_MIN_INPUT,
                value: PropertyValue::Float(self.min_input),
            });
        }
        if self.max_input != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_MAX_INPUT,
                value: PropertyValue::Float(self.max_input),
            });
        }
        if self.min_output != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_MIN_OUTPUT,
                value: PropertyValue::Float(self.min_output),
            });
        }
        if self.max_output != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_RANGE_MAPPER_MAX_OUTPUT,
                value: PropertyValue::Float(self.max_output),
            });
        }
        props
    }
}

pub struct DataConverterFormula {
    pub name: String,
    pub random_mode_value: u64,
}

impl RiveObject for DataConverterFormula {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_FORMULA
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.random_mode_value != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_FORMULA_RANDOM_MODE_VALUE,
                value: PropertyValue::UInt(self.random_mode_value),
            });
        }
        props
    }
}

pub struct DataConverterSystemDegsToRads {
    pub name: String,
    pub operation_type: u64,
}

impl RiveObject for DataConverterSystemDegsToRads {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_SYSTEM_DEGS_TO_RADS
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.operation_type != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_OPERATION_TYPE,
                value: PropertyValue::UInt(self.operation_type),
            });
        }
        props
    }
}

pub struct DataConverterSystemNormalizer {
    pub name: String,
    pub operation_type: u64,
    pub operation_value: f32,
}

impl RiveObject for DataConverterSystemNormalizer {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_SYSTEM_NORMALIZER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.operation_type != 0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_OPERATION_TYPE,
                value: PropertyValue::UInt(self.operation_type),
            });
        }
        if self.operation_value != 1.0 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_OPERATION_VALUE_VALUE,
                value: PropertyValue::Float(self.operation_value),
            });
        }
        props
    }
}

pub struct DataConverterNumberToList {
    pub name: String,
    pub view_model_id: u64,
}

impl RiveObject for DataConverterNumberToList {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_NUMBER_TO_LIST
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.view_model_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NUMBER_TO_LIST_VIEW_MODEL_ID,
                value: PropertyValue::UInt(self.view_model_id),
            });
        }
        props
    }
}

pub struct DataConverterListToLength {
    pub name: String,
}

impl RiveObject for DataConverterListToLength {
    fn type_key(&self) -> u16 {
        type_keys::DATA_CONVERTER_LIST_TO_LENGTH
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        props
    }
}

pub struct FormulaTokenArgumentSeparator {
    pub parent_id: u64,
}

impl RiveObject for FormulaTokenArgumentSeparator {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_ARGUMENT_SEPARATOR
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        props
    }
}

pub struct FormulaTokenParenthesisClose {
    pub parent_id: u64,
}

impl RiveObject for FormulaTokenParenthesisClose {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_PARENTHESIS_CLOSE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        props
    }
}

pub struct FormulaTokenOperation {
    pub parent_id: u64,
    pub operation_type: u64,
}

impl RiveObject for FormulaTokenOperation {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_OPERATION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        if self.operation_type != 0 {
            props.push(Property {
                key: property_keys::FORMULA_TOKEN_OPERATION_TYPE,
                value: PropertyValue::UInt(self.operation_type),
            });
        }
        props
    }
}

pub struct FormulaTokenFunction {
    pub parent_id: u64,
    pub function_type: u64,
}

impl RiveObject for FormulaTokenFunction {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_FUNCTION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        if self.function_type != 0 {
            props.push(Property {
                key: property_keys::FORMULA_TOKEN_FUNCTION_TYPE,
                value: PropertyValue::UInt(self.function_type),
            });
        }
        props
    }
}

pub struct FormulaTokenValue {
    pub parent_id: u64,
    pub operation_value: f32,
}

impl RiveObject for FormulaTokenValue {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_VALUE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        if self.operation_value != 1.0 {
            props.push(Property {
                key: property_keys::FORMULA_TOKEN_VALUE_OPERATION_VALUE,
                value: PropertyValue::Float(self.operation_value),
            });
        }
        props
    }
}

pub struct FormulaTokenParenthesisOpen {
    pub parent_id: u64,
}

impl RiveObject for FormulaTokenParenthesisOpen {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_PARENTHESIS_OPEN
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        props
    }
}

pub struct FormulaTokenInput {
    pub parent_id: u64,
}

impl RiveObject for FormulaTokenInput {
    fn type_key(&self) -> u16 {
        type_keys::FORMULA_TOKEN_INPUT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.parent_id != 0 {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_converter_rounder_type_key() {
        let obj = DataConverterRounder {
            name: "rounder".to_string(),
            decimals: 0,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_ROUNDER);
    }

    #[test]
    fn test_data_converter_rounder_default_suppression() {
        let obj = DataConverterRounder {
            name: "rounder".to_string(),
            decimals: 0,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::DATA_CONVERTER_NAME);
    }

    #[test]
    fn test_data_converter_rounder_non_default() {
        let obj = DataConverterRounder {
            name: "rounder".to_string(),
            decimals: 2,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[1].key, property_keys::DATA_CONVERTER_ROUNDER_DECIMALS);
        assert_eq!(props[1].value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_data_converter_to_string_type_key() {
        let obj = DataConverterToString {
            name: "to_str".to_string(),
            flags: 0,
            decimals: 0,
            color_format: String::new(),
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_TO_STRING);
    }

    #[test]
    fn test_data_converter_to_string_defaults() {
        let obj = DataConverterToString {
            name: "".to_string(),
            flags: 0,
            decimals: 0,
            color_format: String::new(),
        };
        assert!(obj.properties().is_empty());
    }

    #[test]
    fn test_data_converter_to_string_non_default() {
        let obj = DataConverterToString {
            name: "fmt".to_string(),
            flags: 1,
            decimals: 3,
            color_format: "hex".to_string(),
        };
        let props = obj.properties();
        assert_eq!(props.len(), 4);
    }

    #[test]
    fn test_data_converter_to_number_type_key() {
        let obj = DataConverterToNumber {
            name: "to_num".to_string(),
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_TO_NUMBER);
    }

    #[test]
    fn test_data_converter_group_type_key() {
        let obj = DataConverterGroup {
            name: "grp".to_string(),
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_GROUP);
    }

    #[test]
    fn test_data_converter_group_item_type_key() {
        let obj = DataConverterGroupItem {
            converter_id: u32::MAX as u64,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_GROUP_ITEM);
    }

    #[test]
    fn test_data_converter_group_item_default_suppression() {
        let obj = DataConverterGroupItem {
            converter_id: u32::MAX as u64,
        };
        assert!(obj.properties().is_empty());
    }

    #[test]
    fn test_data_converter_group_item_non_default() {
        let obj = DataConverterGroupItem { converter_id: 5 };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(
            props[0].key,
            property_keys::DATA_CONVERTER_GROUP_ITEM_CONVERTER_ID
        );
        assert_eq!(props[0].value, PropertyValue::UInt(5));
    }

    #[test]
    fn test_data_converter_operation_value_type_key() {
        let obj = DataConverterOperationValue {
            name: "op".to_string(),
            operation_type: 0,
            operation_value: 1.0,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_OPERATION_VALUE);
    }

    #[test]
    fn test_data_converter_operation_value_defaults() {
        let obj = DataConverterOperationValue {
            name: "op".to_string(),
            operation_type: 0,
            operation_value: 1.0,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
    }

    #[test]
    fn test_data_converter_operation_value_non_default() {
        let obj = DataConverterOperationValue {
            name: "multiply".to_string(),
            operation_type: 2,
            operation_value: 3.5,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[1].key, property_keys::DATA_CONVERTER_OPERATION_TYPE);
        assert_eq!(
            props[2].key,
            property_keys::DATA_CONVERTER_OPERATION_VALUE_VALUE
        );
        assert_eq!(props[2].value, PropertyValue::Float(3.5));
    }

    #[test]
    fn test_data_converter_trigger_type_key() {
        let obj = DataConverterTrigger {
            name: "trig".to_string(),
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_TRIGGER);
    }

    #[test]
    fn test_data_converter_operation_view_model_type_key() {
        let obj = DataConverterOperationViewModel {
            name: "op_vm".to_string(),
            operation_type: 0,
        };
        assert_eq!(
            obj.type_key(),
            type_keys::DATA_CONVERTER_OPERATION_VIEW_MODEL
        );
    }

    #[test]
    fn test_data_converter_string_pad_type_key() {
        let obj = DataConverterStringPad {
            name: "pad".to_string(),
            length: 1,
            text: String::new(),
            pad_type: 0,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_STRING_PAD);
    }

    #[test]
    fn test_data_converter_string_pad_defaults() {
        let obj = DataConverterStringPad {
            name: "pad".to_string(),
            length: 1,
            text: String::new(),
            pad_type: 0,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
    }

    #[test]
    fn test_data_converter_string_pad_non_default() {
        let obj = DataConverterStringPad {
            name: "pad".to_string(),
            length: 10,
            text: "0".to_string(),
            pad_type: 1,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 4);
    }

    #[test]
    fn test_data_converter_string_remove_zeros_type_key() {
        let obj = DataConverterStringRemoveZeros {
            name: "rm0".to_string(),
        };
        assert_eq!(
            obj.type_key(),
            type_keys::DATA_CONVERTER_STRING_REMOVE_ZEROS
        );
    }

    #[test]
    fn test_data_converter_string_trim_type_key() {
        let obj = DataConverterStringTrim {
            name: "trim".to_string(),
            trim_type: 1,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_STRING_TRIM);
    }

    #[test]
    fn test_data_converter_string_trim_default_suppression() {
        let obj = DataConverterStringTrim {
            name: "trim".to_string(),
            trim_type: 1,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
    }

    #[test]
    fn test_data_converter_string_trim_non_default() {
        let obj = DataConverterStringTrim {
            name: "trim".to_string(),
            trim_type: 3,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[1].key, property_keys::DATA_CONVERTER_STRING_TRIM_TYPE);
    }

    #[test]
    fn test_data_converter_interpolator_type_key() {
        let obj = DataConverterInterpolator {
            name: "interp".to_string(),
            duration: 1.0,
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_INTERPOLATOR);
    }

    #[test]
    fn test_data_converter_interpolator_defaults() {
        let obj = DataConverterInterpolator {
            name: "interp".to_string(),
            duration: 1.0,
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
    }

    #[test]
    fn test_data_converter_interpolator_non_default() {
        let obj = DataConverterInterpolator {
            name: "interp".to_string(),
            duration: 0.5,
            interpolation_type: 2,
            interpolator_id: 3,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 4);
    }

    #[test]
    fn test_data_converter_boolean_negate_type_key() {
        let obj = DataConverterBooleanNegate {
            name: "negate".to_string(),
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_BOOLEAN_NEGATE);
    }

    #[test]
    fn test_data_converter_range_mapper_type_key() {
        let obj = DataConverterRangeMapper {
            name: "map".to_string(),
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
            flags: 0,
            min_input: 1.0,
            max_input: 1.0,
            min_output: 1.0,
            max_output: 1.0,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_RANGE_MAPPER);
    }

    #[test]
    fn test_data_converter_range_mapper_defaults() {
        let obj = DataConverterRangeMapper {
            name: "map".to_string(),
            interpolation_type: 1,
            interpolator_id: u32::MAX as u64,
            flags: 0,
            min_input: 1.0,
            max_input: 1.0,
            min_output: 1.0,
            max_output: 1.0,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
    }

    #[test]
    fn test_data_converter_range_mapper_non_default() {
        let obj = DataConverterRangeMapper {
            name: "map".to_string(),
            interpolation_type: 2,
            interpolator_id: 7,
            flags: 1,
            min_input: 0.0,
            max_input: 100.0,
            min_output: 0.0,
            max_output: 200.0,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 8);
    }

    #[test]
    fn test_data_converter_formula_type_key() {
        let obj = DataConverterFormula {
            name: "formula".to_string(),
            random_mode_value: 0,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_FORMULA);
    }

    #[test]
    fn test_data_converter_system_degs_to_rads_type_key() {
        let obj = DataConverterSystemDegsToRads {
            name: "d2r".to_string(),
            operation_type: 0,
        };
        assert_eq!(
            obj.type_key(),
            type_keys::DATA_CONVERTER_SYSTEM_DEGS_TO_RADS
        );
    }

    #[test]
    fn test_data_converter_system_normalizer_type_key() {
        let obj = DataConverterSystemNormalizer {
            name: "norm".to_string(),
            operation_type: 0,
            operation_value: 1.0,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_SYSTEM_NORMALIZER);
    }

    #[test]
    fn test_data_converter_number_to_list_type_key() {
        let obj = DataConverterNumberToList {
            name: "n2l".to_string(),
            view_model_id: u32::MAX as u64,
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_NUMBER_TO_LIST);
    }

    #[test]
    fn test_data_converter_list_to_length_type_key() {
        let obj = DataConverterListToLength {
            name: "l2l".to_string(),
        };
        assert_eq!(obj.type_key(), type_keys::DATA_CONVERTER_LIST_TO_LENGTH);
    }

    #[test]
    fn test_formula_token_argument_separator_type_key() {
        let obj = FormulaTokenArgumentSeparator { parent_id: 0 };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_ARGUMENT_SEPARATOR);
    }

    #[test]
    fn test_formula_token_parenthesis_close_type_key() {
        let obj = FormulaTokenParenthesisClose { parent_id: 0 };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_PARENTHESIS_CLOSE);
    }

    #[test]
    fn test_formula_token_operation_type_key() {
        let obj = FormulaTokenOperation {
            parent_id: 0,
            operation_type: 0,
        };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_OPERATION);
    }

    #[test]
    fn test_formula_token_operation_non_default() {
        let obj = FormulaTokenOperation {
            parent_id: 3,
            operation_type: 1,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[1].key, property_keys::FORMULA_TOKEN_OPERATION_TYPE);
    }

    #[test]
    fn test_formula_token_function_type_key() {
        let obj = FormulaTokenFunction {
            parent_id: 0,
            function_type: 0,
        };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_FUNCTION);
    }

    #[test]
    fn test_formula_token_value_type_key() {
        let obj = FormulaTokenValue {
            parent_id: 0,
            operation_value: 1.0,
        };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_VALUE);
    }

    #[test]
    fn test_formula_token_value_non_default() {
        let obj = FormulaTokenValue {
            parent_id: 2,
            operation_value: 42.0,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(
            props[1].key,
            property_keys::FORMULA_TOKEN_VALUE_OPERATION_VALUE
        );
        assert_eq!(props[1].value, PropertyValue::Float(42.0));
    }

    #[test]
    fn test_formula_token_parenthesis_open_type_key() {
        let obj = FormulaTokenParenthesisOpen { parent_id: 0 };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_PARENTHESIS_OPEN);
    }

    #[test]
    fn test_formula_token_input_type_key() {
        let obj = FormulaTokenInput { parent_id: 0 };
        assert_eq!(obj.type_key(), type_keys::FORMULA_TOKEN_INPUT);
    }
}
