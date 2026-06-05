const fs = require("node:fs");
const path = require("node:path");
const { ROOT } = require("./shared");

const SCREENSHOT_DIR = path.join(ROOT, "target", "playwright-vision");

async function callVisionModel(apiKey, referencePath, generatedPath) {
  const refBase64 = fs.readFileSync(referencePath).toString("base64");
  const genBase64 = fs.readFileSync(generatedPath).toString("base64");

  const response = await fetch("https://api.openai.com/v1/chat/completions", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${apiKey}`,
    },
    body: JSON.stringify({
      model: "gpt-4o",
      messages: [
        {
          role: "system",
          content:
            "You are a visual QA judge. Compare two images of Rive animations/graphics. Determine if they are semantically similar enough to pass an approval gate. Respond with JSON: {\"pass\": boolean, \"score\": number (0-100), \"reason\": string}";
        },
        {
          role: "user",
          content: [
            {
              type: "text",
              text: "Compare these two images. Image 1 is the reference. Image 2 is the generated version. Are they semantically similar? Consider: same objects, same colors, same layout. Minor differences in exact pixel values are acceptable. Missing animations or state machine behavior is a failure.",
            },
            {
              type: "image_url",
              image_url: {
                url: `data:image/png;base64,${refBase64}`,
              },
            },
            {
              type: "image_url",
              image_url: {
                url: `data:image/png;base64,${genBase64}`,
              },
            },
          ],
        },
      ],
      max_tokens: 500,
      response_format: { type: "json_object" },
    }),
  });

  if (!response.ok) {
    throw new Error(`Vision API error: ${response.status} ${await response.text()}`);
  }

  const data = await response.json();
  const content = data.choices[0].message.content;
  return JSON.parse(content);
}

async function main() {
  const apiKey = process.env.OPENAI_API_KEY;
  if (!apiKey) {
    console.error("Error: OPENAI_API_KEY environment variable required");
    console.error("Set it with: export OPENAI_API_KEY=sk-...");
    process.exit(1);
  }

  const fixtures = [
    "comparison_trim",
    "comparison_quantize_test",
    "comparison_official_test",
  ];

  console.log("Vision Model Approval Gate");
  console.log("==========================\n");

  for (const fixture of fixtures) {
    const refPath = path.join(SCREENSHOT_DIR, `${fixture}-reference.png`);
    const genPath = path.join(SCREENSHOT_DIR, `${fixture}-generated.png`);

    if (!fs.existsSync(refPath) || !fs.existsSync(genPath)) {
      console.log(`${fixture}: missing screenshots (run vision-compare.js first)`);
      continue;
    }

    try {
      const result = await callVisionModel(apiKey, refPath, genPath);
      const status = result.pass ? "PASS" : "FAIL";
      console.log(`${fixture}: ${status} (score: ${result.score}/100)`);
      console.log(`  reason: ${result.reason}`);
    } catch (err) {
      console.log(`${fixture}: ERROR - ${err.message}`);
    }
  }
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
