# TeaQL Coding Agent Instructions

This project uses the TeaQL-generated Rust crate `fifa-world-cup-2026-service`.

## Core Rule

Always prefer TeaQL-generated APIs over handwritten data-access code.

Business code must stay on the supported surface: `Q` for reads, generated
entity APIs and `entity.save(&ctx).await` for state changes, `E` for safe
expression access, and project-specific extensions or wrappers around
`UserContext` for request context behavior. Accessing lower-level TeaQL runtime
interfaces directly is not allowed from business code.

Do not hand-write SQL, repository orchestration, relation loading, DTO mapping,
or graph persistence unless the user explicitly asks for a low-level escape
hatch.

## Application Layer Safety Guardrails

### 1. Do NOT Use Repository API for Domain Entities
- Never use `save_entity_graph_from()`, `save_entity_with_comment()`, or any other repository-level methods directly on a `Repository` or `ResolvedRepository` instance to persist entities.
- Saving via the repository graph API executes a full entity graph replacement. If child relations are not explicitly loaded and re-attached, the framework will implicitly delete the missing records.

### 2. Use the Entity API Exclusively
- Always perform state transitions or updates directly on the domain entity object itself, and call `.save(&ctx).await`.
- Data MUST NOT be deleted implicitly. Unless `.mark_as_delete()` is explicitly called, no command should delete any data.

### 3. ABSOLUTE BAN ON `T::` TOOLS
- Inside the application layer, you are strictly forbidden from calling any stateless utility from `teaql_tool::T` directly.
- All side effects (network, file) and all stateful computations (time, formatting, ID generation) MUST go through the user context (`ctx`).
- Do not use `chrono::Utc`, `chrono::Local`, `std::fs`, `reqwest`, or `std::process::Command` directly.

### 4. MANDATORY BUSINESS INTENT (`.comment()`)
- Every tool call MUST end with `.comment("English intent description")`.
- Pure computation tools return `MustComment` — a zero-cost wrapper that cannot be used without calling `.comment()`. The compiler will reject code that skips this step.

```rust
// Correct: context-aware computation with mandatory intent
let deadline = ctx.time().today().add_days(7).comment("Calculate grace period deadline");
let id = ctx.id().uuid().comment("Generate trace ID for callback");

// Correct: audited IO with mandatory intent
let data = ctx.http().get("https://...").comment("Sync external tasks").await?;

// WRONG: compiler error — T:: is banned, MustComment cannot be unwrapped
let now = T::time().today();
let raw = std::fs::read("file.txt");
```

## Generated Crate

Import the generated domain API from `fifa-world-cup-2026-service`:

```rust
use fifa_world_cup_2026_service::{E, Q};
```

The generated crate provides:

- entity structs for the domain objects
- query facade `Q`
- safe expression facade `E`
- generated request builders
- relation loading helpers
- graph save helpers, such as `entity.save(&ctx).await`
- runtime registration helpers:
  - `module()`
  - `module_with_behaviors()`
  - `repository_registry()`
  - `behavior_registry()`

## MANDATORY AUDIT RULE (Zero-cost Intent Logging)

Whenever you query or persist data, you MUST chain a comment explaining your business intent. This allows the system to build an automatic audit trail.
- For queries: chain `.comment("...")` for audit behavior and `.purpose("...")` before execution for trace-chain intent.
- For updates/saves: chain `.set_comment("...")` before saving.

## CRUD & Query Patterns

### 1. Querying (Read)
Use `Q` for reads. Always include `.comment()` to record audit behavior and `.purpose()` to record query intent in the trace chain.

```rust
let rows = Q::tournaments()
    .comment("Fetch tournaments for processing")
    .select_self()
    .page(1, 20)
    .purpose("List tournaments page for processing")
    .execute_for_list(&ctx)
    .await?;
```

Avoid direct `sqlx::query(...)` unless raw SQL is explicitly requested. Do NOT call generated repositories directly.

### 2. Creating (Create)
Use `Q::tournaments().purpose("purpose").new_entity(&ctx)` to create a new entity with the correct root context, then use graph save:

```rust
let mut entity = Q::tournaments().purpose("Create example entity").new_entity(&ctx);
// entity.update_name("example");
entity.set_comment("Created new Tournament for user request")
      .save(&ctx).await?;
```

### 3. Updating (Update)
Fetch the graph node, use generated typed setters to modify fields, and append intent before saving:

```rust
if let Some(mut entity) = Q::tournaments()
    .with_id_is(id)
    .purpose("Load Tournament for update")
    .execute_for_one(&ctx)
    .await?
{
    // entity.update_status(new_status)
    entity.set_comment("Updating status due to state transition")
          .save(&ctx).await?;
}
```

### 4. Audited Soft-Delete (Delete)
Do NOT call `repo.delete`. Use the elegant `mark_as_delete` method chained with `set_comment`:

```rust
if let Some(mut entity) = Q::tournaments()
    .with_id_is(id)
    .purpose("Load Tournament for soft delete")
    .execute_for_one(&ctx)
    .await?
{
    entity.mark_as_delete()
          .set_comment("Soft deleted Tournament as requested")
          .save(&ctx).await?;
}
```

## Advanced TeaQL Paradigms

### Dynamic JSON Filtering
When building multi-condition UI filters, do NOT write complex `if-else` query builders. Use dynamic JSON filtering:


```rust
let items = Q::<tournaments>()
    .comment("Search with dynamic UI filters")
    .filter_with_json(filter_json_value)
    .purpose("Search tournaments with dynamic UI filters")
    .execute_for_list(&ctx).await?;
```

### Faceted Aggregation
For dashboard metrics and grouping, use generated facet methods to let the database handle aggregation:

```rust
let aggregations = Q::<tournaments>()
    .comment("Aggregate data for dashboard")
    // .facet_by_status_as("status_stats")
    .purpose("Aggregate tournaments data for dashboard")
    .execute_for_list(&ctx).await?;
```

### Partial Projections & DTOs
When only a few fields are needed, avoid loading the full entity graph. Project specific columns into a custom Rust struct (`return_type::<T>()`):

```rust
// let stats = Q::<tournaments>()
//     .comment("Fetch lightweight specific fields")
//     .select_status()
//     .count_id_as("count")
//     .group_by_status()
//     .return_type::<StatusStatsDTO>()
//     .purpose("Fetch lightweight status statistics")
//     .execute_for_list(&ctx).await?;
```

### Domain Behavior Injection
NEVER manually edit generated Entity files. Inject business logic by defining Rust Extension Traits in your application logic (`service.rs`):

```rust
pub trait <Tournament>Ext {
    fn custom_business_logic(&mut self);
}

impl <Tournament>Ext for <Tournament> {
    fn custom_business_logic(&mut self) {
        // ...
    }
}
```

## Relation Loading

Use generated relation helpers. Avoid manual N+1 query loops.

```rust
Q::<entity_plural>()
    .select_<relation>_with(Q::<target_plural>().select_self())
```

## Standard Filtering

Use generated readable filters when available. Use direct `teaql_core::Expr` only when no generated helper exists.

```rust
Q::<entity_plural>()
    .which_<fields>_is(...)
    .with_<relation>_matching(Q::<target_plural>().select_self())
```

## Low-Level Warnings

Do not manually coordinate multiple repository insert/update calls unless the task explicitly requires low-level control.
Do not call `runtime_new(...)`, `entity_root()`, repositories, or runtime internals to create entities.

## Safe Value Access

Use `E` for long-chain value access:

```rust
let value = E::<entity>(entity)
    .get_<relation>()
    .get_<field>()
    .eval();
```

Do not write nested `unwrap()` chains for optional relations.

## Runtime Setup

Use the generated runtime helper as the default application entrypoint:

```rust
let ctx = fifa_world_cup_2026_service::service_runtime_from_env().await?;
```

This helper reads the generated database environment variables, connects to the
data service, registers the generated module, repositories, behaviors, and
checkers, and calls `ensure_schema().await?`. `ensure_schema()` is the standard
schema and seed-data path; it applies the generated `initial_graph` entries for
root and constant data. Do not hand-write duplicate seed `INSERT` statements for
generated constants.

If a connection pool already exists, use:

```rust
let ctx = fifa_world_cup_2026_service::service_runtime_from_pool(pool).await?;
```

Manual `UserContext` assembly is an advanced customization path only. Use it
only when you deliberately need a custom runtime and understand which generated
registries, behaviors, checkers, resources, schema setup, and initial graphs you
are bypassing:

```rust
let ctx = teaql_runtime::UserContext::new()
    .with_module(fifa_world_cup_2026_service::module())
    .with_repository_registry(fifa_world_cup_2026_service::repository_registry())
    .with_repository_behavior_registry(fifa_world_cup_2026_service::behavior_registry());
```

Use `module_with_behaviors()` or `module_with_behaviors_and_checkers()` in this
manual path only when behavior hooks or checkers should be active.

## SQL Debugging

If query behavior is unclear, enable TeaQL SQL logs through `UserContext`:

```rust
ctx.enable_all_sql_log();
let logs = ctx.sql_logs();
```

Use the debug SQL to explain behavior or diagnose performance.

## Schema Changes

If a requested change requires new entities, fields, relations, constants, or
modules, update the KSML model first and regenerate the crate.

Do not manually edit generated entity/request/expression files. Treat generated
files as disposable.


## Domain Context

Read these files before making domain-specific changes:

- `docs/teaql-domain-map.md`
- `docs/teaql-query-examples.md`
- `docs/teaql-save-graph.md`
- `docs/teaql-sql-log.md`