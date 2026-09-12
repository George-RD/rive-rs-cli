from hashlib import sha1
from pathlib import Path
import re

path = Path("src/builder/validation.rs")
data = path.read_bytes()
expected = "80779b359b9bfa622b74cd8d284f73ed1ef10f50"
actual = sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()
if actual != expected:
    raise RuntimeError(f"refusing to patch unexpected validator blob {actual}")
text = data.decode()
pattern = re.compile(r"\bvalidate_object_spec\(([^()]*)\)")
occurrences = list(pattern.finditer(text))
replacements = []
root_calls = 0
nested_calls = 0
definitions = 0
for match in occurrences:
    args = [arg.strip() for arg in match.group(1).split(",") if arg.strip()]
    if len(args) != 3:
        raise RuntimeError(f"unexpected validator argument list: {args!r}")
    if args[0] == "spec: &ObjectSpec":
        extra = "is_artboard_child: bool"
        definitions += 1
    elif args == ["child", "&mut object_names", "&ParentKind::Artboard"]:
        extra = "true"
        root_calls += 1
    elif args[1] == "object_names":
        extra = "false"
        nested_calls += 1
    else:
        raise RuntimeError(f"unrecognized validator call: {args!r}")
    replacement = "validate_object_spec(" + ", ".join(args + [extra]) + ")"
    replacements.append((match.start(), match.end(), replacement))
if definitions != 1 or root_calls != 1 or nested_calls == 0:
    raise RuntimeError(f"unexpected validator inventory: {definitions=} {root_calls=} {nested_calls=}")
for start, end, replacement in reversed(replacements):
    text = text[:start] + replacement + text[end:]
function_start = text.index("pub(crate) fn validate_object_spec(")
body_start = text.index(") -> Result<(), String> {", function_start) + len(") -> Result<(), String> {")
text = text[:body_start] + '''
    if !is_artboard_child
        && let Some((name, _)) = super::objects::file_asset(spec)
    {
        return Err(format!("asset '{name}' must be a direct child of an artboard"));
    }
''' + text[body_start:]
path.write_text(text)
print(f"Patched the canonical validator: {root_calls} artboard entry and {nested_calls} recursive calls.")
Path(".agent-257/nested-test.rs").unlink()
Path(".agent-257/review-fixes.py").unlink()
