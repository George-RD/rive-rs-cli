from pathlib import Path

path = Path('.github/workflows/ci.yml')
text = path.read_text()
before = '''      - run: npx playwright install --with-deps chromium
      - name: Run typed behavior runtime contract
'''
after = '''      - run: npx playwright install --with-deps chromium
      - name: Run AuthoringSpec stacking runtime contract
        run: |
          RIVE_CHROME="$(node --input-type=module -e 'import { chromium } from "playwright"; process.stdout.write(chromium.executablePath())')" \\
            cargo test --locked --test authoring_stacking_runtime -- --ignored
      - name: Run typed behavior runtime contract
'''
if before in text:
    path.write_text(text.replace(before, after, 1))
elif after not in text:
    raise SystemExit('Playwright insertion seam not found')
