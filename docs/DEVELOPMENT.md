# Protea OS Development Rules

## Project posture

Protea OS is being developed as a real long-term operating system project.

"Student project" describes the available resources, not the quality target.

The code must therefore be written so that it can evolve into production software.

## Free-first policy

Before adding a paid dependency, service or platform:

1. identify the exact capability required;
2. check whether Linux/open-source tooling provides it;
3. check whether it can be implemented locally;
4. document the trade-off;
5. only then consider a paid option.

## No fake implementations

The following are not acceptable as final system functionality:

- fake login screens
- hard-coded account state
- decorative settings with no persistence
- buttons that claim to change system state but do nothing
- fake performance numbers
- fake hardware detection
- simulated boot screens presented as an OS
- cloud-only functionality for something that should work offline

A temporary development stub is allowed only when clearly marked and accompanied by an issue describing the real implementation.

## Dependency rule

Every runtime dependency should answer:

- What problem does it solve?
- Why is it needed?
- Can it run on the minimum supported hardware?
- What license does it use?
- What happens if it is unavailable?

## Testing rule

New core behavior should have tests before it becomes a dependency of another subsystem.

Hardware-dependent behavior needs both:

- deterministic tests where possible;
- real hardware tests where necessary.

## Security rule

Do not store secrets in Git.

Do not disable signature verification to make development easier unless the change is isolated to a clearly marked development configuration.

## Git rule

Keep commits small and meaningful.

Recommended prefixes:

- `docs:`
- `core:`
- `pc:`
- `phone:`
- `build:`
- `test:`
- `security:`
- `ci:`

## Completion rule

A phase is not complete because source code exists.

A phase is complete only when:

- code builds;
- tests pass;
- the feature runs on its intended environment;
- failure behavior is understood;
- documentation is updated;
- no known blocking error is ignored.
