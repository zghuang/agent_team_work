# Project Working Agreement (GitHub) — Lightweight Guide

## 1) Roles & Responsibilities

### Owner / Boss (Decision Maker)
- Defines vision and high-level requirements.
- Approves scope changes and releases.
- Final decision maker when the team cannot reach agreement.

### Coordinator (Product Manager + Scrum Master)
- Turns requirements into clear GitHub Issues (user story + acceptance criteria).
- Prioritizes backlog, plans iterations, assigns tasks, coordinates resources.
- Runs ceremonies (standup/planning/review/retro), removes blockers.
- Ensures transparency (board updated, deadlines visible).
- Escalates conflicts/no-consensus to Owner.

### Senior Developer (Tech Lead)
- Owns architecture and key technical decisions.
- Implements core/complex parts; mentors developers.
- Reviews PRs (quality/security/maintainability) and enforces standards.
- Keeps technical docs updated when design changes.

### Developer
- Implements assigned issues, writes unit tests, updates docs where needed.
- Opens PRs early as Draft, responds to review feedback quickly.
- Fixes bugs discovered by testing.

### Tester (QA)
- Validates features vs acceptance criteria.
- Writes test cases/plans as needed; performs regression testing.
- Reports bugs in GitHub Issues (repro steps + expected/actual).
- Can block release if acceptance criteria are not met.

---

## 2) Requirement Handling & Decision Rule

For every new request:
1. Coordinator publishes the requirement as a GitHub Issue (problem, scope, acceptance criteria).
2. Team discussion to confirm:
   - expected behavior, edge cases, non-functional needs (perf/security)
   - technical approach and testing approach
3. If consensus is reached → proceed.
4. If consensus is not reached within the agreed timebox → Owner decides.

---

## 3) GitHub as the Source of Truth (Issues + Discussions)

### GitHub Issues (mandatory)
All work must start from an Issue. This includes features, bugs, tech debt, refactors, and chores.

Each Issue must include:
- User story (what/why)
- Acceptance criteria (checklist)
- Priority + owner/assignee
- Links to related PRs / discussions

Rule: Telegram/chat can be used for speed, but the final requirement and decisions must be captured in GitHub.

### GitHub Discussions (recommended for decisions & design)
Use GitHub Discussions for content that benefits from visibility, structure, and long-term reference, such as:
- Architecture proposals / tradeoff analysis
- API contract changes
- Product decisions and rationale
- Cross-cutting design topics (logging, auth, data model)
- "How should we do X?" debates
- Meeting notes (key outcomes only)

Rule: If a decision affects scope, architecture, API, or user behavior, summarize it in a Discussion (or in the Issue) and link it.

Working pattern:
- Create a Discussion → gather options → decide → post the final decision summary
- Link the Discussion from the Issue (and vice versa)

---

## 4) Definition of Done (DoD)
- Code implemented
- Unit tests added/updated
- PR reviewed & approved
- QA passed (acceptance criteria met)
- Docs updated if behavior/API changed
- Issue updated with outcomes/links

---

## 5) Branching Strategy (Simple & Standard)

### Long-lived branches
- main: production (stable)
- develop: integration branch for upcoming release

### Short-lived branches (from develop)
- feature/<issue-id>-short-name
- bugfix/<issue-id>-short-name
- hotfix/<issue-id>-short-name (from main only for production emergencies)

### Merge rules
- PRs merge into develop (normal work)
- Releases: develop → main via release PR
- Hotfix: main hotfix merged back into develop as well

---

## 6) Pull Request (PR) Rules

### When to open a PR
- Open early as Draft PR when work starts (visibility + feedback).
- Mark ready when tests pass and implementation complete.

### Required reviewers
- Senior Dev approval required for all merges to develop/main.
- Tester approval required for merges to main (release/hotfix).

### PR description (must include)
- What changed + why
- Link: Closes #<issue-id>
- How to test (steps)
- Risk/impact notes (if any)

---

## 7) Commit Convention (Mandatory)

Use Conventional Commits:
<type>(<scope>): <short summary>

Types:
- feat: new feature
- fix: bug fix
- refactor: code change without behavior change
- test: tests only
- docs: documentation only
- chore: tooling/maintenance

Examples:
- feat(auth): add login endpoint
- fix(api): handle null payload
- test(user): add service tests
- chore(ci): update pipeline

Guidelines:
- One logical change per commit
- Reference issues in PRs (preferred) and/or commits when useful

---

## 8) Quality Gates (CI + Reviews)

### Must pass before merge
- CI checks green (lint/build/tests)
- Senior Dev review approved
- No unresolved conversations
- Tests included for critical logic

### Tester acceptance
- Feature meets acceptance criteria
- No critical regressions
- Bugs are tracked as Issues (with severity)

---

## 9) Coordinator Operating Rules (Task & Resource Management)

- Maintains GitHub Project board (Backlog → Todo → In Progress → In Review → Testing → Done).
- Assigns owners and due dates; ensures nobody is overloaded.
- Tracks blockers daily; escalates quickly when blocked > 1 day.
- Controls scope within sprint/iteration; negotiates changes with Owner.

---

## 10) Communication & Collaboration Norms

- Use GitHub Issues/PR comments for decisions and technical discussion (audit trail).
- Telegram/chat for quick questions and fast iteration.
- Critical outcomes from Telegram must be summarized in GitHub (Issue or Discussion), including:
  - Final decision
  - Alternatives considered
  - Reasoning
  - Impact / follow-ups

Expected response time:
- PR review request: within 24h
- Blocking question: within 4h during working hours

---

## 11) Release & Hotfix

### Release (normal)
1. Coordinator confirms "release candidate" in develop
2. Tester completes regression
3. Coordinator opens PR develop → main
4. Owner approves release
5. Tag version on main (e.g., v1.2.0)

### Hotfix (urgent production)
1. Senior Dev creates hotfix/... from main
2. Minimal change, fast review + QA
3. Merge to main and back-merge to develop
4. Owner informed/approves if risk is high

---

## 12) Disagreement / Escalation

1. Try to resolve with data (tradeoffs: time, risk, maintainability).
2. Timebox discussion (e.g., 30–60 minutes).
3. If still not aligned:
   - Coordinator summarizes options + pros/cons (in Issue or Discussion)
   - Owner makes the final decision
