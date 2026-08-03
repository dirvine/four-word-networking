# ADR-0001: Adopt Architecture Decision Records

- **Status:** Proposed
- **Date:** 2026-08-03
- **Decision owners:** Engineering team
- **Reviewers:** Engineering team
- **Supersedes:** none
- **Superseded by:** none
- **Related:** Portfolio ADR governance rollout

## Context

Four-word networking is used by x0x and other Autonomi components at human-facing network boundaries. Its encoding, dictionary, compatibility, and API decisions therefore need durable rationale rather than relying only on implementation history and pull-request discussion.

## Decision Drivers

- Preserve reasoning behind externally visible encoding and compatibility choices.
- Make architectural trade-offs visible during review.
- Give humans and AI coding tools a reliable source of project constraints.
- Prevent silent drift from accepted decisions.

## Considered Options

1. Keep architecture reasoning only in pull requests and design notes.
2. Maintain informal design documents without lifecycle governance.
3. Adopt version-controlled Architecture Decision Records with CI governance.

## Decision

We propose maintaining Architecture Decision Records in `docs/adr/` using the repository template. New decisions start as `Proposed`; humans may mark them `Accepted` after engineering review. Accepted ADRs are immutable: a changed decision requires a new superseding ADR.

## Consequences

### Positive

- Architectural intent becomes searchable and reviewable.
- Encoding and compatibility changes retain their rationale.
- AI coding agents receive explicit constraints before changing public behaviour.

### Negative / Trade-offs

- Significant design changes require a small amount of additional documentation and review.

### Neutral / Operational

- CI validates new ADR structure and rejects changes to Accepted ADRs.

## Validation

The governance unit tests and repository-local validator must pass in CI. Human review of this Proposed ADR determines whether the practice itself is accepted.
