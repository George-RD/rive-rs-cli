from pathlib import Path
import runpy

script_path = Path('.github/apply-authoring-stacking.py')
script = script_path.read_text()
strict = '''    if old not in text:
        raise SystemExit(f"replacement target not found: {path}")
    target.write_text(text.replace(old, new, 1))
'''
idempotent = '''    if old not in text:
        if new in text:
            return
        raise SystemExit(f"replacement target not found: {path}")
    target.write_text(text.replace(old, new, 1))
'''
if strict not in script:
    raise SystemExit('replace helper not found')
script_path.write_text(script.replace(strict, idempotent, 1))

frontend_path = Path('src/authoring/frontend.rs')
frontend = frontend_path.read_text()
before = '            visual: VisualSection {\n                nodes: vec![VisualNode::Instance {\n'
after = '            visual: VisualSection {\n                stacking: Default::default(),\n                nodes: vec![VisualNode::Instance {\n'
if before in frontend:
    frontend_path.write_text(frontend.replace(before, after, 1))
elif after not in frontend:
    raise SystemExit('VisualSection validation literal not found')

runpy.run_path(str(script_path), run_name='__main__')
