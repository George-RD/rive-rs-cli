#!/usr/bin/env python3
"""Vision Gate Orchestrator - Subagent-based visual approval gate for Rive parity.

This script implements a vision model gate workflow using subagents to review
visual comparisons between reference and generated .riv files.

Supported vision model providers:
- OpenAI GPT-4o (default)
- Anthropic Claude 3 Opus / Sonnet
- Google Gemini Pro Vision

Usage:
    export OPENAI_API_KEY=sk-...
    export ANTHROPIC_API_KEY=sk-ant-...
    export GOOGLE_API_KEY=...
    python3 scripts/vision_gate_orchestrator.py

Or within the OMP harness eval tool:
    exec(open("scripts/vision_gate_orchestrator.py").read())
"""

import os
import sys
import json
import base64
import concurrent.futures
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# Configuration
SCREENSHOT_DIR = Path(os.environ.get("VISION_SCREENSHOT_DIR", "target/playwright-vision"))
REFERENCE_DIR = Path(os.environ.get("VISION_REFERENCE_DIR", "parity/official"))
FIXTURES = os.environ.get("VISION_FIXTURES", "comparison_trim,comparison_quantize_test,comparison_official_test").split(",")

# API Keys (read from environment)
OPENAI_API_KEY = os.environ.get("OPENAI_API_KEY")
ANTHROPIC_API_KEY = os.environ.get("ANTHROPIC_API_KEY")
GOOGLE_API_KEY = os.environ.get("GOOGLE_API_KEY")


def encode_image(path: str) -> str:
    """Encode image file as base64 data URL."""
    with open(path, "rb") as f:
        data = f.read()
    ext = Path(path).suffix.lower()
    mime = "image/png" if ext == ".png" else "image/jpeg" if ext in (".jpg", ".jpeg") else "application/octet-stream"
    return f"data:{mime};base64,{base64.b64encode(data).decode()}"


def judge_with_openai(ref_path: str, gen_path: str, fixture: str) -> Dict:
    """Use OpenAI GPT-4o to judge visual likeness."""
    if not OPENAI_API_KEY:
        return {"provider": "openai", "error": "OPENAI_API_KEY not set"}
    
    import urllib.request
    import urllib.error
    
    ref_b64 = encode_image(ref_path)
    gen_b64 = encode_image(gen_path)
    
    payload = {
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": (
                    "You are a visual QA judge for Rive animation files. "
                    "Compare two images: a reference (Image 1) and a generated version (Image 2). "
                    "Determine if they are semantically similar. Consider: same objects, same colors, "
                    "same layout. Minor pixel differences are acceptable. Missing major elements or "
                    "wrong colors is a failure. Respond with JSON only: {\"pass\": boolean, "
                    '"score": number (0-100), "reason": string}'
                )
            },
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": f"Fixture: {fixture}. Compare these two images. Image 1 is the reference. Image 2 is the generated version."},
                    {"type": "image_url", "image_url": {"url": ref_b64}},
                    {"type": "image_url", "image_url": {"url": gen_b64}}
                ]
            }
        ],
        "max_tokens": 500,
        "response_format": {"type": "json_object"}
    }
    
    req = urllib.request.Request(
        "https://api.openai.com/v1/chat/completions",
        data=json.dumps(payload).encode(),
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {OPENAI_API_KEY}"
        },
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


def judge_with_anthropic(ref_path: str, gen_path: str, fixture: str) -> Dict:
    """Use Anthropic Claude 3 to judge visual likeness."""
    if not ANTHROPIC_API_KEY:
        return {"provider": "anthropic", "error": "ANTHROPIC_API_KEY not set"}
    
    import urllib.request
    
    ref_b64 = encode_image(ref_path)
    gen_b64 = encode_image(gen_path)
    
    payload = {
        "model": "claude-3-opus-20240229",
        "max_tokens": 500,
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": (
                            f"You are a visual QA judge for Rive animation files. Compare these two images. "
                            f"Image 1 is the reference. Image 2 is the generated version for fixture '{fixture}'. "
                            f"Determine if they are semantically similar. Consider: same objects, same colors, "
                            f"same layout. Minor pixel differences are acceptable. Missing major elements or "
                            f"wrong colors is a failure. Respond with JSON only: "
                            f'{{"pass": boolean, "score": number (0-100), "reason": string}}'
                        )
                    },
                    {"type": "image", "source": {"type": "base64", "media_type": "image/png", "data": ref_b64.split(",")[1]}},
                    {"type": "image", "source": {"type": "base64", "media_type": "image/png", "data": gen_b64.split(",")[1]}}
                ]
            }
        ]
    }
    
    req = urllib.request.Request(
        "https://api.anthropic.com/v1/messages",
        data=json.dumps(payload).encode(),
        headers={
            "Content-Type": "application/json",
            "x-api-key": ANTHROPIC_API_KEY,
            "anthropic-version": "2023-06-01"
        },
        method="POST"
    )
    
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            data = json.loads(resp.read())
            # Extract JSON from text response
            text = data["content"][0]["text"]
            # Try to parse JSON from the text
            try:
                result = json.loads(text)
            except json.JSONDecodeError:
                # Try to extract JSON from markdown code blocks
                import re
                match = re.search(r'```json\s*(.*?)\s*```', text, re.DOTALL)
                if match:
                    result = json.loads(match.group(1))
                else:
                    result = {"pass": False, "score": 0, "reason": f"Could not parse JSON from response: {text[:200]}"}
            result["provider"] = "anthropic"
            return result
    except Exception as e:
        return {"provider": "anthropic", "error": str(e)}


def judge_with_gemini(ref_path: str, gen_path: str, fixture: str) -> Dict:
    """Use Google Gemini Pro Vision to judge visual likeness."""
    if not GOOGLE_API_KEY:
        return {"provider": "gemini", "error": "GOOGLE_API_KEY not set"}
    
    import urllib.request
    
    ref_b64 = encode_image(ref_path).split(",")[1]
    gen_b64 = encode_image(gen_path).split(",")[1]
    
    payload = {
        "contents": [
            {
                "parts": [
                    {"text": (
                        f"You are a visual QA judge for Rive animation files. Compare these two images. "
                        f"Image 1 is the reference. Image 2 is the generated version for fixture '{fixture}'. "
                        f"Determine if they are semantically similar. Consider: same objects, same colors, "
                        f"same layout. Minor pixel differences are acceptable. Missing major elements or "
                        f"wrong colors is a failure. Respond with JSON only: "
                        f'{{"pass": boolean, "score": number (0-100), "reason": string}}'
                    )},
                    {"inline_data": {"mime_type": "image/png", "data": ref_b64}},
                    {"inline_data": {"mime_type": "image/png", "data": gen_b64}}
                ]
            }
        ],
        "generationConfig": {"maxOutputTokens": 500}
    }
    
    req = urllib.request.Request(
        f"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-vision:generateContent?key={GOOGLE_API_KEY}",
        data=json.dumps(payload).encode(),
        headers={"Content-Type": "application/json"},
        method="POST"
    )
    
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            data = json.loads(resp.read())
            text = data["candidates"][0]["content"]["parts"][0]["text"]
            try:
                result = json.loads(text)
            except json.JSONDecodeError:
                import re
                match = re.search(r'```json\s*(.*?)\s*```', text, re.DOTALL)
                if match:
                    result = json.loads(match.group(1))
                else:
                    result = {"pass": False, "score": 0, "reason": f"Could not parse JSON: {text[:200]}"}
            result["provider"] = "gemini"
            return result
    except Exception as e:
        return {"provider": "gemini", "error": str(e)}


def judge_fixture(fixture: str, providers: List[str] = None) -> Dict:
    """Judge a single fixture using the specified providers."""
    if providers is None:
        providers = ["openai", "anthropic", "gemini"]
    
    ref_path = SCREENSHOT_DIR / f"{fixture}-reference.png"
    gen_path = SCREENSHOT_DIR / f"{fixture}-generated.png"
    
    if not ref_path.exists() or not gen_path.exists():
        return {"fixture": fixture, "error": "Missing screenshots"}
    
    results = {"fixture": fixture, "judgments": []}
    
    for provider in providers:
        if provider == "openai":
            result = judge_with_openai(str(ref_path), str(gen_path), fixture)
        elif provider == "anthropic":
            result = judge_with_anthropic(str(ref_path), str(gen_path), fixture)
        elif provider == "gemini":
            result = judge_with_gemini(str(ref_path), str(gen_path), fixture)
        else:
            result = {"provider": provider, "error": "Unknown provider"}
        results["judgments"].append(result)
    
    return results


def aggregate_results(results: List[Dict]) -> Dict:
    """Aggregate judgments across providers and fixtures."""
    summary = {
        "total_fixtures": len(results),
        "passed": 0,
        "failed": 0,
        "errors": 0,
        "fixtures": []
    }
    
    for r in results:
        fixture_summary = {"fixture": r["fixture"], "judgments": r.get("judgments", [])}
        
        # Count passes per fixture
        passes = [j for j in r.get("judgments", []) if j.get("pass") is True]
        fails = [j for j in r.get("judgments", []) if j.get("pass") is False]
        errs = [j for j in r.get("judgments", []) if "error" in j]
        
        if errs:
            fixture_summary["status"] = "error"
            summary["errors"] += 1
        elif passes and not fails:
            fixture_summary["status"] = "pass"
            summary["passed"] += 1
        else:
            fixture_summary["status"] = "fail"
            summary["failed"] += 1
        
        # Average score from successful judgments
        scores = [j.get("score", 0) for j in r.get("judgments", []) if "error" not in j and "score" in j]
        if scores:
            fixture_summary["avg_score"] = round(sum(scores) / len(scores), 2)
        
        summary["fixtures"].append(fixture_summary)
    
    return summary


def main():
    """Run the vision gate orchestrator."""
    print("=" * 60)
    print("Vision Gate Orchestrator")
    print("=" * 60)
    print(f"Screenshots: {SCREENSHOT_DIR}")
    print(f"Fixtures: {FIXTURES}")
    print()
    
    # Check available providers
    available = []
    if OPENAI_API_KEY:
        available.append("openai")
    if ANTHROPIC_API_KEY:
        available.append("anthropic")
    if GOOGLE_API_KEY:
        available.append("gemini")
    
    if not available:
        print("ERROR: No vision model API keys configured.")
        print("Set one of: OPENAI_API_KEY, ANTHROPIC_API_KEY, GOOGLE_API_KEY")
        sys.exit(1)
    
    print(f"Available providers: {available}")
    print()
    
    # Run judgments in parallel
    print("Running judgments...")
    results = []
    
    # Use ThreadPoolExecutor for parallel API calls
    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
        futures = {executor.submit(judge_fixture, f, available): f for f in FIXTURES}
        for future in concurrent.futures.as_completed(futures):
            fixture = futures[future]
            try:
                result = future.result()
                results.append(result)
                print(f"  ✓ {fixture} judged")
            except Exception as e:
                print(f"  ✗ {fixture} error: {e}")
                results.append({"fixture": fixture, "error": str(e)})
    
    print()
    
    # Aggregate and report
    summary = aggregate_results(results)
    
    print("Results")
    print("-" * 60)
    for f in summary["fixtures"]:
        status_icon = "✓" if f["status"] == "pass" else "✗" if f["status"] == "fail" else "?"
        score_str = f" (avg score: {f.get('avg_score', 'N/A')})" if "avg_score" in f else ""
        print(f"  {status_icon} {f['fixture']}: {f['status']}{score_str}")
        for j in f.get("judgments", []):
            provider = j.get("provider", "unknown")
            if "error" in j:
                print(f"      {provider}: ERROR - {j['error']}")
            else:
                verdict = "PASS" if j.get("pass") else "FAIL"
                print(f"      {provider}: {verdict} (score: {j.get('score', 'N/A')}) - {j.get('reason', 'N/A')[:80]}")
    
    print()
    print(f"Summary: {summary['passed']} passed, {summary['failed']} failed, {summary['errors']} errors")
    
    # Save detailed results
    output_path = Path("target/vision-gate-results.json")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(summary, f, indent=2)
    print(f"\\nDetailed results saved to: {output_path}")
    
    # Exit with non-zero if any failures
    if summary["failed"] > 0 or summary["errors"] > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
