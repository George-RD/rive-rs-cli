#!/usr/bin/env python3
"""Vision Gate Subagent Workflow

Runs within the OMP harness eval tool using agent() subagents
for independent visual review by multiple vision model providers.

Usage in eval:
    exec(open("scripts/vision_gate_subagent.py").read())
"""

import os
import json
import base64

SCREENSHOT_DIR = "target/playwright-vision"
FIXTURES = ["comparison_trim", "comparison_quantize_test", "comparison_official_test"]
PROVIDERS = ["openai"]  # Extend as needed

def build_subagent_prompt(fixture: str, provider: str, ref_path: str, gen_path: str) -> str:
    """Build a prompt for a subagent to review a fixture pair."""
    api_key_var = {
        "openai": "OPENAI_API_KEY",
        "anthropic": "ANTHROPIC_API_KEY",
        "gemini": "GOOGLE_API_KEY"
    }.get(provider, f"{provider.upper()}_API_KEY")
    
    prompt = f"""
You are a visual QA judge. Review the Rive animation fixture '{fixture}'.

Task:
1. Read the reference screenshot: {ref_path}
2. Read the generated screenshot: {gen_path}
3. Use bash to run a Python script that:
   - Encodes both images as base64
   - Sends them to the {provider} vision API
   - Returns a JSON judgment

For OpenAI, use model gpt-4o with this payload structure:
{{
  "model": "gpt-4o",
  "messages": [
    {{
      "role": "system",
      "content": "You are a visual QA judge. Compare two images. Return JSON: {{\\"pass\\": bool, \\"score\\": 0-100, \\"reason\\": str}}"
    }},
    {{
      "role": "user",
      "content": [
        {{"type": "text", "text": "Compare these two images. Image 1 is reference. Image 2 is generated."}},
        {{"type": "image_url", "image_url": {{"url": "data:image/png;base64,<ref_data>"}}}},
        {{"type": "image_url", "image_url": {{"url": "data:image/png;base64,<gen_data>"}}}}
      ]
    }}
  ],
  "max_tokens": 300,
  "response_format": {{"type": "json_object"}}
}}

API endpoint: https://api.openai.com/v1/chat/completions
Headers: Authorization: Bearer $OPENAI_API_KEY

Return ONLY a JSON object with these exact keys:
- pass (boolean): true if visually similar enough
- score (number 0-100): likeness score
- reason (string): brief explanation
"""
    return prompt


def run_subagent_vision_gate():
    """Run vision gate using eval subagents."""
    print("=" * 60)
    print("Vision Gate Subagent Workflow")
    print("=" * 60)
    
    results = []
    
    for fixture in FIXTURES:
        ref_path = f"{SCREENSHOT_DIR}/{fixture}-reference.png"
        gen_path = f"{SCREENSHOT_DIR}/{fixture}-generated.png"
        
        if not os.path.exists(ref_path) or not os.path.exists(gen_path):
            print(f"  ! {fixture}: missing screenshots")
            continue
        
        print(f"\nReviewing {fixture}...")
        fixture_results = {"fixture": fixture, "judgments": []}
        
        for provider in PROVIDERS:
            api_key = os.environ.get({
                "openai": "OPENAI_API_KEY",
                "anthropic": "ANTHROPIC_API_KEY",
                "gemini": "GOOGLE_API_KEY"
            }.get(provider))
            
            if not api_key:
                print(f"  ! {provider}: no API key")
                continue
            
            prompt = build_subagent_prompt(fixture, provider, ref_path, gen_path)
            
            # Spawn subagent for independent review
            print(f"  -> Spawning {provider} subagent...")
            try:
                judgment = agent(
                    prompt,
                    label=f"vision-gate-{fixture}-{provider}",
                    schema={
                        "type": "object",
                        "properties": {
                            "pass": {"type": "boolean"},
                            "score": {"type": "number"},
                            "reason": {"type": "string"}
                        },
                        "required": ["pass", "score", "reason"]
                    }
                )
                judgment["provider"] = provider
                fixture_results["judgments"].append(judgment)
                status = "PASS" if judgment.get("pass") else "FAIL"
                print(f"     {status} (score: {judgment.get('score', 'N/A')})")
            except Exception as e:
                print(f"     ERROR: {e}")
                fixture_results["judgments"].append({
                    "provider": provider,
                    "error": str(e)
                })
        
        results.append(fixture_results)
    
    # Aggregate
    print("\n" + "-" * 60)
    passed = sum(1 for r in results if any(j.get("pass") for j in r.get("judgments", [])))
    failed = len(results) - passed
    print(f"Summary: {passed} passed, {failed} failed out of {len(results)} fixtures")
    
    # Save
    with open("target/vision-gate-subagent-results.json", "w") as f:
        json.dump(results, f, indent=2)
    print("Results saved to target/vision-gate-subagent-results.json")
    
    return results


# Auto-run if executed directly in eval
if __name__ == "__main__":
    run_subagent_vision_gate()
