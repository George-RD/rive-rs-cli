#![no_main]

use libfuzzer_sys::fuzz_target;
use rive_cli::validator::{parse_riv, InspectFilter};

fuzz_target!(|data: &[u8]| {
    let _ = parse_riv(data, &InspectFilter::default());
});
