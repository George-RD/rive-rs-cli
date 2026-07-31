from pathlib import Path

path = Path("src/authoring/lower/path.rs")
text = path.read_text()
old = "#[derive(Clone, Copy)]\npub(super) struct PathPlacement"
new = "#[derive(Debug, Clone, Copy)]\npub(super) struct PathPlacement"
if text.count(old) != 1:
    raise RuntimeError("expected one PathPlacement derive")
path.write_text(text.replace(old, new, 1))
