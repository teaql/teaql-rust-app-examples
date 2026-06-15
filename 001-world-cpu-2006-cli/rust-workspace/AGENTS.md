# TeaQL Rust Workspace Instructions

> [!WARNING]
> **PARADIGM SHIFT WARNING: DO NOT GUESS METHOD NAMES**
> TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
> You must clear your memory of traditional ORMs (Diesel, SeaORM, etc.).
> There are no standard setter methods. The entity structure is entirely proprietary and dynamically generated.
> **RULE:** If you encounter a compilation error like `no method named update_xxx found` or similar, **ABSOLUTELY DO NOT GUESS** another method name. You must immediately use the `view_file` tool to read the generated source code for the entity and find the exact method signature, or stop and ask the user for help.

This workspace is the editable Rust application layer for the generated TeaQL domain crate `fifa-world-cup-2026-service`.

## Project Structure

```text
.
├── Cargo.toml                      # Workspace root configuration
├── README.md                       # Quick start and developer guide
├── build/
│   ├── API_GUIDE.md                # (READ THIS FIRST) domain-specific query/mutation APIs
│   └── TOOL_API.md                 # Framework runtime reference (UserContext, SmartList)
├── lib/
│   └── fifa_world_cup_2026_service/    # Generated domain crate (Do not edit generated files)
├── models/
│   └── fifa-world-cup-2026-service.xml           # Source of truth (KSML model)
└── src/
    ├── lib.rs                      # Application library
    └── main.rs                     # Application entry point
```

## API Rules (Quick Reference)

| Stage | Methods | On Struct | Description |
|-------|---------|-----------|-------------|
| **1. Filter** | `with_<>_is(v)`, `with_<>_matching(Q...)` | `<>Request<>` | Specify query conditions |
| **2. Load** | `select_<>()`, `select_<>_with(Q...)` | `<>Request<>` | Eager load related entities |
| **3. Page** | `.page(n,s)`, `.top(n)`, `.unlimited()` | `<>Request<>` | Apply sorting and pagination |
| **4. Comment** | `.comment("what")` | `<>Request<>` | Document the database intent |
| **5. Purpose** | `.purpose("why")` | `<>Request<>` | Transition to `PurposedQuery<>` |
| **6. Execute** | `.execute_for_list(ctx)` | `PurposedQuery<>` | Perform database execution |

> [!NOTE]
> For complete documentation, see the generated [`build/API_GUIDE.md`](./build/API_GUIDE.md) and [`build/TOOL_API.md`](./build/TOOL_API.md).

> [!CAUTION]
> **AI AGENT GUARDRAIL: DO NOT BYPASS TEAQL ON COMPILATION/RUNTIME ERRORS**
> If you encounter compilation errors (e.g., `no method named save found` in Rust) or runtime validation errors (e.g., `Missing: .audit_as()` or `Missing: .purpose()`):
> 1. **DO NOT** attempt to write direct SQL, direct SQLx queries, or bypass the TeaQL API.
> 2. **DO NOT** add or use external libraries (e.g., SeaORM, Diesel, rusqlite) to access the database.
> 3. **DO NOT** write custom Repository or DAO implementations.
> 4. **THE ONLY CORRECT FIX** is to apply the **Triple-Intent Audit Pattern**:
>    - **Rust Entity Save**: You must call `.audit_as("comment")` to wrap the entity before saving: `entity.audit_as("action comment").save(&ctx).await?`
>    - **Queries**: You must chain `.comment("what is loaded").purpose("why it is loaded")` before calling the execution method (e.g., `.execute_for_list(&ctx).await?`).
> Any workaround that bypasses TeaQL will violate database safety constraints and fail CI.

## Boundaries

- Keep the KSML model under `../models`.
- Keep generated TeaQL runtime code under `../generate-lib`.
- Do not edit generated files under `../generate-lib/lib`.
- Do not edit generated library files. If generated APIs are missing or awkward, update the KSML model or report the missing TeaQL API and regenerate.
- Put application queries, services, tests, and integration code in this workspace.
- Do not use other database access technologies from workspace business code. This includes direct SQLx queries, SeaORM, Diesel, rbatis, rusqlite, tokio-postgres, mysql_async, raw database clients, and hand-written repository/DAO layers.
- Do not add dependencies or helper wrappers that bypass TeaQL for persistence, querying, transactions, relation loading, or DTO mapping.

## Application Layer Safety Guardrails

### 1. Use the Entity API Exclusively
- Always perform state transitions or updates directly on the domain entity object itself, and call `.audit_as("comment").save(&ctx).await`.
- Never use repository-level methods (`save_entity_graph_from()`, etc.) directly. They execute full graph replacements and may implicitly delete child records.
- Data MUST NOT be deleted implicitly. Unless `.mark_as_delete()` is explicitly called, no command should delete any data.

### 2. ABSOLUTE BAN ON `T::` TOOLS
- Inside the application layer, you are strictly forbidden from calling any stateless utility from `teaql_tool::T` directly.
- All side effects (network, file) and all stateful computations (time, formatting, ID generation) MUST go through the user context (`ctx`).
- Do not use `chrono::Utc`, `chrono::Local`, `std::fs`, `reqwest`, or `std::process::Command` directly.

### 3. MANDATORY BUSINESS INTENT (`.comment()`)
- Every tool call MUST end with `.comment("English intent description")`.
- Pure computation tools return `MustComment<T>` — a zero-cost wrapper that cannot be used without calling `.comment()`. The compiler will reject code that skips this step.

```rust
// Correct
let deadline = ctx.time().today().add_days(7).comment("Calculate grace period deadline");
let data = ctx.http().get("https://...").comment("Sync external tasks").await?;

// WRONG: compiler error
let now = T::time().today();
```

### 4. TRIPLE-INTENT AUDIT ENFORCEMENT
- **Mutations (Saves)**: You CANNOT call `entity.save(&ctx).await` directly. The entity struct itself does not implement a direct `.save()` method. To persist changes, you MUST call `.audit_as("Comment describing the business action")` which wraps the entity in `Audited` and unlocks the `.save(&ctx).await?` method.
  ```rust
  // Correct
  entity.audit_as("Assign task to robot").save(&ctx).await?;

  // WRONG: will fail to compile (no method named `save` found)
  entity.save(&ctx).await?;
  ```
- **Queries**: Every select/query chain MUST call `.comment("What the query loads")` and `.purpose("Why the query is needed")` before calling the execution method (e.g. `.execute_for_list(&ctx).await?`). The compiler enforces `.purpose(...)` to get a `PurposedQuery` which exposes the execute methods.
  ```rust
  // Correct
  Q::tasks()
      .with_status_is_active()
      .comment("Load active tasks")
      .purpose("Render robot kanban dashboard")
      .execute_for_list(&ctx).await?;
  ```

### 5. PREVENTING STACK OVERFLOW (CYCLIC REFERENCES & MASSIVE GRAPHS)
- **Graph Saving is the Core Advantage**: TeaQL encourages building an object graph in memory and saving it all at once via `.audit_as("...").save(&ctx).await?`. This is the standard and preferred way to handle business logic.
- **Cyclic Reference Hazard**: During `save()`, TeaQL recursively traverses loaded relation objects to build the mutation plan. If you have a cyclic reference in memory (e.g., `tenant` has a loaded `owner` pointing to `user`, and `user` has a loaded `tenant` pointing back to `tenant`), calling `.save()` will cause a **Stack Overflow**.
  - **The Fix**: You MUST break the loop in memory before calling `.save()`. Set one side of the relation (usually the back-reference or parent link on the child entity) to `None` or clear the children list before saving.
- **Massive Graph Hazard (Sample Data)**: While saving graphs is encouraged, building an excessively large graph (e.g., attaching > 20 child items to a root entity during system initialization or sample data generation) can cause a `Stack Overflow` in debug builds due to massive stack frame allocation.
  - **The Fix**: If you are generating seed data or sample data exceeding 20 items, DO NOT build a single massive graph. You must chunk the creation into smaller loops or save the children independently.

If the domain needs new entities, fields, relations, constants, or generated API names, update the model and regenerate the library before changing workspace code.

## Generated Crate

The generated crate is consumed by local path:

```toml
fifa_world_cup_2026_service = { package = "fifa-world-cup-2026-service", path = "../generate-lib/lib" }
```

When refreshing the generated crate, extract or copy the `rust-lib` output into `../generate-lib` while preserving the top-level `lib/` directory. The expected manifest path is `../generate-lib/lib/Cargo.toml`; do not flatten `lib/*` into `../generate-lib/`.
Before writing code against the generated TeaQL crate, read the library guide at `../generate-lib/lib/AGENTS.md`. If the crate comes from a Cargo registry instead of a local path, locate the unpacked crate source with `cargo metadata` or vendor the dependency with `cargo vendor`, then read that crate's `AGENTS.md` before using its APIs.

Import generated APIs from `fifa_world_cup_2026_service`:

```rust
use fifa_world_cup_2026_service::{E, Q};
```

Use generated query builders, expression helpers, relation selectors, update helpers, and graph save APIs before considering lower-level data access.
Business code may use only the supported TeaQL surface: `Q`, generated entity APIs, `entity.audit_as("comment").save(&ctx).await`, `E`, and workspace-specific extensions or wrappers around `UserContext`.
Create new generated entities with `Q::tournaments().purpose("purpose").new_entity(&ctx)` or the corresponding `Q::<entity_plural>().purpose("purpose").new_entity(&ctx)` entrypoint; do not call `runtime_new(...)`, `entity_root()`, repositories, or runtime internals to construct entities.
Do not access lower-level TeaQL internals directly: generated repositories, repository registries, metadata registries, SQL executors, transaction internals, or runtime persistence hooks.
If the supported surface is not enough, report the missing generated API instead of bypassing it.

## Runtime

`src/main.rs` is a thin Tokio async smoke entrypoint. Keep long-running business logic in `src/lib.rs` or dedicated modules and call it from the entrypoint.

Do not add a web framework unless the project explicitly needs one.

## TeaQL Rust Pitfalls

- SQLite `:memory:` is not suitable for TeaQL integration tests. TeaQL/SQLx may use a different connection from the pool, and SQLite in-memory databases are connection-local, so later operations can fail with missing tables. Use a file-backed SQLite database for integration tests.


## Checks

Run these from this workspace:

```bash
cargo check
cargo test
```