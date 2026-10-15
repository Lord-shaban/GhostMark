"""Check foundation integrity using Python 3.11+ and the standard library."""
from __future__ import annotations

import argparse
from pathlib import Path
import re
import tempfile
import tomllib
from urllib.parse import unquote, urlsplit

REQUIRED = (
    "README.md", "LICENSE", "CONTRIBUTING.md", "CODE_OF_CONDUCT.md", "SECURITY.md",
    "GOVERNANCE.md", "AGENTS.md", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
    ".gitignore", ".gitattributes", ".editorconfig", ".github/CODEOWNERS",
    ".github/PULL_REQUEST_TEMPLATE.md", ".github/ISSUE_TEMPLATE/config.yml",
    ".github/ISSUE_TEMPLATE/bug_report.yml", ".github/ISSUE_TEMPLATE/feature_request.yml",
    ".github/dependabot.yml", ".github/workflows/ci.yml",
    ".github/workflows/dependency-review.yml", "crates/ghostmark-core/Cargo.toml",
    "crates/ghostmark-core/src/lib.rs", "docs/README.ar.md", "docs/roadmap.md",
    "docs/architecture.md", "docs/development.md", "docs/workflow.md",
    "docs/testing.md", "docs/compatibility.md", "docs/releasing.md",
    "docs/adr/README.md", "docs/adr/0001-rust-stack.md",
    "docs/verification/README.md", "docs/verification/TEMPLATE.md",
)
LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
ACTION = re.compile(r"^\s*(?:-\s+)?uses:\s*([^\s#]+)", re.MULTILINE)


def check(root: Path) -> list[str]:
    errors = [f"Missing required file: {name}" for name in REQUIRED if not (root / name).is_file()]
    # Check tracked-style source paths only, never build artifacts or raw evidence.
    paths = [root / name for name in REQUIRED if name.endswith(".md")]
    paths.extend((root / "docs").rglob("*.md"))
    for path in sorted(set(paths)):
        if not path.is_file():
            continue
        text = path.read_text(encoding="utf-8")
        # Code examples may contain Markdown-looking strings that are not links.
        text = re.sub(r"```.*?```", "", text, flags=re.DOTALL)
        for target in LINK.findall(text):
            target = target.strip().removeprefix("<").removesuffix(">")
            parsed = urlsplit(target)
            if parsed.scheme or target.startswith(("#", "//")):
                continue
            local = unquote(parsed.path)
            if local and not (path.parent / local).exists():
                errors.append(f"Broken local link in {path.relative_to(root)}: {target}")
    for path in sorted((root / ".github" / "workflows").glob("*.yml")):
        for action in ACTION.findall(path.read_text(encoding="utf-8")):
            if not action.startswith(("./", "docker://")) and not re.fullmatch(r"[^@]+@[a-f0-9]{40}", action):
                errors.append(f"Unpinned action in {path.relative_to(root)}: {action}")
    try:
        manifest = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
        toolchain = tomllib.loads((root / "rust-toolchain.toml").read_text(encoding="utf-8"))
        lock = tomllib.loads((root / "Cargo.lock").read_text(encoding="utf-8"))
        package = manifest["workspace"]["package"]
        channel = toolchain["toolchain"]["channel"]
        if not re.fullmatch(r"\d+\.\d+\.\d+", channel):
            errors.append("Rust toolchain must be pinned to a stable patch version")
        if package["rust-version"] != ".".join(channel.split(".")[:2]):
            errors.append("Workspace rust-version and pinned toolchain disagree")
        if package["edition"] != "2024":
            errors.append("Workspace must use the accepted Rust 2024 edition")
        for member in manifest["workspace"]["members"]:
            crate = tomllib.loads((root / member / "Cargo.toml").read_text(encoding="utf-8"))
            item = crate["package"]
            version = package["version"] if isinstance(item["version"], dict) else item["version"]
            if not any(p["name"] == item["name"] and p["version"] == version for p in lock.get("package", [])):
                errors.append(f"Cargo.lock does not contain the workspace package {item['name']} {version}")
            if item.get("publish") is not False:
                errors.append(f"Workspace application crate must set publish=false: {member}")
    except (OSError, KeyError, TypeError, tomllib.TOMLDecodeError) as error:
        errors.append(f"Invalid or missing Cargo/toolchain configuration: {error}")
    return errors


def self_test(root: Path) -> None:
    with tempfile.TemporaryDirectory(prefix="ghostmark-repo-check-") as directory:
        fixture = Path(directory)
        for name in REQUIRED:
            source = root / name
            destination = fixture / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(source.read_bytes())
        assert not check(fixture), "The valid fixture must pass"
        readme = fixture / "README.md"
        original = readme.read_text(encoding="utf-8")
        readme.write_text(original + "\n[Broken](missing-file.md)\n", encoding="utf-8")
        assert any("Broken local link" in error for error in check(fixture))
        readme.write_text(original, encoding="utf-8")
        readme.unlink()
        assert any("Missing required file: README.md" in error for error in check(fixture))
        readme.write_text(original, encoding="utf-8")
        workflow = fixture / ".github/workflows/ci.yml"
        workflow_original = workflow.read_text(encoding="utf-8")
        workflow.write_text(workflow_original + "\n# test fixture\n- uses: actions/checkout@main\n", encoding="utf-8")
        assert any("Unpinned action" in error for error in check(fixture))
        workflow.write_text(workflow_original, encoding="utf-8")
        compiler = fixture / "rust-toolchain.toml"
        compiler.write_text('[toolchain]\nchannel = "stable"\n', encoding="utf-8")
        assert any("must be pinned" in error for error in check(fixture))
        compiler.write_text('[toolchain]\nchannel = "1.1.0"\n', encoding="utf-8")
        assert any("disagree" in error for error in check(fixture))
    print("Repository validator self-tests passed (valid, broken link, missing file, floating action, floating compiler, version mismatch).")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    repository = Path(__file__).resolve().parent.parent
    if arguments.self_test:
        self_test(repository)
    else:
        failures = check(repository)
        for failure in failures:
            print(f"ERROR: {failure}")
        if failures:
            raise SystemExit(1)
        print("Repository files, local links, Action pins and Rust configuration are valid.")
