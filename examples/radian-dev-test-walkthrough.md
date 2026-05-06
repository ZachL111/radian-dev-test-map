# Radian Dev Test Map Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 156 | ship |
| stress | diagnostic quality | 189 | ship |
| edge | review cost | 225 | ship |
| recovery | safe rewrite | 185 | ship |
| stale | change width | 103 | hold |

Start with `edge` and `stale`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The useful comparison is `review cost` against `change width`, not the raw score alone.
