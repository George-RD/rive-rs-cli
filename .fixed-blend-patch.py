from pathlib import Path

root = Path('.')
p = root / 'src/authoring/spec.rs'
s = p.read_text()
if 'Weight { motion: String, weight: ScalarExpr }' in s:
    raise SystemExit(0)
old = '''pub enum BehaviorDirectBlendMotionSpec {
    Input { motion: String, input: String },
    Binding { motion: String, binding: String },
}

impl BehaviorDirectBlendMotionSpec {
    pub(crate) fn motion(&self) -> &str {
        match self {
            Self::Input { motion, .. } | Self::Binding { motion, .. } => motion,
        }
    }

    pub(crate) fn source_id(&self) -> &str {
        match self {
            Self::Input { input, .. } => input,
            Self::Binding { binding, .. } => binding,
        }
    }

    pub(crate) fn binding(&self) -> Option<&str> {
        match self {
            Self::Input { .. } => None,
            Self::Binding { binding, .. } => Some(binding),
        }
    }
}'''
new = '''pub enum BehaviorDirectBlendMotionSpec {
    Input { motion: String, input: String },
    Binding { motion: String, binding: String },
    Weight { motion: String, weight: ScalarExpr },
}

impl BehaviorDirectBlendMotionSpec {
    pub(crate) fn motion(&self) -> &str {
        match self {
            Self::Input { motion, .. }
            | Self::Binding { motion, .. }
            | Self::Weight { motion, .. } => motion,
        }
    }

    pub(crate) fn binding(&self) -> Option<&str> {
        match self {
            Self::Input { .. } | Self::Weight { .. } => None,
            Self::Binding { binding, .. } => Some(binding),
        }
    }
}'''
assert s.count(old) == 1
p.write_text(s.replace(old, new))
p = root / 'src/authoring/frontend/compiler/behavior.rs'
s = p.read_text()
s = s.replace('AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorInputKind, BehaviorInputSpec,', 'AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorDirectBlendMotionSpec, BehaviorInputKind, BehaviorInputSpec,')
old = '''                let children = blend
                    .motions
                    .iter()
                    .map(|motion| {
                        let animation_name = animation_runtime_name(spec, motion.motion());
                        json!({
                            "type": "blend_animation_direct",
                            "animation_id": animation_index_by_name.get(animation_name.as_str())
                                .expect("validated motion has a lowered animation"),
                            "input_id": input_index_by_id.get(motion.source_id())
                                .expect("validated direct blend input")
                        })
                    })
                    .collect::<Vec<_>>();'''
new = '''                let children = blend
                    .motions
                    .iter()
                    .enumerate()
                    .map(|(index, motion)| {
                        let animation_name = animation_runtime_name(spec, motion.motion());
                        let mut child = json!({
                            "type": "blend_animation_direct",
                            "animation_id": animation_index_by_name.get(animation_name.as_str())
                                .expect("validated motion has a lowered animation")
                        });
                        match motion {
                            BehaviorDirectBlendMotionSpec::Input { input, .. }
                            | BehaviorDirectBlendMotionSpec::Binding { binding: input, .. } => {
                                child["input_id"] = json!(input_index_by_id.get(input.as_str())
                                    .expect("validated direct blend input"));
                            }
                            BehaviorDirectBlendMotionSpec::Weight { weight, .. } => {
                                child["blend_source"] = json!(DIRECT_BLEND_SOURCE_FIXED);
                                child["mix_value"] = json!(evaluate_expression(
                                    weight,
                                    &format!("{state_path}.direct_blend.motions[{index}].weight"),
                                    &spec.parameters,
                                    Unit::Scalar,
                                )?);
                            }
                        }
                        Ok(child)
                    })
                    .collect::<Result<Vec<_>, AuthoringDiagnostic>>()?;'''
assert s.count(old) == 1
s = s.replace(old, new)
old = '''            context.validate_blend_source(motion.source_id(), motion.binding(), &path, diagnostics);'''
new = '''            match motion {
                BehaviorDirectBlendMotionSpec::Input { input, .. } => {
                    context.validate_blend_source(input, None, &path, diagnostics);
                }
                BehaviorDirectBlendMotionSpec::Binding { binding, .. } => {
                    context.validate_blend_source(binding, Some(binding), &path, diagnostics);
                }
                BehaviorDirectBlendMotionSpec::Weight { weight, .. } => {
                    if let Err(error) = evaluate_expression(
                        weight,
                        &format!("{path}.weight"),
                        parameters,
                        Unit::Scalar,
                    ) {
                        diagnostics.push(error);
                    }
                }
            }'''
assert s.count(old) == 1
s = s.replace(old, new)
s = s.replace('use super::MotionTargetIndex;', 'use super::MotionTargetIndex;\n\nconst DIRECT_BLEND_SOURCE_FIXED: u64 = 1;')
p.write_text(s)
