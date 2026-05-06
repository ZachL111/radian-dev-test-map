# radian-dev-test-map

`radian-dev-test-map` keeps a focused Rust implementation around developer tools. The project goal is to build a Rust toolkit that studies test behavior through round-trip fixtures, with lossless normalization checks and fixture-scale datasets.

## Project Rationale

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Radian Dev Test Map Review Notes

`edge` and `stale` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Feature Set

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/radian-dev-test-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `review cost` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust addition stays small enough to inspect in one sitting.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Test Command

The verifier is intentionally local. It should fail if the fixture score math, lane assignment, or language-specific test drifts.

## Next Improvements

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
