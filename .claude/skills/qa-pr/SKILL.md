---
name: qa-pr
description: Reviews a pull request of ltruchot/haiku-generator and reports findings, without approving, merging or changing anything. Use when asked to QA, review or check a PR, or to say whether a PR is ready.
argument-hint: "[pr-number]"
allowed-tools: Bash(gh:*) Bash(git:*) Read Grep
---

# QA a pull request

- Repo: ltruchot/haiku-generator
- Default branch: master
- Prod URL: https://developers.gods.academy/loic-truchot/projects/haiku-generator/

Read-only: never approve, never merge, never push, never comment on the PR. Report here.

## Steps

1. The PR is `$ARGUMENTS`. Empty: `gh pr list --repo ltruchot/haiku-generator` and ask which one.
2. Read what it claims and where it goes:
   `gh pr view $ARGUMENTS --repo ltruchot/haiku-generator --json title,body,baseRefName,headRefName,isDraft,additions,deletions,files,statusCheckRollup`
3. Read the change: `gh pr diff $ARGUMENTS --repo ltruchot/haiku-generator`
   (`--name-only` first on a large PR).
4. A diff shows lines, not their effect. For each file that matters, read the whole file at the
   PR's head, and `grep` the callers of what changed. A finding rests on code you read, never
   on what a name suggests.
5. Walk [references/checklist.md](references/checklist.md): correctness, safety, quality,
   process. One finding per problem.
6. CI: `gh pr checks $ARGUMENTS --repo ltruchot/haiku-generator`. A failing or missing check is a finding.
7. Flag separately anything that touches production configuration or https://developers.gods.academy/loic-truchot/projects/haiku-generator/.

## A finding

- Names the file and the line, says what breaks and in which case, and proposes the fix.
- Is something you can show: the input that fails, the caller that is not updated, the test
  that does not exist. A doubt you could not confirm is listed as a question, not a finding.
- Style that a formatter or a linter of the repo already enforces is not a finding.

## Output

- Verdict line: `ready`, `needs work` or `blocked`, with the one reason.
- Findings, most severe first: `file:line — issue — fix`.
- Questions for the author.
- Missing tests, last.
