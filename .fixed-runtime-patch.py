from pathlib import Path
import json

root = Path('.')
example = root / 'examples/authoring/fixed-blend-panel.v0.json'
if not example.exists():
    document = json.loads((root / 'examples/authoring/direct-blend-panel.v0.json').read_text())
    document['artboard']['id'] = 'fixed-blend-panel'
    chart = document['behavior']['statecharts'][0]
    chart['inputs'] = [item for item in chart['inputs'] if item['id'] != 'foundation']
    chart['states'][0]['direct_blend']['motions'][0] = {
        'motion': 'rest-track',
        'weight': {'kind': 'literal', 'value': 100, 'unit': 'scalar'},
    }
    example.write_text(json.dumps(document, indent=2) + '\n')

p = root / 'tests/playwright/authoring-direct-blend-runtime.js'
s = p.read_text()
if 'const FIXED_WEIGHTS' not in s:
    s = s.replace('const ONE_DIMENSIONAL =', 'const FIXED_WEIGHTS = process.argv.includes("--fixed-weights");\nconst ONE_DIMENSIONAL =')
    s = s.replace('const MODE = ONE_DIMENSIONAL ? "model-blend-1d" : MODEL_BOUND ? "model-blend" : "direct-blend";', '''assert.ok(!FIXED_WEIGHTS || !ONE_DIMENSIONAL, "fixed weights require direct blends");
const BASE_MODE = ONE_DIMENSIONAL ? "model-blend-1d" : MODEL_BOUND ? "model-blend" : "direct-blend";
const MODE = FIXED_WEIGHTS ? `${MODEL_BOUND ? "model-" : ""}fixed-blend` : BASE_MODE;''')
    s = s.replace('`${MODE}-panel.v0.json`', '`${FIXED_WEIGHTS && !MODEL_BOUND ? "fixed-blend" : BASE_MODE}-panel.v0.json`')
    old = '''function buildFixture() {
  fs.mkdirSync(DIRECTORY, { recursive: true });
  const binary = path.join(DIRECTORY, `${FIXTURE}.riv`);
  const compiled = spawnSync("cargo", [
    "run", "--quiet", "--", "authoring", "compile", SOURCE, "-o", binary, "--json",
  ], { cwd: ROOT, encoding: "utf8" });'''
    new = '''function buildFixture({ label = "dynamic", weights = null, restLast = false } = {}) {
  const directory = label === "dynamic" ? DIRECTORY : path.join(DIRECTORY, label);
  const fixture = label === "dynamic" ? FIXTURE : `${FIXTURE}_${label}`;
  fs.mkdirSync(directory, { recursive: true });
  const source = path.join(directory, "authoring.json");
  const document = JSON.parse(fs.readFileSync(SOURCE, "utf8"));
  if (FIXED_WEIGHTS) {
    const chart = document.behavior.statecharts[0];
    chart.inputs = chart.inputs.filter((input) => input.id !== "foundation");
    const motions = chart.states[0].direct_blend.motions;
    motions[0] = { motion: "rest-track", weight: { kind: "literal", value: 100, unit: "scalar" } };
    if (weights) {
      assert.equal(MODEL_BOUND, false, "constant cases have no model host");
      chart.inputs = chart.inputs.filter((input) => input.kind === "trigger");
      document.parameters = { ...document.parameters,
        left: { value: weights.left, unit: "scalar" },
        right: { value: weights.right, unit: "scalar" },
      };
      motions[1] = { motion: "left-track", weight: { kind: "parameter", name: "left" } };
      motions[2] = { motion: "right-track", weight: { kind: "parameter", name: "right" } };
      if (restLast) motions.push(motions.shift());
    }
  }
  fs.writeFileSync(source, `${JSON.stringify(document, null, 2)}\\n`);
  const binary = path.join(directory, `${fixture}.riv`);
  const compiled = spawnSync("cargo", [
    "run", "--quiet", "--", "authoring", "compile", source, "-o", binary, "--json",
  ], { cwd: ROOT, encoding: "utf8" });'''
    assert s.count(old) == 1
    s = s.replace(old, new)
    s = s.replace('fs.writeFileSync(path.join(DIRECTORY, "compile.json"), compiled.stdout);\n  fs.copyFileSync(SOURCE, path.join(DIRECTORY, "authoring.json"));\n  fs.copyFileSync(binary, path.join(HARNESS_DIR, `${FIXTURE}.riv`));', 'fs.writeFileSync(path.join(directory, "compile.json"), compiled.stdout);\n  fs.copyFileSync(binary, path.join(HARNESS_DIR, `${fixture}.riv`));')
    s = s.replace('fixture: FIXTURE,', 'fixture, weights, restLast,\n    sourceSha256: sha256(fs.readFileSync(source)),')
    s = s.replace('left: name(MODEL_BOUND ? "left-model" : "panel/left-weight"),', 'left: weights ? null : name(MODEL_BOUND ? "left-model" : "panel/left-weight"),')
    s = s.replace('right: name(MODEL_BOUND ? "right-model" : "panel/right-weight"),', 'right: weights ? null : name(MODEL_BOUND ? "right-model" : "panel/right-weight"),')
    s = s.replace('const leftInput = find(plan.left);\n    const rightInput = find(plan.right);', 'const leftInput = plan.left ? find(plan.left) : null;\n    const rightInput = plan.right ? find(plan.right) : null;\n    if (plan.weights && inputs.length !== 2) throw new Error("fixed weights introduced extra controls");')
    for item in ['left', 'right', 'leftInput', 'rightInput']:
        s = s.replace(f'window.__DIRECT_BLEND.{item}.value,', f'window.__DIRECT_BLEND.{item}?.value ?? null,')
    old = '''    page = await openFixturePage(browser, PORT, FIXTURE, {
      pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 1 },
    });
    page.on("pageerror", (error) => errors.push(String(error)));
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
    await mount(page, plan);'''
    assert s.count(old) == 1
    s = s.replace(old, '    page = await openControlledFixture(browser, plan, errors);')
    anchor = 'async function main() {'
    helper = '''async function openControlledFixture(browser, plan, errors) {
  const page = await openFixturePage(browser, PORT, plan.fixture, {
    pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 1 },
  });
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  try {
    await mount(page, plan);
    return page;
  } catch (error) {
    await page.close();
    throw error;
  }
}

'''
    s = s.replace(anchor, helper + anchor)
    s = s.replace('  const plan = buildFixture();', '  const plan = buildFixture();\n  const fixedPlans = [];')
    anchor = '    assert.deepEqual(errors, []);'
    extra = '''    if (FIXED_WEIGHTS && !MODEL_BOUND) {
      for (const [label, left, right, expected, restLast] of [
        ["fixed-zero", 0, 0, { left: 40, right: 40 }, false],
        ["fixed-fractions", 25.5, 75.5, { left: 80.8, right: 160.8 }, false],
        ["fixed-half", 50, 50, { left: 120, right: 120 }, false],
        ["fixed-full", 100, 100, { left: 200, right: 200 }, false],
        ["fixed-reversed", 75.5, 25.5, { left: 160.8, right: 80.8 }, false],
        ["fixed-rest-last", 25.5, 75.5, { left: 40, right: 40 }, true],
      ]) {
        const fixed = buildFixture({ label, weights: { left, right }, restLast });
        fixedPlans.push(fixed);
        await page.close();
        page = await openControlledFixture(browser, fixed, errors);
        samples.push(await sample(page, label, expected));
        assert.equal(samples.at(-1).leftWeight, null);
        assert.equal(samples.at(-1).rightWeight, null);
        if (label === "fixed-fractions") {
          await page.evaluate(() => window.__DIRECT_BLEND.reset.fire());
          samples.push(await sample(page, "fixed-reset", { left: 40, right: 40 }));
          await page.evaluate(() => window.__DIRECT_BLEND.resume.fire());
          samples.push(await sample(page, "fixed-resume", expected));
        }
      }
    }
'''
    s = s.replace(anchor, extra + anchor)
    s = s.replace('      plan, samples,', '      plan, fixedPlans, samples,')
    s = s.replace('    cleanupFixtures([FIXTURE]);', '    cleanupFixtures([FIXTURE, ...fixedPlans.map((plan) => plan.fixture)]);')
    p.write_text(s)

p = root / 'cairn.blueprint'
s = p.read_text()
if 'authoring_fixed_blend_contract.rs' not in s:
    s = s.replace('                "./tests/authoring_font_asset_contract.rs",', '                "./tests/authoring_fixed_blend_contract.rs",\n                "./tests/authoring_font_asset_contract.rs",')
    p.write_text(s)

p = root / 'examples/authoring/README.md'
s = p.read_text()
if '`fixed-blend-panel.v0.json`' not in s:
    s = s.replace('| `blend-meter.v0.json`', '| `fixed-blend-panel.v0.json` | Constant rest weight with independently controlled direct-blend motions; no artificial foundation input |\n| `blend-meter.v0.json`')
    p.write_text(s)
