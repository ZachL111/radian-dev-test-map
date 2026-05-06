# Review Journal

This journal records the domain cases that matter before widening the public API.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 156, lane `ship`
- `stress`: `diagnostic quality`, score 189, lane `ship`
- `edge`: `review cost`, score 225, lane `ship`
- `recovery`: `safe rewrite`, score 185, lane `ship`
- `stale`: `change width`, score 103, lane `hold`

## Note

A future change should add new cases before it changes the scoring rule.
