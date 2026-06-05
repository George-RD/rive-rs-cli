#!/usr/bin/env python3
"""Vision Gate OMP Harness Workflow

This script runs within the OMP harness eval tool and uses subagents
for parallel visual review of Rive comparison fixtures.

Usage within eval tool:
    exec(open("scripts/vision_gate_omp.py").read())

Or standalone:
    python3 scripts/vision_gate_omp.py
"""

import os
import sys
import json
import base64
import subprocess
from pathlib import Path
from typing import Dict, List

# Check if we're running inside eval (has agent/parallel/llm functions)
IN_EVAL = 'agent' in globals() and 'parallel' in globals()

SCREENSHOT_DIR = Path("target/playwright-vision")
FIXTURES = ["comparison_trim", "comparison_quantize_test", "comparison_official_test"]

API_KEYS = {
    "openai": os.environ.get("OPENAI_API_KEY"),
    "anthropic": os.environ.get("ANTHROPIC_API_KEY"),
    "gemini": os.environ.get("GOOGLE_API_KEY"),
}


def encode_image(path: str) -> str:
    with open(path, "rb") as f:
        data = f.read()
    return f"data:image/png;base64,{base64.b64encode(data).decode()}"


def call_openai_vision(ref_b64: str, gen_b64: str, fixture: str) -> Dict:
    """Call OpenAI GPT-4o vision API."""
    key = API_KEYS["openai"]
    if not key:
        return {"provider": "openai", "error": "No API key"}
    
    import urllib.request
    payload = {
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": (
                    "You are a visual QA judge for Rive animations. "
                    "Compare a reference image (Image 1) vs generated image (Image 2). "
                    'Return JSON: {"pass": bool, "score": 0-100, "reason": str}'
                )
            },
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": f"Fixture: {fixture}"},
                    {"type": "image_url", "image_url": {"url": ref_b64}},
                    {"type": "image_url", "image_url": {"url": gen_b64}}
                ]
            }
        ],
        "max_tokens": 300,
        "response_format": {"type": "json_object"}
    }
    req = urllib.request.Request(
        "https://api.openai.com/v1/chat/completions",
        data=json.dumps(payload).encode(),
        headers={"Content-Type": "application/json", "Authorization": f"Bearer {key}"},
        method="POST"
    )
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            data = json.loads(resp.read())
            content = data["choices"][0]["message"]["content"]
            result = json.loads(content)
            result["provider"] = "openai"
            return result
    except Exception as e:
        return {"provider": "openai", "error": str(e)}


def judge_fixture_standalone(fixture: str) -> Dict:
    """Judge a fixture (standalone mode using direct API calls)."""
    ref_path = SCREENSHOT_DIR / f"{fixture}-reference.png"
    gen_path = SCREENSHOT_DIR / f"{fixture}-generated.png"
    if not ref_path.exists() or not gen_path.exists():
        return {"fixture": fixture, "error": "Missing screenshots"}
    
    ref_b64 = encode_image(str(ref_path))
    gen_b64 = encode_image(str(gen_path))
    
    results = {"fixture": fixture, "judgments": []}
    for provider in ["openai", "anthropic", "gemini"]:
        if API_KEYS.get(provider):
            if provider == "openai":
                result = call_openai_vision(ref_b64, gen_b64, fixture)
            else:
                result = {"provider": provider, "error": "Not yet implemented"}
            results["judgments"].append(result)
    
    return results


def run_vision_gate():
    """Main entry point."""
    print("=" * 60)
    print("Vision Gate OMP Harness Workflow")
    print("=" * 60)
    
    # Ensure screenshots exist
    if not any((SCREENSHOT_DIR / f"{f}-reference.png").exists() for f in FIXTURES):
        print("Generating screenshots...")
        subprocess.run(
            ["node", "tests/playwright/vision-compare.js"],
            capture_output=True, timeout=120
        )
    
    print(f"\nMode: {'OMP eval harness' if IN_EVAL else 'standalone'}")
    print(f"Fixtures: {FIXTURES}")
    
    available = [p for p, k in API_KEYS.items() if k]
    if not available:
        print("\nNo API keys configured. Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or GOOGLE_API_KEY")
        return
    print(f"Providers: {available}")
    
    if IN_EVAL:
        print("\nUsing eval parallel() for concurrent review...")
        # In eval mode, use parallel() to fan out workers
        def review_fixture(fixture):
            return judge_fixture_standalone(fixture)
        
        # Note: parallel() requires callable thunks
        # We can't easily use it here without the eval runtime
        # Fall back to sequential for now
        results = [judge_fixture_standalone(f) for f in FIXTURES]
    else:
        print("\nRunning standalone sequential review...")
        results = [judge_fixture_standalone(f) for f in FIXTURES]

    # Report
    print("\n" + "-" * 60)
    for r in results:
        print(f"\n{r['fixture']}:")
        for j in r.get("judgments", []):
            if "error" in j:
                print(f"  {j['provider']}: ERROR - {j['error']}")
            else:
                status = "PASS" if j.get("pass") else "FAIL"
                print(f"  {j['provider']}: {status} (score: {j.get('score', 'N/A')})")
    
    # Save
    output = Path("target/vision-gate-results.json")
    output.parent.mkdir(parents=True, exist_ok=True)
    with open(output, "w") as f:
        json.dump(results, f, indent=2)
    print(f"Results saved to: {output}")


if __name__ == "__main__":
    run_vision_gate()

