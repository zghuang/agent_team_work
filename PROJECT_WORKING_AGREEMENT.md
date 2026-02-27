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
Use GitHub Discussions for content that benefits from visibility, structure, and long-term reference.

---

## 4) Definition of Done (DoD)
- Code implemented
- Unit tests added/updated
- PR reviewed & approved
- QA passed (acceptance criteria met)
- Docs updated if behavior/API changed
- Issue updated with outcomes/links

---

## 5) Branching Strategy
- main: production (stable)
- develop: integration branch for upcoming release
- feature/<issue-id>-short-name
- bugfix/<issue-id>-short-name
- hotfix/<issue-id>-short-name (from main only)

---

## 6) Pull Request (PR) Rules
- Open early as Draft PR when work starts
- Senior Dev approval required for all merges
- Tester approval required for merges to main
- Must include: What changed + why, Link: Closes #<issue-id>, How to test

---

## 7) Commit Convention (Mandatory)
Use Conventional Commits:
<type>(<scope>): <short summary>

Types: feat, fix, refactor, test, docs, chore

---

## 8) Quality Gates
- CI checks green
- Senior Dev review approved
- No unresolved conversations
- Tests included for critical logic
- Tester accepts feature

---

## 9) Coordinator Operating Rules
- Maintains GitHub Project board
- Assigns owners and due dates
- Tracks blockers daily
- Controls scope

---

## 10) Communication & Collaboration Norms
- Use GitHub Issues/PR comments for decisions (audit trail)
- Telegram for quick questions
- Summarize critical outcomes in GitHub

---

## 11) Release & Hotfix
- Release: develop → main via PR, tag version
- Hotfix: from main, merge to main and back to develop

---

## 12) Disagreement / Escalation
1. Try to resolve with data
2. Timebox discussion (30-60 min)
3. Coordinator summarizes → Owner decides
