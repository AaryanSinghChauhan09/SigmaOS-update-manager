# SigmaOS Update Manager

Rolling release update orchestration with Observer, State, and Strategy patterns.

## OOPS Principles

- **Observer Pattern**: Notify on update state changes
- **State Pattern**: Update states (Idle, Checking, Downloading, Installing, Verifying, Completed, Failed)
- **Strategy Pattern**: Different update strategies (Full, Security-only)
- **Singleton Pattern**: Single update manager instance

## Features

- Automated updates with scheduling
- Pre-update snapshot creation
- Post-update verification
- Rollback on failure

## Usage

```bash
sigmaos-update-manager full
sigmaos-update-manager security
```

## Testing

```bash
cargo test --offline
```
