from pathlib import Path
p = Path('tests/authoring_direct_blend_contract.rs')
s = p.read_text()
if 'assert_eq!(variants.len(), 2);' in s:
    assert s.count('assert_eq!(variants.len(), 2);') == 1
    s = s.replace('assert_eq!(variants.len(), 2);', 'assert_eq!(variants.len(), 3);')
    anchor = '    assert_eq!(variants[1]["required"], json!(["motion", "binding"]));'
    assert s.count(anchor) == 1
    s = s.replace(anchor, anchor + '\n    assert_eq!(variants[2]["additionalProperties"], false);\n    assert_eq!(variants[2]["required"], json!(["motion", "weight"]));\n    assert_eq!(variants[2]["properties"]["weight"]["$ref"], "#/$defs/ScalarExpr");')
    p.write_text(s)
