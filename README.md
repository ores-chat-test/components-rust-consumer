# components-rust-consumer

Status: **contract-only**. This suite specifies external Rust crate consumption, feature isolation, and typed chat client integration.

This repository is an executable acceptance-suite boundary, not evidence that the corresponding product capability is complete. The suite must target both `ores-chat` and the isolated `ores-chat-test` fixture. Promotion to `live` requires hosted execution, deterministic assertions, and redacted retained evidence.

The machine-readable plan is in `suite.json` and is validated by the organization policy action pinned to an immutable commit. Public, customer, administrator, and internal-service identities are never interchangeable.

## Role

Component of the `ores-chat-test` organization.

Part of the [`ores-chat-test`](https://github.com/ores-chat-test) organization.

Cross-language contracts in this organization are governed by human-authored TypeSpec and human-authored JSON Schema Draft 2020-12 as **independent peer authorities**, with parity enforced by [`ORESoftware/typespec-json-schema-validator`](https://github.com/ORESoftware/typespec-json-schema-validator); generated schemas and clients are comparison evidence only, never a third authority.
