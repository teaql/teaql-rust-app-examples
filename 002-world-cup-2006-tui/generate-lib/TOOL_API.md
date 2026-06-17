# TeaQL Tool & Runtime API Reference

> Framework-level APIs. These are the same for all TeaQL Rust projects.
> For domain-specific APIs (entities, queries, mutations), see `API_GUIDE.md`.

---

## 1. UserContext — Application Entry Point

`UserContext` is the runtime context passed to all queries and mutations.

### Default Setup (in main.rs)
Use the generated runtime helper first:

```rust
let ctx = fifa_world_cup_2026_service::service_runtime_from_env().await?;
```

This helper reads the generated database environment variables, connects to the
data service, registers generated modules/repositories/behaviors/checkers,
inserts the SQL runtime resources, and calls `ensure_schema().await?`.

If you already have a pool, use:

```rust
let ctx = fifa_world_cup_2026_service::service_runtime_from_pool(pool).await?;
```

Do not duplicate generated constant seed data with manual SQL `INSERT`
statements. The generated module registers root and constant seed data through
`initial_graph`, and the default runtime setup applies that path through
`ensure_schema()`.

### Advanced Manual Setup
Manual `UserContext` assembly is only for custom runtime integration. If you use
it, you are responsible for matching the generated runtime helper's registration
and schema setup behavior.

```rust
let ctx = UserContext::new()
    .with_module(module())                          // entity metadata
    .with_repository_registry(repository_registry()) // generated repositories
    .with_repository_behavior_registry(behavior_registry()) // generated behaviors
    .with_checker_registry(checker_registry())       // generated checkers
    .with_user_identifier("system")                  // who is operating
    .with_timezone("Asia/Shanghai");                 // timezone for date ops
ctx.ensure_schema().await?;                          // auto-create/migrate tables
```

### Key Methods
| Method | Description |
|--------|-------------|
| `.set_user_identifier(id)` | Set who is performing actions |
| `.user_identifier()` | Get current user ID |
| `.set_timezone(tz)` | Set timezone for date operations |
| `.ensure_schema().await?` | Auto-create or migrate database tables |
| `.entity(name)` | Get entity metadata by name |
| `.all_entities()` | List all registered entity descriptors |
| `.next_id(entity_name)` | Generate next ID for an entity |
| `.commit_changes().await?` | Commit pending transactions |
| `.put_local(key, value)` | Store request-scoped key-value |
| `.local(key)` | Retrieve request-scoped value |
| `.insert_resource(obj)` | Store a typed resource |
| `.get_resource::<T>()` | Retrieve a typed resource |

---

## 2. SmartList — Collection with Metadata

`SmartList<T>` wraps `Vec<T>` with pagination metadata.

| Method | Description |
|--------|-------------|
| `.len()` | Number of items in current page |
| `.total_count()` | Total matching records (for pagination) |
| `.is_empty()` | Check if list is empty |
| `.iter()` / `.iter_mut()` | Iterate items |
| `.with_total_count(n)` | Set total count |
| `.with_facet(name, list)` | Attach aggregation facet |
| `.facet(name)` | Get a named facet |
| `.facets()` | Get all facets |

---

## 3. WebResponse — HTTP Response Builder

```rust
// From single entity
let response = WebResponse::from_entity(&task);

// From list with pagination
let response = WebResponse::from_smart_list(task_list);

// Error response
let response = WebResponse::fail("Invalid request");
```

---

## 4. AuditConfig — Logging Control

### Environment Variables
| Variable | Values | Default |
|----------|--------|---------|
| `TEAQL_AUDIT` | `verbose`, `production`, `silent` | `production` |
| `TEAQL_SCHEMA` | `execute`, `dry_run`, `verify` | `execute` |
| `TEAQL_SQL_LOG` | `all`, `select`, `mutation`, `off` | `off` |

### Programmatic Config
```rust
let config = AuditConfig::production();
let config = AuditConfig::verbose_all();
let config = AuditConfig::silent_all();
let config = AuditConfig::focus_on(Module::Mutation);
```

---

## 5. Schema Management

```rust
ctx.ensure_schema().await?;
// Auto-creates tables and adds new columns. Never drops columns.
```

| SchemaMode | Behavior |
|------------|----------|
| `Execute` | Apply schema changes automatically |
| `DryRun` | Log what would change, don't apply |
| `Verify` | Fail if schema doesn't match |

---

## 6. Save Pipeline

When `entity.audit_as("comment").save(&ctx).await?` is called:
1. Enforces `audit_as()` is set (Triple-Intent)
2. Runs checkers
3. Diffs entity against database state
4. Generates INSERT/UPDATE/DELETE SQL
5. Writes audit log with comment
6. Fires entity events

> Never write raw SQL. Use the generated entity APIs.

---

## 7. Value Types

| Rust Type | Model Type | DB Mapping |
|-----------|-----------|------------|
| `String` | `string()` | VARCHAR |
| `i64` | `number()` | BIGINT |
| `f64` | `money()` / `decimal()` | DECIMAL |
| `bool` | `boolean()` | BOOLEAN |
| `chrono::NaiveDate` | `date()` | DATE |
| `chrono::NaiveDateTime` | `dateTime()` / `createTime()` | DATETIME |
| `serde_json::Value` | `json()` | TEXT (JSON) |
| `u64` | `id()` | BIGINT (FK) |

---

## 8. DateRange — Time Period Filtering

```rust
use teaql_tool_std::DateRange;

request.with_create_time_between_range(DateRange::today());
request.with_create_time_between_range(DateRange::this_month());
request.with_create_time_between_range(DateRange::last_n_days(7));
```

---

## 9. XlsWorkbook — Excel Export

```rust
let page = XlsPage::new("orders")
    .add_block(XlsBlock::new("orders", 0, 0, "Order Report").span(3, 1));
let workbook = XlsWorkbook::new().add_page(page);
let json = workbook.to_json_value();
```