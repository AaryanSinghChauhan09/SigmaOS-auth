# SigmaOS-auth

Authentication and authorization framework for the SigmaOS project.

This repository owns the focused authentication and authorization framework component in SigmaOS's multi-repository architecture. The crate manifest and src/ tree are the source of truth for its current API and implementation status.

## Development

Use the Rust toolchain declared by the repository. Run cargo check and the available crate tests before proposing implementation changes. Keep component boundaries explicit, document upstream behavior that informs changes, and include rollback considerations for user-visible or system-level behavior.

## Status

This overview describes repository scope; it does not claim that every planned feature is implemented or production-ready.

Password hash matching uses a constant-time byte comparison. Account and
password expiration checks require a trusted caller-supplied timestamp;
requests with configured expiry but no timestamp fail with `ServiceError`.
Session creation/closure and password change currently return `ServiceError`
because no persistent session or password-update backend is implemented.
