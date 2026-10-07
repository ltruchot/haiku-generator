# QA checklist

## Correctness

- The behavior matches the PR title and body; nothing unrelated rides along
- Edge cases: empty, null, huge, concurrent, repeated
- Errors are surfaced, not swallowed
- Every caller of a changed function, type or route is updated

## Safety

- No secret, token or credential in the diff, in a fixture or in a log line
- Input from outside is validated before it reaches a query, a path, a shell or HTML
- No new network call, dependency or permission without a stated reason
- A new dependency: pinned, maintained, and its install scripts read
- Migrations are reversible, and safe to run while the previous version still serves

## Quality

- Tests added or updated, and they fail without the change
- Names say what, comments say why
- No dead code, no commented-out code, no leftover debug output

## Process

- Targets master
- Commits are atomic and their messages state the change
- CI is green, and no check was skipped or disabled by the PR
