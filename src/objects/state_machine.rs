use super::core::{property_keys, type_keys, Property, PropertyValue, RiveObject};

pub struct StateMachine {
    pub name: String,
    pub parent_id: u64,
}

impl StateMachine {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self { name, parent_id }
    }
}

impl RiveObject for StateMachine {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::ANIMATION_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct StateMachineComponent {
    pub name: String,
}

impl RiveObject for StateMachineComponent {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_COMPONENT
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

pub struct StateMachineInput {
    pub name: String,
}

impl RiveObject for StateMachineInput {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_INPUT
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

pub struct StateMachineNumber {
    pub name: String,
    pub value: f32,
}

impl RiveObject for StateMachineNumber {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_NUMBER
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::STATE_MACHINE_COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::STATE_MACHINE_NUMBER_VALUE,
                value: PropertyValue::Float(self.value),
            },
        ]
    }
}

pub struct StateMachineBool {
    pub name: String,
    pub value: u64,
}

impl RiveObject for StateMachineBool {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_BOOL
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::STATE_MACHINE_COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::STATE_MACHINE_BOOL_VALUE,
                value: PropertyValue::UInt(self.value),
            },
        ]
    }
}

pub struct StateMachineTrigger {
    pub name: String,
}

impl RiveObject for StateMachineTrigger {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_TRIGGER
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

pub struct StateMachineLayer {
    pub name: String,
}

impl RiveObject for StateMachineLayer {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_LAYER
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

pub struct EntryState;

impl RiveObject for EntryState {
    fn type_key(&self) -> u16 {
        type_keys::ENTRY_STATE
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct ExitState;

impl RiveObject for ExitState {
    fn type_key(&self) -> u16 {
        type_keys::EXIT_STATE
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct AnyState;

impl RiveObject for AnyState {
    fn type_key(&self) -> u16 {
        type_keys::ANY_STATE
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct AnimationState {
    pub animation_id: u64,
    pub flags: u64,
}

impl AnimationState {
    pub fn new(animation_id: u64) -> Self {
        Self {
            animation_id,
            flags: 0,
        }
    }
}

impl RiveObject for AnimationState {
    fn type_key(&self) -> u16 {
        type_keys::ANIMATION_STATE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::ANIMATION_STATE_ANIMATION_ID,
            value: PropertyValue::UInt(self.animation_id),
        }];
        if self.flags != 0 {
            props.push(Property {
                key: property_keys::LAYER_STATE_FLAGS,
                value: PropertyValue::UInt(self.flags),
            });
        }
        props
    }
}

pub struct LayerState {
    pub flags: u64,
}

impl RiveObject for LayerState {
    fn type_key(&self) -> u16 {
        type_keys::LAYER_STATE
    }

    fn properties(&self) -> Vec<Property> {
        if self.flags != 0 {
            vec![Property {
                key: property_keys::LAYER_STATE_FLAGS,
                value: PropertyValue::UInt(self.flags),
            }]
        } else {
            vec![]
        }
    }
}

pub struct StateTransition {
    pub state_to_id: u64,
    pub flags: u64,
    pub duration: u64,
    pub exit_time: u64,
    pub random_weight: u64,
}

impl StateTransition {
    pub fn new(state_to_id: u64) -> Self {
        Self {
            state_to_id,
            flags: 0,
            duration: 0,
            exit_time: 0,
            random_weight: 0,
        }
    }
}

impl RiveObject for StateTransition {
    fn type_key(&self) -> u16 {
        type_keys::STATE_TRANSITION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::STATE_TRANSITION_STATE_TO_ID,
            value: PropertyValue::UInt(self.state_to_id),
        }];
        if self.flags != 0 {
            props.push(Property {
                key: property_keys::STATE_TRANSITION_FLAGS,
                value: PropertyValue::UInt(self.flags),
            });
        }
        if self.duration != 0 {
            props.push(Property {
                key: property_keys::STATE_TRANSITION_DURATION,
                value: PropertyValue::UInt(self.duration),
            });
        }
        if self.exit_time != 0 {
            props.push(Property {
                key: property_keys::STATE_TRANSITION_EXIT_TIME,
                value: PropertyValue::UInt(self.exit_time),
            });
        }
        if self.random_weight != 0 {
            props.push(Property {
                key: property_keys::STATE_TRANSITION_RANDOM_WEIGHT,
                value: PropertyValue::UInt(self.random_weight),
            });
        }
        props
    }
}

pub struct TransitionCondition;

impl RiveObject for TransitionCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionInputCondition {
    pub input_id: u64,
}

impl RiveObject for TransitionInputCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_INPUT_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID,
            value: PropertyValue::UInt(self.input_id),
        }]
    }
}

pub struct TransitionTriggerCondition {
    pub input_id: u64,
}

impl RiveObject for TransitionTriggerCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_TRIGGER_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID,
            value: PropertyValue::UInt(self.input_id),
        }]
    }
}

pub struct TransitionValueCondition {
    pub input_id: u64,
    pub op: u64,
}

impl RiveObject for TransitionValueCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            },
            Property {
                key: property_keys::TRANSITION_VALUE_CONDITION_OP,
                value: PropertyValue::UInt(self.op),
            },
        ]
    }
}

pub struct TransitionNumberCondition {
    pub input_id: u64,
    pub op: u64,
    pub value: f32,
}

impl TransitionNumberCondition {
    pub fn new(input_id: u64, op: u64, value: f32) -> Self {
        Self {
            input_id,
            op,
            value,
        }
    }
}

impl RiveObject for TransitionNumberCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_NUMBER_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            },
            Property {
                key: property_keys::TRANSITION_VALUE_CONDITION_OP,
                value: PropertyValue::UInt(self.op),
            },
            Property {
                key: property_keys::TRANSITION_NUMBER_CONDITION_VALUE,
                value: PropertyValue::Float(self.value),
            },
        ]
    }
}

pub struct TransitionBoolCondition {
    pub input_id: u64,
    pub op: u64,
}

impl TransitionBoolCondition {
    pub fn new(input_id: u64, op: u64) -> Self {
        Self { input_id, op }
    }
}

impl RiveObject for TransitionBoolCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_BOOL_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            },
            Property {
                key: property_keys::TRANSITION_VALUE_CONDITION_OP,
                value: PropertyValue::UInt(self.op),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let sm = StateMachine::new("SM".to_string(), 1);
        assert_eq!(sm.type_key(), 53);
        let props = sm.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::ANIMATION_NAME);
        assert_eq!(props[0].value, PropertyValue::String("SM".to_string()));
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_state_machine_number() {
        let n = StateMachineNumber {
            name: "speed".to_string(),
            value: 1.5,
        };
        assert_eq!(n.type_key(), 56);
        let props = n.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::STATE_MACHINE_COMPONENT_NAME);
        assert_eq!(props[0].key, 138);
        assert_eq!(props[0].value, PropertyValue::String("speed".to_string()));
        assert_eq!(props[1].key, property_keys::STATE_MACHINE_NUMBER_VALUE);
        assert_eq!(props[1].value, PropertyValue::Float(1.5));
    }

    #[test]
    fn test_state_machine_bool() {
        let b = StateMachineBool {
            name: "isRunning".to_string(),
            value: 1,
        };
        assert_eq!(b.type_key(), 59);
        let props = b.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, 138);
        assert_eq!(props[1].key, property_keys::STATE_MACHINE_BOOL_VALUE);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_state_machine_trigger() {
        let t = StateMachineTrigger {
            name: "fire".to_string(),
        };
        assert_eq!(t.type_key(), 58);
        let props = t.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, 138);
    }

    #[test]
    fn test_state_machine_layer() {
        let l = StateMachineLayer {
            name: "Layer 1".to_string(),
        };
        assert_eq!(l.type_key(), 57);
        let props = l.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, 138);
    }

    #[test]
    fn test_animation_state() {
        let s = AnimationState::new(0);
        assert_eq!(s.type_key(), 61);
        let props = s.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::ANIMATION_STATE_ANIMATION_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(0));
    }

    #[test]
    fn test_animation_state_with_flags() {
        let mut s = AnimationState::new(2);
        s.flags = 1;
        let props = s.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[1].key, property_keys::LAYER_STATE_FLAGS);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_entry_state() {
        let s = EntryState;
        assert_eq!(s.type_key(), 63);
        assert!(s.properties().is_empty());
    }

    #[test]
    fn test_exit_state() {
        let s = ExitState;
        assert_eq!(s.type_key(), 64);
        assert!(s.properties().is_empty());
    }

    #[test]
    fn test_any_state() {
        let s = AnyState;
        assert_eq!(s.type_key(), 62);
        assert!(s.properties().is_empty());
    }

    #[test]
    fn test_layer_state() {
        let s = LayerState { flags: 0 };
        assert_eq!(s.type_key(), 60);
        assert!(s.properties().is_empty());
    }

    #[test]
    fn test_state_transition() {
        let t = StateTransition::new(3);
        assert_eq!(t.type_key(), 65);
        let props = t.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::STATE_TRANSITION_STATE_TO_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(3));
    }

    #[test]
    fn test_state_transition_with_all_fields() {
        let mut t = StateTransition::new(2);
        t.flags = 1;
        t.duration = 500;
        t.exit_time = 100;
        t.random_weight = 50;
        let props = t.properties();
        assert_eq!(props.len(), 5);
        let keys: Vec<u16> = props.iter().map(|p| p.key).collect();
        assert!(keys.contains(&property_keys::STATE_TRANSITION_STATE_TO_ID));
        assert!(keys.contains(&property_keys::STATE_TRANSITION_FLAGS));
        assert!(keys.contains(&property_keys::STATE_TRANSITION_DURATION));
        assert!(keys.contains(&property_keys::STATE_TRANSITION_EXIT_TIME));
        assert!(keys.contains(&property_keys::STATE_TRANSITION_RANDOM_WEIGHT));
    }

    #[test]
    fn test_transition_condition() {
        let c = TransitionCondition;
        assert_eq!(c.type_key(), 476);
        assert!(c.properties().is_empty());
    }

    #[test]
    fn test_transition_input_condition() {
        let c = TransitionInputCondition { input_id: 0 };
        assert_eq!(c.type_key(), 67);
        let props = c.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(
            props[0].key,
            property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID
        );
    }

    #[test]
    fn test_transition_trigger_condition() {
        let c = TransitionTriggerCondition { input_id: 1 };
        assert_eq!(c.type_key(), 68);
        let props = c.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, 155);
    }

    #[test]
    fn test_transition_value_condition() {
        let c = TransitionValueCondition { input_id: 0, op: 2 };
        assert_eq!(c.type_key(), 69);
        let props = c.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, 155);
        assert_eq!(props[1].key, 156);
    }

    #[test]
    fn test_transition_number_condition() {
        let c = TransitionNumberCondition::new(0, 4, 10.0);
        assert_eq!(c.type_key(), 70);
        let props = c.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(
            props[0].key,
            property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID
        );
        assert_eq!(props[0].value, PropertyValue::UInt(0));
        assert_eq!(props[1].key, property_keys::TRANSITION_VALUE_CONDITION_OP);
        assert_eq!(props[1].value, PropertyValue::UInt(4));
        assert_eq!(
            props[2].key,
            property_keys::TRANSITION_NUMBER_CONDITION_VALUE
        );
        assert_eq!(props[2].value, PropertyValue::Float(10.0));
    }

    #[test]
    fn test_transition_bool_condition() {
        let c = TransitionBoolCondition::new(1, 0);
        assert_eq!(c.type_key(), 71);
        let props = c.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(
            props[0].key,
            property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID
        );
        assert_eq!(props[0].value, PropertyValue::UInt(1));
        assert_eq!(props[1].key, property_keys::TRANSITION_VALUE_CONDITION_OP);
        assert_eq!(props[1].value, PropertyValue::UInt(0));
    }

    #[test]
    fn test_abstract_bases() {
        let comp = StateMachineComponent {
            name: "base".to_string(),
        };
        assert_eq!(comp.type_key(), 54);
        assert_eq!(comp.properties().len(), 1);
        assert_eq!(comp.properties()[0].key, 138);

        let input = StateMachineInput {
            name: "input".to_string(),
        };
        assert_eq!(input.type_key(), 55);
        assert_eq!(input.properties().len(), 1);
        assert_eq!(input.properties()[0].key, 138);
    }
}
