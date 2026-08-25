use std::{env, fs, path::Path};

use rive_cli::{
    authoring::lower_authoring_json,
    builder::{SceneSpec, build_scene},
    encoder::encode_riv,
    objects::core::RiveObject,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or("usage: compile_authoring INPUT OUTPUT")?;
    let output = args.next().ok_or("usage: compile_authoring INPUT OUTPUT")?;

    let source = fs::read_to_string(&input)?;
    let lowered = lower_authoring_json(&source).map_err(|error| {
        std::io::Error::other(format!("authoring diagnostics: {:#?}", error.diagnostics))
    })?;
    let spec: SceneSpec = serde_json::from_value(lowered.scene)?;
    let base_dir = Path::new(&input).parent().unwrap_or_else(|| Path::new("."));
    let scene = build_scene(&spec, Some(base_dir))?;
    let refs: Vec<&dyn RiveObject> = scene.iter().map(|object| &**object).collect();
    let bytes = encode_riv(&refs, 0);
    fs::write(&output, &bytes)?;
    eprintln!("wrote {} bytes to {}", bytes.len(), output);
    Ok(())
}
