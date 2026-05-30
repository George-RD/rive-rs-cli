use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct StateMachine {
    pub name: String,
}

impl StateMachine {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl RiveObject for StateMachine {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::ANIMATION_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
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

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
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

pub struct StateMachineListener {
    pub target_id: u64,
    pub listener_type_value: u64,
}

impl RiveObject for StateMachineListener {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_LISTENER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.target_id != 0 {
            props.push(Property {
                key: property_keys::LISTENER_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        if self.listener_type_value != 0 {
            props.push(Property {
                key: property_keys::LISTENER_TYPE_VALUE,
                value: PropertyValue::UInt(self.listener_type_value),
            });
        }
        props
    }
}

pub struct ListenerBoolChange {
    pub input_id: u64,
    pub value: u64,
}

impl RiveObject for ListenerBoolChange {
    fn type_key(&self) -> u16 {
        type_keys::LISTENER_BOOL_CHANGE
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::LISTENER_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            },
            Property {
                key: property_keys::LISTENER_BOOL_VALUE,
                value: PropertyValue::UInt(self.value),
            },
        ]
    }
}

pub struct NestedStateMachine {
    pub name: String,
    pub parent_id: u64,
    pub animation_id: u64,
}

pub struct Event {
    pub name: String,
    pub parent_id: u64,
}

pub struct NestedSimpleAnimation {
    pub name: String,
    pub parent_id: u64,
    pub animation_id: u64,
    pub speed: f32,
    pub is_playing: bool,
    pub mix: f32,
}

pub struct ListenerTriggerChange {
    pub input_id: u64,
}

pub struct ListenerNumberChange {
    pub input_id: u64,
    pub value: f32,
}

impl RiveObject for NestedStateMachine {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_STATE_MACHINE
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
            Property {
                key: property_keys::NESTED_ANIMATION_ID,
                value: PropertyValue::UInt(self.animation_id),
            },
        ]
    }
}

impl RiveObject for Event {
    fn type_key(&self) -> u16 {
        type_keys::EVENT
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

impl RiveObject for NestedSimpleAnimation {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_SIMPLE_ANIMATION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
            Property {
                key: property_keys::NESTED_ANIMATION_ID,
                value: PropertyValue::UInt(self.animation_id),
            },
        ];
        if self.speed != 1.0 {
            props.push(Property {
                key: property_keys::NESTED_SPEED,
                value: PropertyValue::Float(self.speed),
            });
        }
        if self.is_playing {
            props.push(Property {
                key: property_keys::NESTED_IS_PLAYING,
                value: PropertyValue::Bool(self.is_playing),
            });
        }
        if self.mix != 1.0 {
            props.push(Property {
                key: property_keys::NESTED_MIX,
                value: PropertyValue::Float(self.mix),
            });
        }
        props
    }
}

impl RiveObject for ListenerTriggerChange {
    fn type_key(&self) -> u16 {
        type_keys::LISTENER_TRIGGER_CHANGE
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::LISTENER_INPUT_ID,
            value: PropertyValue::UInt(self.input_id),
        }]
    }
}

impl RiveObject for ListenerNumberChange {
    fn type_key(&self) -> u16 {
        type_keys::LISTENER_NUMBER_CHANGE
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::LISTENER_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            },
            Property {
                key: property_keys::LISTENER_NUMBER_VALUE,
                value: PropertyValue::Float(self.value),
            },
        ]
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

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
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

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
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

pub struct BlendState;

impl RiveObject for BlendState {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_STATE
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct BlendStateDirect;

impl RiveObject for BlendStateDirect {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_STATE_DIRECT
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct BlendAnimation {
    pub animation_id: u64,
}

impl RiveObject for BlendAnimation {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_ANIMATION
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::BLEND_ANIMATION_ANIMATION_ID,
            value: PropertyValue::UInt(self.animation_id),
        }]
    }
}

pub struct BlendAnimation1D {
    pub animation_id: u64,
    pub value: f32,
}

impl RiveObject for BlendAnimation1D {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_ANIMATION_1D
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::BLEND_ANIMATION_ANIMATION_ID,
            value: PropertyValue::UInt(self.animation_id),
        }];
        if self.value != 0.0 {
            props.push(Property {
                key: property_keys::BLEND_ANIMATION_1D_VALUE,
                value: PropertyValue::Float(self.value),
            });
        }
        props
    }
}

pub struct BlendAnimationDirect {
    pub animation_id: u64,
    pub input_id: u64,
    pub mix_value: f32,
    pub blend_source: u64,
}

impl RiveObject for BlendAnimationDirect {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_ANIMATION_DIRECT
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::BLEND_ANIMATION_ANIMATION_ID,
            value: PropertyValue::UInt(self.animation_id),
        }];
        if self.input_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::BLEND_ANIMATION_DIRECT_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            });
        }
        if self.mix_value != 100.0 {
            props.push(Property {
                key: property_keys::BLEND_ANIMATION_DIRECT_MIX_VALUE,
                value: PropertyValue::Float(self.mix_value),
            });
        }
        if self.blend_source != 0 {
            props.push(Property {
                key: property_keys::BLEND_ANIMATION_DIRECT_BLEND_SOURCE,
                value: PropertyValue::UInt(self.blend_source),
            });
        }
        props
    }
}

#[allow(dead_code)] // rive-runtime type for blend state transitions
pub struct BlendStateTransition {
    pub state_to_id: u64,
    pub flags: u64,
    pub duration: u64,
    pub exit_time: u64,
    pub exit_blend_animation_id: u64,
}

impl BlendStateTransition {
    #[allow(dead_code)]
    pub fn new(state_to_id: u64) -> Self {
        Self {
            state_to_id,
            flags: 0,
            duration: 0,
            exit_time: 0,
            exit_blend_animation_id: 0,
        }
    }
}

impl RiveObject for BlendStateTransition {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_STATE_TRANSITION
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
        if self.exit_blend_animation_id != 0 {
            props.push(Property {
                key: property_keys::BLEND_STATE_TRANSITION_EXIT_BLEND_ANIMATION_ID,
                value: PropertyValue::UInt(self.exit_blend_animation_id),
            });
        }
        props
    }
}

pub struct BlendState1D;

impl RiveObject for BlendState1D {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_STATE_1D
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct BlendState1DInput {
    pub input_id: u64,
}

impl RiveObject for BlendState1DInput {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_STATE_1D_INPUT
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.input_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::BLEND_STATE_1D_INPUT_ID,
                value: PropertyValue::UInt(self.input_id),
            });
        }
        props
    }
}

pub struct TransitionPropertyComparator;

impl RiveObject for TransitionPropertyComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_PROPERTY_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionViewModelCondition {
    pub op_value: u64,
}

impl RiveObject for TransitionViewModelCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VIEW_MODEL_CONDITION
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.op_value != 0 {
            props.push(Property {
                key: property_keys::TRANSITION_VIEW_MODEL_CONDITION_OP_VALUE,
                value: PropertyValue::UInt(self.op_value),
            });
        }
        props
    }
}

pub struct TransitionValueBooleanComparator {
    pub value: bool,
}

impl RiveObject for TransitionValueBooleanComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_BOOLEAN_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::TRANSITION_VALUE_BOOLEAN_COMPARATOR_VALUE,
            value: PropertyValue::Bool(self.value),
        }]
    }
}

pub struct TransitionValueColorComparator {
    pub value: u32,
}

impl RiveObject for TransitionValueColorComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_COLOR_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::TRANSITION_VALUE_COLOR_COMPARATOR_VALUE,
            value: PropertyValue::Color(self.value),
        }]
    }
}

pub struct TransitionValueNumberComparator {
    pub value: f32,
}

impl RiveObject for TransitionValueNumberComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_NUMBER_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::TRANSITION_VALUE_NUMBER_COMPARATOR_VALUE,
            value: PropertyValue::Float(self.value),
        }]
    }
}

pub struct TransitionValueEnumComparator;

impl RiveObject for TransitionValueEnumComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_ENUM_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionValueStringComparator {
    pub value: String,
}

impl RiveObject for TransitionValueStringComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_STRING_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::TRANSITION_VALUE_STRING_COMPARATOR_VALUE,
            value: PropertyValue::String(self.value.clone()),
        }]
    }
}

pub struct TransitionValueTriggerComparator {
    pub value: u64,
}

impl RiveObject for TransitionValueTriggerComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_TRIGGER_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: property_keys::TRANSITION_VALUE_TRIGGER_COMPARATOR_VALUE,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}

pub struct OpenUrlEvent {
    pub name: String,
    pub parent_id: u64,
    pub url: String,
    pub target_value: u64,
}

impl OpenUrlEvent {
    pub fn new(name: String, parent_id: u64, url: String) -> Self {
        Self {
            name,
            parent_id,
            url,
            target_value: 0,
        }
    }
}

impl RiveObject for OpenUrlEvent {
    fn type_key(&self) -> u16 {
        type_keys::OPEN_URL_EVENT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if !self.url.is_empty() {
            props.push(Property {
                key: property_keys::OPEN_URL_EVENT_URL,
                value: PropertyValue::String(self.url.clone()),
            });
        }
        if self.target_value != 0 {
            props.push(Property {
                key: property_keys::OPEN_URL_EVENT_TARGET_VALUE,
                value: PropertyValue::UInt(self.target_value),
            });
        }
        props
    }
}

pub struct AudioEvent {
    pub name: String,
    pub parent_id: u64,
    pub asset_id: u64,
}

impl AudioEvent {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            asset_id: u32::MAX as u64,
        }
    }
}

impl RiveObject for AudioEvent {
    fn type_key(&self) -> u16 {
        type_keys::AUDIO_EVENT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.asset_id != u32::MAX as u64 {
            props.push(Property {
                key: property_keys::AUDIO_EVENT_ASSET_ID,
                value: PropertyValue::UInt(self.asset_id),
            });
        }
        props
    }
}

pub struct CustomPropertyNumber {
    pub name: String,
    pub parent_id: u64,
    pub property_value: f32,
}

impl CustomPropertyNumber {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            property_value: 0.0,
        }
    }
}

impl RiveObject for CustomPropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_NUMBER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.property_value != 0.0 {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_NUMBER_PROPERTY_VALUE,
                value: PropertyValue::Float(self.property_value),
            });
        }
        props
    }
}

pub struct CustomPropertyBoolean {
    pub name: String,
    pub parent_id: u64,
    pub property_value: u64,
}

impl CustomPropertyBoolean {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            property_value: 0,
        }
    }
}

impl RiveObject for CustomPropertyBoolean {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_BOOLEAN
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_BOOLEAN_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct CustomPropertyString {
    pub name: String,
    pub parent_id: u64,
    pub property_value: String,
}

impl CustomPropertyString {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            property_value: String::new(),
        }
    }
}

impl RiveObject for CustomPropertyString {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_STRING
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if !self.property_value.is_empty() {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_STRING_PROPERTY_VALUE,
                value: PropertyValue::String(self.property_value.clone()),
            });
        }
        props
    }
}

pub struct CustomPropertyColor {
    pub name: String,
    pub parent_id: u64,
    pub property_value: u32,
}

impl CustomPropertyColor {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            property_value: 0xFF000000,
        }
    }
}

impl RiveObject for CustomPropertyColor {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_COLOR
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.property_value != 0x00000000 {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_COLOR_PROPERTY_VALUE,
                value: PropertyValue::Color(self.property_value),
            });
        }
        props
    }
}

pub struct CustomPropertyTrigger {
    pub name: String,
    pub parent_id: u64,
    pub property_value: u64,
}

impl CustomPropertyTrigger {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            property_value: 0,
        }
    }
}

impl RiveObject for CustomPropertyTrigger {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_TRIGGER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_TRIGGER_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}

pub struct CustomPropertyEnum {
    pub name: String,
    pub parent_id: u64,
    pub property_value: u64,
    pub enum_id: u64,
}

impl CustomPropertyEnum {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            property_value: 0,
            enum_id: 0,
        }
    }
}

impl RiveObject for CustomPropertyEnum {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_ENUM
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_ENUM_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        if self.enum_id != 0 {
            props.push(Property {
                key: property_keys::CUSTOM_PROPERTY_ENUM_ENUM_ID,
                value: PropertyValue::UInt(self.enum_id),
            });
        }
        props
    }
}

pub struct CustomPropertyGroup {
    pub name: String,
    pub parent_id: u64,
}

impl CustomPropertyGroup {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self { name, parent_id }
    }
}

impl RiveObject for CustomPropertyGroup {
    fn type_key(&self) -> u16 {
        type_keys::CUSTOM_PROPERTY_GROUP
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}

pub struct ListenerAlignTarget {
    pub target_id: u64,
}

impl RiveObject for ListenerAlignTarget {
    fn type_key(&self) -> u16 {
        type_keys::LISTENER_ALIGN_TARGET
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.target_id != 0 {
            props.push(Property {
                key: property_keys::LISTENER_ALIGN_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        props
    }
}

pub struct ListenerFireEvent {
    pub event_id: u64,
}

impl RiveObject for ListenerFireEvent {
    fn type_key(&self) -> u16 {
        type_keys::LISTENER_FIRE_EVENT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.event_id != 0 {
            props.push(Property {
                key: property_keys::LISTENER_FIRE_EVENT_EVENT_ID,
                value: PropertyValue::UInt(self.event_id),
            });
        }
        props
    }
}

pub struct ListenerViewModelChange {
    pub view_model_property_id: u64,
}

impl RiveObject for ListenerViewModelChange {
    fn type_key(&self) -> u16 {
        type_keys::LISTENER_VIEW_MODEL_CHANGE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        props
    }
}

#[allow(dead_code)]
pub struct StateMachineFireEvent {
    pub name: String,
    pub event_id: u64,
    pub occurs_value: u64,
}

#[allow(dead_code)]
impl StateMachineFireEvent {
    pub fn new(name: String) -> Self {
        Self {
            name,
            event_id: 0,
            occurs_value: 0,
        }
    }
}

impl RiveObject for StateMachineFireEvent {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_FIRE_EVENT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if self.event_id != 0 {
            props.push(Property {
                key: property_keys::STATE_MACHINE_FIRE_EVENT_EVENT_ID,
                value: PropertyValue::UInt(self.event_id),
            });
        }
        if self.occurs_value != 0 {
            props.push(Property {
                key: property_keys::STATE_MACHINE_FIRE_EVENT_OCCURS_VALUE,
                value: PropertyValue::UInt(self.occurs_value),
            });
        }
        props
    }
}

#[allow(dead_code)]
pub struct StateMachineFireTrigger {
    pub name: String,
}

impl RiveObject for StateMachineFireTrigger {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_FIRE_TRIGGER
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

#[allow(dead_code)]
pub struct StateMachineFireAction {
    pub name: String,
}

impl RiveObject for StateMachineFireAction {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_FIRE_ACTION
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

#[allow(dead_code)]
pub struct StateMachineComponentNestedArtboard {
    pub name: String,
    pub artboard_id: u64,
}

impl RiveObject for StateMachineComponentNestedArtboard {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_COMPONENT_NESTED_ARTBOARD
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if self.artboard_id != 0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            });
        }
        props
    }
}

#[allow(dead_code)]
pub struct StateMachineNestedInput {
    pub name: String,
    pub nested_input_id: u64,
}

impl RiveObject for StateMachineNestedInput {
    fn type_key(&self) -> u16 {
        type_keys::STATE_MACHINE_NESTED_INPUT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::STATE_MACHINE_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if self.nested_input_id != 0 {
            props.push(Property {
                key: property_keys::NESTED_INPUT_ID,
                value: PropertyValue::UInt(self.nested_input_id),
            });
        }
        props
    }
}

#[allow(dead_code)]
pub struct BlendState1DViewModel;

impl RiveObject for BlendState1DViewModel {
    fn type_key(&self) -> u16 {
        type_keys::BLEND_STATE_1D_VIEW_MODEL
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionPropertyViewModelComparator;

impl RiveObject for TransitionPropertyViewModelComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_PROPERTY_VIEW_MODEL_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionPropertyArtboardComparator;

impl RiveObject for TransitionPropertyArtboardComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_PROPERTY_ARTBOARD_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionArtboardCondition {
    pub op_value: u64,
}

impl RiveObject for TransitionArtboardCondition {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_ARTBOARD_CONDITION
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.op_value != 0 {
            props.push(Property {
                key: property_keys::TRANSITION_VIEW_MODEL_CONDITION_OP_VALUE,
                value: PropertyValue::UInt(self.op_value),
            });
        }
        props
    }
}

pub struct TransitionSelfComparator;

impl RiveObject for TransitionSelfComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_SELF_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TransitionValueIdComparator {
    pub value: u64,
}

impl RiveObject for TransitionValueIdComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_ID_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: property_keys::TRANSITION_VALUE_ID_COMPARATOR_VALUE,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}

pub struct TransitionValueAssetComparator {
    pub value: u64,
}

impl RiveObject for TransitionValueAssetComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_ASSET_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: property_keys::TRANSITION_VALUE_ASSET_COMPARATOR_VALUE,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}

pub struct TransitionValueArtboardComparator {
    pub value: u64,
}

impl RiveObject for TransitionValueArtboardComparator {
    fn type_key(&self) -> u16 {
        type_keys::TRANSITION_VALUE_ARTBOARD_COMPARATOR
    }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: property_keys::TRANSITION_VALUE_ARTBOARD_COMPARATOR_VALUE,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let sm = StateMachine::new("SM".to_string());
        assert_eq!(sm.type_key(), 53);
        let props = sm.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::ANIMATION_NAME);
        assert_eq!(props[0].value, PropertyValue::String("SM".to_string()));
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
    fn test_state_machine_listener_properties_default_omission() {
        let listener = StateMachineListener {
            target_id: 0,
            listener_type_value: 0,
        };
        assert_eq!(listener.type_key(), type_keys::STATE_MACHINE_LISTENER);
        assert!(listener.properties().is_empty());
    }

    #[test]
    fn test_state_machine_listener_properties() {
        let listener = StateMachineListener {
            target_id: 3,
            listener_type_value: 1,
        };
        let props = listener.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::LISTENER_TARGET_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(3));
        assert_eq!(props[1].key, property_keys::LISTENER_TYPE_VALUE);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_listener_bool_change_properties() {
        let action = ListenerBoolChange {
            input_id: 2,
            value: 1,
        };
        assert_eq!(action.type_key(), type_keys::LISTENER_BOOL_CHANGE);
        let props = action.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::LISTENER_INPUT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(2));
        assert_eq!(props[1].key, property_keys::LISTENER_BOOL_VALUE);
        assert_eq!(props[1].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_nested_state_machine_properties() {
        let nested = NestedStateMachine {
            name: "NestedSM".to_string(),
            parent_id: 5,
            animation_id: 9,
        };
        assert_eq!(nested.type_key(), type_keys::NESTED_STATE_MACHINE);
        let props = nested.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(
            props[0].value,
            PropertyValue::String("NestedSM".to_string())
        );
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(5));
        assert_eq!(props[2].key, property_keys::NESTED_ANIMATION_ID);
        assert_eq!(props[2].value, PropertyValue::UInt(9));
    }

    #[test]
    fn test_event_properties() {
        let event = Event {
            name: "Evt".to_string(),
            parent_id: 4,
        };
        assert_eq!(event.type_key(), type_keys::EVENT);
        let props = event.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(4));
    }

    #[test]
    fn test_nested_simple_animation_properties_defaults() {
        let nested = NestedSimpleAnimation {
            name: "NestedSimple".to_string(),
            parent_id: 5,
            animation_id: 9,
            speed: 1.0,
            is_playing: false,
            mix: 1.0,
        };
        assert_eq!(nested.type_key(), type_keys::NESTED_SIMPLE_ANIMATION);
        let props = nested.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[2].key, property_keys::NESTED_ANIMATION_ID);
    }

    #[test]
    fn test_nested_simple_animation_properties_custom() {
        let nested = NestedSimpleAnimation {
            name: "NestedSimple".to_string(),
            parent_id: 5,
            animation_id: 9,
            speed: 2.0,
            is_playing: true,
            mix: 0.25,
        };
        let props = nested.properties();
        assert_eq!(props.len(), 6);
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::NESTED_SPEED
                    && p.value == PropertyValue::Float(2.0))
        );
        assert!(props.iter().any(|p| {
            p.key == property_keys::NESTED_IS_PLAYING && p.value == PropertyValue::Bool(true)
        }));
        assert!(
            props.iter().any(
                |p| p.key == property_keys::NESTED_MIX && p.value == PropertyValue::Float(0.25)
            )
        );
    }

    #[test]
    fn test_listener_trigger_change_properties() {
        let action = ListenerTriggerChange { input_id: 2 };
        assert_eq!(action.type_key(), type_keys::LISTENER_TRIGGER_CHANGE);
        let props = action.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::LISTENER_INPUT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_listener_number_change_properties() {
        let action = ListenerNumberChange {
            input_id: 2,
            value: 3.5,
        };
        assert_eq!(action.type_key(), type_keys::LISTENER_NUMBER_CHANGE);
        let props = action.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::LISTENER_INPUT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(2));
        assert_eq!(props[1].key, property_keys::LISTENER_NUMBER_VALUE);
        assert_eq!(props[1].value, PropertyValue::Float(3.5));
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
    fn test_blend_animation_direct_preserves_zero_input_id() {
        let animation = BlendAnimationDirect {
            animation_id: 3,
            input_id: 0,
            mix_value: 100.0,
            blend_source: 0,
        };
        let props = animation.properties();
        assert!(props.iter().any(|property| {
            property.key == property_keys::BLEND_ANIMATION_DIRECT_INPUT_ID
                && property.value == PropertyValue::UInt(0)
        }));
    }

    #[test]
    fn test_blend_state_1d_has_no_properties() {
        let state = BlendState1D;
        assert!(state.properties().is_empty());
    }

    #[test]
    fn test_blend_state_1d_input_preserves_zero_input_id() {
        let state = BlendState1DInput { input_id: 0 };
        let props = state.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::BLEND_STATE_1D_INPUT_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(0));
    }

    #[test]
    fn test_blend_state_1d_input_omits_unset_input_id() {
        let state = BlendState1DInput {
            input_id: u32::MAX as u64,
        };
        assert!(state.properties().is_empty());
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

    #[test]
    fn test_listener_align_target_type_key() {
        let obj = ListenerAlignTarget { target_id: 5 };
        assert_eq!(obj.type_key(), 126);
    }

    #[test]
    fn test_listener_align_target_properties() {
        let obj = ListenerAlignTarget { target_id: 3 };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::LISTENER_ALIGN_TARGET_ID);
        assert_eq!(props[0].value, PropertyValue::UInt(3));
    }

    #[test]
    fn test_listener_align_target_zero_omitted() {
        let obj = ListenerAlignTarget { target_id: 0 };
        assert!(obj.properties().is_empty());
    }

    #[test]
    fn test_listener_fire_event_type_key() {
        let obj = ListenerFireEvent { event_id: 1 };
        assert_eq!(obj.type_key(), 168);
    }

    #[test]
    fn test_listener_fire_event_properties() {
        let obj = ListenerFireEvent { event_id: 7 };
        let props = obj.properties();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, property_keys::LISTENER_FIRE_EVENT_EVENT_ID);
    }

    #[test]
    fn test_listener_view_model_change_type_key() {
        let obj = ListenerViewModelChange {
            view_model_property_id: 1,
        };
        assert_eq!(obj.type_key(), 487);
    }

    #[test]
    fn test_state_machine_fire_event_type_key() {
        let obj = StateMachineFireEvent::new("fire".to_string());
        assert_eq!(obj.type_key(), 169);
    }

    #[test]
    fn test_state_machine_fire_event_properties() {
        let mut obj = StateMachineFireEvent::new("fire".to_string());
        obj.event_id = 2;
        obj.occurs_value = 1;
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::STATE_MACHINE_COMPONENT_NAME);
        assert_eq!(
            props[1].key,
            property_keys::STATE_MACHINE_FIRE_EVENT_EVENT_ID
        );
        assert_eq!(
            props[2].key,
            property_keys::STATE_MACHINE_FIRE_EVENT_OCCURS_VALUE
        );
    }

    #[test]
    fn test_state_machine_fire_trigger_type_key() {
        let obj = StateMachineFireTrigger {
            name: "trig".to_string(),
        };
        assert_eq!(obj.type_key(), 614);
        assert_eq!(obj.properties().len(), 1);
    }

    #[test]
    fn test_state_machine_fire_action_type_key() {
        let obj = StateMachineFireAction {
            name: "action".to_string(),
        };
        assert_eq!(obj.type_key(), 615);
        assert_eq!(obj.properties().len(), 1);
    }

    #[test]
    fn test_sm_component_nested_artboard_type_key() {
        let obj = StateMachineComponentNestedArtboard {
            name: "nested".to_string(),
            artboard_id: 1,
        };
        assert_eq!(obj.type_key(), 172);
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[1].key, property_keys::NESTED_ARTBOARD_ARTBOARD_ID);
    }

    #[test]
    fn test_sm_nested_input_type_key() {
        let obj = StateMachineNestedInput {
            name: "ni".to_string(),
            nested_input_id: 5,
        };
        assert_eq!(obj.type_key(), 173);
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[1].key, property_keys::NESTED_INPUT_ID);
    }

    #[test]
    fn test_blend_state_1d_view_model_type_key() {
        let obj = BlendState1DViewModel;
        assert_eq!(obj.type_key(), 528);
        assert!(obj.properties().is_empty());
    }
}
