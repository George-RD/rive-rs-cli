from pathlib import Path

path = Path('tests/authoring_model_blend_1d_contract.rs')
text = path.read_text()
path.write_text(text.replace('PropertyValueRead::Bytes(vec![0, 0])', 'PropertyValueRead::Bytes { length: 2 }'))
