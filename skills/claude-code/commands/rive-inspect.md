Inspect a Rive .riv binary file and display its object tree.

Usage: /rive-inspect <file.riv> [--json] [--type-key N] [--property-key N]

Steps:
1. Run: cargo run -- inspect <file.riv> [flags]
2. Display the object tree to the user
3. If --json flag used, output is structured JSON

Options:
- --json: Output as JSON instead of human-readable tree
- --type-key N: Filter to objects of this type key
- --type-name NAME: Filter by type name
- --object-index N: Show specific object by index
- --property-key N: Filter to properties with this key
