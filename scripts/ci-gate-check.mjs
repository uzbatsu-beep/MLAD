/**
 * MLAD's single required status decision: fail closed on missing/failed,
 * unexpectedly skipped, or inconsistent native/Windows checks.
 * No dependencies, network access, native I/O, or controller interaction.
 */
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";

export function evaluateGate({ scope, frontend, rust, windows, runtimeRequired }) {
  const problems = [];
  if (scope !== "success") {
    problems.push(`Scope classification must succeed (got ${JSON.stringify(scope)})`);
  }
  if (frontend !== "success") {
    problems.push(`Frontend, docs, and security models must succeed (got ${JSON.stringify(frontend)})`);
  }

  if (runtimeRequired !== "true" && runtimeRequired !== "false") {
    problems.push(`Invalid or missing scope.runtime_required: ${JSON.stringify(runtimeRequired)}`);
  } else if (runtimeRequired === "true") {
    if (rust !== "success") {
      problems.push(`Native Rust tests were required but not successful (got ${JSON.stringify(rust)})`);
    }
    if (windows !== "success") {
      problems.push(`Windows NTFS + unsigned Tauri build were required but not successful (got ${JSON.stringify(windows)})`);
    }
  } else {
    if (rust !== "skipped") {
      problems.push(`Docs-only PR must skip Rust build (got ${JSON.stringify(rust)})`);
    }
    if (windows !== "skipped") {
      problems.push(`Docs-only PR must skip Windows build (got ${JSON.stringify(windows)})`);
    }
  }

  return { ok: problems.length === 0, problems };
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  const results = {
    scope: process.env.MLAD_SCOPE_RESULT,
    frontend: process.env.MLAD_FRONTEND_RESULT,
    rust: process.env.MLAD_RUST_RESULT,
    windows: process.env.MLAD_WINDOWS_RESULT,
    runtimeRequired: process.env.MLAD_RUNTIME_REQUIRED,
  };
  const verdict = evaluateGate(results);
  const summary = `MLAD ci-gate: scope=${results.scope} frontend=${results.frontend} rust=${results.rust} windows=${results.windows} runtimeRequired=${results.runtimeRequired}`;
  console.log(summary);
  if (!verdict.ok) {
    for (const problem of verdict.problems) console.error("CI GATE REFUSAL:", problem);
    process.exitCode = 1;
  } else {
    console.log("MLAD ci-gate PASS — every applicable validation succeeded.");
  }
}
