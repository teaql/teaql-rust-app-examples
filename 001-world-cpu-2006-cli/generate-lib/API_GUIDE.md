# API Guide — `fifa-world-cup-2026-service`

> Domain-specific query, mutation, and expression APIs for this project.
> For framework-level runtime APIs (UserContext, SmartList, etc.), see `TOOL_API.md`.

---

## AI Agent Notice: Runtime Logging & Trace

TeaQL Runtime provides rich, automated trace logs for all SQL executions and state mutations. AI Agents are **STRONGLY ENCOURAGED** to evaluate and observe the system through these logs instead of inspecting the raw database directly.

You can configure the logging output via environment variables when running the target binary or tests:
- `TEAQL_LOG_ENDPOINT`: Set to a file path (e.g., `eval.log`) to redirect logs to a specific file. If unset, it prints to standard output.
- `TEAQL_LOG_FORMAT`: Set to `json` (or `debug`) for structured machine-readable log entries, or `human` (default) for beautifully formatted trace chains.

Example for automated evaluation:
\`\`\`bash
TEAQL_LOG_ENDPOINT=eval.log TEAQL_LOG_FORMAT=json cargo run --bin my_app
\`\`\`

---

## Part 1 — API Rules

### 1. Query Chain

Every query follows the same shape:

```rust
use fifa_world_cup_2026_service::{Q, E};

let results = Q::<entity_plural>()            // 1. entry point
    .with_name_contains("search")              // 2. filters
    .order_by_create_time_desc()               // 3. ordering
    .page(1, 20)                               // 4. pagination
    .comment("List active items")              // 5. intent comment
    .purpose("Load dashboard data")            // 6. purpose (unlocks execute)
    .execute_for_list(&ctx).await?;            // 7. execute
```

**Execution methods** (available on `PurposedQuery`):

| Method | Returns |
|--------|---------|
| `.execute_for_list(&ctx).await?` | `SmartList<Entity>` — paginated list |
| `.execute_for_first(&ctx).await?` | `Option<Entity>` — first match |
| `.execute_for_one(&ctx).await?` | `Option<Entity>` — single match |
| `.execute_for_count(&ctx).await?` | `u64` — total count |
| `.execute_for_page(&ctx, offset, limit).await?` | `SmartList<Entity>` with `total_count` |
| `.execute_for_records(&ctx).await?` | `SmartList<Record>` — raw records |

**Check methods** (available on base Request, without `purpose`):

| Method | Returns |
|--------|---------|
| `.execute_for_exists(&ctx).await?` | `bool` — existence check |

**Pagination helpers:**

| Method | Meaning |
|--------|---------|
| `.page(page_number, page_size)` | 1-based page number |
| `.page_offset(offset, limit)` | 0-based offset |
| `.top(n)` | Limit to first N results |
| `.unlimited()` | Remove default 200-row limit |

### 2. Filter Operators

Filters use a **human/thing** naming convention derived from each entity.

- **Things** — filter prefix is `with_`: `with_name_is("X")`, `with_name_contains("X")`
- **Humans** — filter prefix is `whose_`: `whose_name_is("X")`, `whose_name_containing("X")`

The suffix also changes for string operations:

| Entity type | Prefix | Verb suffix | Example |
|-------------|--------|-------------|---------|
| Thing | `with_` | `_contains` / `_starts_with` | `with_title_contains("rust")` |
| Human | `whose_` | `_containing` / `_starting_with` | `whose_name_containing("alice")` |

**Available filter methods per field:**

| Method pattern | SQL equivalent |
|----------------|----------------|
| `<prefix>_<field>_is(value)` | `field = value` |
| `<prefix>_<field>_is_not(value)` | `field != value` |
| `<prefix>_<field>_greater_than(value)` | `field > value` |
| `<prefix>_<field>_less_than(value)` | `field < value` |
| `<prefix>_<field>_between(lo, hi)` | `field BETWEEN lo AND hi` |
| `<prefix>_<field>_between_range(DateRange)` | Time range filter |
| `<prefix>_<field>_in([...])` | `field IN (...)` |
| `<prefix>_<field>_not_in([...])` | `field NOT IN (...)` |
| `<prefix>_<field>_contain<suffix>(s)` | `field LIKE '%s%'` |
| `<prefix>_<field>_start<suffix>_with(s)` | `field LIKE 's%'` |
| `<prefix>_<field>_end<suffix>_with(s)` | `field LIKE '%s'` |
| `<prefix>_<field>_is_unknown()` | `field IS NULL` |
| `<prefix>_<field>_is_known()` | `field IS NOT NULL` |
| `<prefix>_<field>_before(value)` | `field < value` (temporal alias) |
| `<prefix>_<field>_after(value)` | `field > value` (temporal alias) |

**Ordering methods:**

| Method | SQL |
|--------|-----|
| `.order_by_<field>_asc()` | `ORDER BY field ASC` |
| `.order_by_<field>_desc()` | `ORDER BY field DESC` |

### 3. Entity Field Methods

For each scalar field `p` on an entity, the generated struct provides:

```rust
entity.p()                  // read current value
entity.update_p(value)      // stage an update (returns &mut Self)
entity.changed_p()          // check if p was changed (returns Option<Value>)
```

For object-relation fields:

```rust
entity.relation()           // Option<&RelatedEntity>
```

For reverse-relation (child) collections:

```rust
entity.children_list()      // &Vec<ChildEntity> or &SmartList<ChildEntity>
entity.children_list_mut()  // &mut Vec<ChildEntity>
```

### 4. Relation Methods on Queries

**Load a relation** (eager-load the related entity):

```rust
Q::<entity_plural>()
    .select_<relation>()                           // default sub-select
    .select_<relation>_with(Q::<related>()...)    // custom sub-query
```

**Filter by relation** (EXISTS / IN subquery):

```rust
Q::<entity_plural>()
    .with_<relation>_matching(Q::<related>()...)      // keep matching
    .without_<relation>_matching(Q::<related>()...)   // exclude matching
    .have_<relation_plural>()                           // has any children
    .have_no_<relation_plural>()                        // has no children
```

### 5. Constant Status Shortcuts

When a field references a **constant entity** (e.g. `Status`, `Type`), the code generator creates shortcut methods:

```rust
// Query filters (on the request)
Q::<entities>()
    .<prefix>_is_<code>()           // e.g. .with_status_is_active()
    .<prefix>_is_not_<code>()       // e.g. .with_status_is_not_active()

// Entity mutations (on the struct)
entity.update_<relation>_to_<code>()   // e.g. entity.update_status_to_active()
entity.<relation>_is_<code>()          // e.g. entity.status_is_active() -> bool
```

### 6. Mutation Patterns

**Create:**

```rust
let mut entity = Q::<entities>().purpose("Create example entity").new_entity(&ctx);
entity.update_name("Example");
entity.update_status_to_active();
let saved = entity.audit_as("Create new item").save(&ctx).await?;
```

**Update:**

```rust
let mut entity = Q::<entities>()
    .filter_by_id(id)
    .comment("Load for update")
    .purpose("Update item name")
    .execute_for_one(&ctx).await?
    .expect("entity not found");

entity.update_name("New Name");
entity.audit_as("Rename item").save(&ctx).await?;
```

**Delete:**

```rust
entity.mark_as_delete();
entity.audit_as("Remove obsolete item").save(&ctx).await?;
```

**Graph save** — a single `save()` persists the entity and all attached child entities:

```rust
let mut parent = Q::<parents>().purpose("Create parent entity").new_entity(&ctx);
parent.update_name("Parent");

let mut child = Q::<children>().purpose("Create child entity").new_entity(&ctx);
child.update_title("Child");
parent.children_list_mut().push(child);

parent.audit_as("Create parent with child").save(&ctx).await?;
```

### 7. Expression Facade (`E`)

`E` provides a safe, chainable way to extract values from loaded entities:

```rust
let entity = Q::<entities>()
    .filter_by_id(id)
    .select_<relation>()
    .comment("Load with relation")
    .purpose("Extract relation field")
    .execute_for_one(&ctx).await?
    .expect("not found");

let value = E::<entity_module>(entity)
    .get_<field>()
    .eval();
```

### 8. Aggregation

```rust
let records = Q::<entities>()
    .group_by_<field>()
    .aggregate_count("count")
    .aggregate_sum("<field>", "total")
    .comment("Aggregate report")
    .purpose("Dashboard stats")
    .execute_for_records(&ctx).await?;
```

You can also group by a field and attach subqueries for advanced aggregation:

```rust
let records = Q::<entities>()
    .group_by_<field>_with(Q::<entities>().aggregate_count("count"))
    .execute_for_records(&ctx).await?;
```

For multifaceted metrics where you need multiple distinct groupings or conditions in a single query:

```rust
let records = Q::<entities>()
    .facet_by_<field>_as("stats", Q::<entities>().aggregate_count("total"))
    .execute_for_records(&ctx).await?;

// Extract the faceted results
let stats = records.facet("stats");
```

---

## Part 2 — Domain Entity Graph

### `MatchStage`

| Attribute | Value |
|-----------|-------|
| Module | `match_stage` |
| Query entry | `Q::match_stages()` |
| Minimal query | `Q::match_stages_minimal()` |
| With-children query | `Q::match_stages_with_children()` |
| Expression | `E::match_stage(value)` |
| Graph save | `match_stage.audit_as("comment").save(&ctx).await` |
| New entity | `Q::match_stages().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `code`: `String` — read: `.code()`, update: `.update_code(value)`, changed: `.changed_code()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `tournament_match_list` → `TournamentMatch` — load: `.select_tournament_match_list()`, filter: `.with_tournament_match_list_matching(Q::...)`



### `MatchStatus`

| Attribute | Value |
|-----------|-------|
| Module | `match_status` |
| Query entry | `Q::match_statuses()` |
| Minimal query | `Q::match_statuses_minimal()` |
| With-children query | `Q::match_statuses_with_children()` |
| Expression | `E::match_status(value)` |
| Graph save | `match_status.audit_as("comment").save(&ctx).await` |
| New entity | `Q::match_statuses().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `code`: `String` — read: `.code()`, update: `.update_code(value)`, changed: `.changed_code()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `tournament_match_list` → `TournamentMatch` — load: `.select_tournament_match_list()`, filter: `.with_tournament_match_list_matching(Q::...)`



### `GoalCategory`

| Attribute | Value |
|-----------|-------|
| Module | `goal_category` |
| Query entry | `Q::goal_categories()` |
| Minimal query | `Q::goal_categories_minimal()` |
| With-children query | `Q::goal_categories_with_children()` |
| Expression | `E::goal_category(value)` |
| Graph save | `goal_category.audit_as("comment").save(&ctx).await` |
| New entity | `Q::goal_categories().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `code`: `String` — read: `.code()`, update: `.update_code(value)`, changed: `.changed_code()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `match_goal_list` → `MatchGoal` — load: `.select_match_goal_list()`, filter: `.with_match_goal_list_matching(Q::...)`



### `CardCategory`

| Attribute | Value |
|-----------|-------|
| Module | `card_category` |
| Query entry | `Q::card_categories()` |
| Minimal query | `Q::card_categories_minimal()` |
| With-children query | `Q::card_categories_with_children()` |
| Expression | `E::card_category(value)` |
| Graph save | `card_category.audit_as("comment").save(&ctx).await` |
| New entity | `Q::card_categories().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `code`: `String` — read: `.code()`, update: `.update_code(value)`, changed: `.changed_code()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `match_card_list` → `MatchCard` — load: `.select_match_card_list()`, filter: `.with_match_card_list_matching(Q::...)`



### `Confederation`

| Attribute | Value |
|-----------|-------|
| Module | `confederation` |
| Query entry | `Q::confederations()` |
| Minimal query | `Q::confederations_minimal()` |
| With-children query | `Q::confederations_with_children()` |
| Expression | `E::confederation(value)` |
| Graph save | `confederation.audit_as("comment").save(&ctx).await` |
| New entity | `Q::confederations().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `name`: `String` — read: `.name()`, update: `.update_name(value)`, changed: `.changed_name()`
- `code`: `String` — read: `.code()`, update: `.update_code(value)`, changed: `.changed_code()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `tournament_team_list` → `TournamentTeam` — load: `.select_tournament_team_list()`, filter: `.with_tournament_team_list_matching(Q::...)`



### `Tournament`

| Attribute | Value |
|-----------|-------|
| Module | `tournament` |
| Query entry | `Q::tournaments()` |
| Minimal query | `Q::tournaments_minimal()` |
| With-children query | `Q::tournaments_with_children()` |
| Expression | `E::tournament(value)` |
| Graph save | `tournament.audit_as("comment").save(&ctx).await` |
| New entity | `Q::tournaments().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `tournament_name`: `String` — read: `.tournament_name()`, update: `.update_tournament_name(value)`, changed: `.changed_tournament_name()`
- `host_countries`: `String` — read: `.host_countries()`, update: `.update_host_countries(value)`, changed: `.changed_host_countries()`
- `start_date`: `chrono::NaiveDate` — read: `.start_date()`, update: `.update_start_date(value)`, changed: `.changed_start_date()`
- `end_date`: `chrono::NaiveDate` — read: `.end_date()`, update: `.update_end_date(value)`, changed: `.changed_end_date()`
- `total_teams`: `i32` — read: `.total_teams()`, update: `.update_total_teams(value)`, changed: `.changed_total_teams()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Children (one-to-many):**

- `match_stage_list` → `MatchStage` — load: `.select_match_stage_list()`, filter: `.with_match_stage_list_matching(Q::...)`
- `match_status_list` → `MatchStatus` — load: `.select_match_status_list()`, filter: `.with_match_status_list_matching(Q::...)`
- `goal_category_list` → `GoalCategory` — load: `.select_goal_category_list()`, filter: `.with_goal_category_list_matching(Q::...)`
- `card_category_list` → `CardCategory` — load: `.select_card_category_list()`, filter: `.with_card_category_list_matching(Q::...)`
- `confederation_list` → `Confederation` — load: `.select_confederation_list()`, filter: `.with_confederation_list_matching(Q::...)`
- `tournament_team_list` → `TournamentTeam` — load: `.select_tournament_team_list()`, filter: `.with_tournament_team_list_matching(Q::...)`
- `match_group_list` → `MatchGroup` — load: `.select_match_group_list()`, filter: `.with_match_group_list_matching(Q::...)`
- `tournament_match_list` → `TournamentMatch` — load: `.select_tournament_match_list()`, filter: `.with_tournament_match_list_matching(Q::...)`
- `match_goal_list` → `MatchGoal` — load: `.select_match_goal_list()`, filter: `.with_match_goal_list_matching(Q::...)`
- `match_card_list` → `MatchCard` — load: `.select_match_card_list()`, filter: `.with_match_card_list_matching(Q::...)`
- `group_standing_list` → `GroupStanding` — load: `.select_group_standing_list()`, filter: `.with_group_standing_list_matching(Q::...)`



### `TournamentTeam`

| Attribute | Value |
|-----------|-------|
| Module | `tournament_team` |
| Query entry | `Q::tournament_teams()` |
| Minimal query | `Q::tournament_teams_minimal()` |
| With-children query | `Q::tournament_teams_with_children()` |
| Expression | `E::tournament_team(value)` |
| Graph save | `tournament_team.audit_as("comment").save(&ctx).await` |
| New entity | `Q::tournament_teams().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `team_name`: `String` — read: `.team_name()`, update: `.update_team_name(value)`, changed: `.changed_team_name()`
- `team_code`: `String` — read: `.team_code()`, update: `.update_team_code(value)`, changed: `.changed_team_code()`
- `emoji_flag`: `String` — read: `.emoji_flag()`, update: `.update_emoji_flag(value)`, changed: `.changed_emoji_flag()`
- `fifa_ranking`: `i32` — read: `.fifa_ranking()`, update: `.update_fifa_ranking(value)`, changed: `.changed_fifa_ranking()`
- `manager_name`: `String` — read: `.manager_name()`, update: `.update_manager_name(value)`, changed: `.changed_manager_name()`
- `group_letter`: `String` — read: `.group_letter()`, update: `.update_group_letter(value)`, changed: `.changed_group_letter()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `confederation` → `Confederation` — load: `.select_confederation()`, filter: `.with_confederation_matching(Q::...)`
- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `tournament_match_list_as_home_team` → `TournamentMatch` — load: `.select_tournament_match_list_as_home_team()`, filter: `.with_tournament_match_list_as_home_team_matching(Q::...)`
- `tournament_match_list_as_away_team` → `TournamentMatch` — load: `.select_tournament_match_list_as_away_team()`, filter: `.with_tournament_match_list_as_away_team_matching(Q::...)`
- `match_goal_list` → `MatchGoal` — load: `.select_match_goal_list()`, filter: `.with_match_goal_list_matching(Q::...)`
- `match_card_list` → `MatchCard` — load: `.select_match_card_list()`, filter: `.with_match_card_list_matching(Q::...)`
- `group_standing_list` → `GroupStanding` — load: `.select_group_standing_list()`, filter: `.with_group_standing_list_matching(Q::...)`


**Constant values for `confederation` (`Confederation`):**

- **Afc** — filter: `._is_afc()` / entity: `.update__to_afc()`, check: `._is_afc()`
- **Caf** — filter: `._is_caf()` / entity: `.update__to_caf()`, check: `._is_caf()`
- **Concacaf** — filter: `._is_concacaf()` / entity: `.update__to_concacaf()`, check: `._is_concacaf()`
- **Conmebol** — filter: `._is_conmebol()` / entity: `.update__to_conmebol()`, check: `._is_conmebol()`
- **Ofc** — filter: `._is_ofc()` / entity: `.update__to_ofc()`, check: `._is_ofc()`
- **Uefa** — filter: `._is_uefa()` / entity: `.update__to_uefa()`, check: `._is_uefa()`

### `MatchGroup`

| Attribute | Value |
|-----------|-------|
| Module | `match_group` |
| Query entry | `Q::match_groups()` |
| Minimal query | `Q::match_groups_minimal()` |
| With-children query | `Q::match_groups_with_children()` |
| Expression | `E::match_group(value)` |
| Graph save | `match_group.audit_as("comment").save(&ctx).await` |
| New entity | `Q::match_groups().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `group_letter`: `String` — read: `.group_letter()`, update: `.update_group_letter(value)`, changed: `.changed_group_letter()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `tournament_match_list` → `TournamentMatch` — load: `.select_tournament_match_list()`, filter: `.with_tournament_match_list_matching(Q::...)`
- `group_standing_list` → `GroupStanding` — load: `.select_group_standing_list()`, filter: `.with_group_standing_list_matching(Q::...)`



### `TournamentMatch`

| Attribute | Value |
|-----------|-------|
| Module | `tournament_match` |
| Query entry | `Q::tournament_matches()` |
| Minimal query | `Q::tournament_matches_minimal()` |
| With-children query | `Q::tournament_matches_with_children()` |
| Expression | `E::tournament_match(value)` |
| Graph save | `tournament_match.audit_as("comment").save(&ctx).await` |
| New entity | `Q::tournament_matches().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `match_number`: `i32` — read: `.match_number()`, update: `.update_match_number(value)`, changed: `.changed_match_number()`
- `match_date`: `chrono::NaiveDate` — read: `.match_date()`, update: `.update_match_date(value)`, changed: `.changed_match_date()`
- `venue_name`: `String` — read: `.venue_name()`, update: `.update_venue_name(value)`, changed: `.changed_venue_name()`
- `venue_city`: `String` — read: `.venue_city()`, update: `.update_venue_city(value)`, changed: `.changed_venue_city()`
- `venue_country`: `String` — read: `.venue_country()`, update: `.update_venue_country(value)`, changed: `.changed_venue_country()`
- `home_score`: `i32` — read: `.home_score()`, update: `.update_home_score(value)`, changed: `.changed_home_score()`
- `away_score`: `i32` — read: `.away_score()`, update: `.update_away_score(value)`, changed: `.changed_away_score()`
- `extra_time_home`: `i32` — read: `.extra_time_home()`, update: `.update_extra_time_home(value)`, changed: `.changed_extra_time_home()`
- `extra_time_away`: `i32` — read: `.extra_time_away()`, update: `.update_extra_time_away(value)`, changed: `.changed_extra_time_away()`
- `penalty_home`: `i32` — read: `.penalty_home()`, update: `.update_penalty_home(value)`, changed: `.changed_penalty_home()`
- `penalty_away`: `i32` — read: `.penalty_away()`, update: `.update_penalty_away(value)`, changed: `.changed_penalty_away()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `home_team` → `TournamentTeam` — load: `.select_home_team()`, filter: `.with_home_team_matching(Q::...)`
- `away_team` → `TournamentTeam` — load: `.select_away_team()`, filter: `.with_away_team_matching(Q::...)`
- `match_stage` → `MatchStage` — load: `.select_match_stage()`, filter: `.with_match_stage_matching(Q::...)`
- `match_group` → `MatchGroup` — load: `.select_match_group()`, filter: `.with_match_group_matching(Q::...)`
- `match_status` → `MatchStatus` — load: `.select_match_status()`, filter: `.with_match_status_matching(Q::...)`
- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

**Children (one-to-many):**

- `match_goal_list` → `MatchGoal` — load: `.select_match_goal_list()`, filter: `.with_match_goal_list_matching(Q::...)`
- `match_card_list` → `MatchCard` — load: `.select_match_card_list()`, filter: `.with_match_card_list_matching(Q::...)`


**Constant values for `match_stage` (`MatchStage`):**

- **Group** — filter: `._is_group()` / entity: `.update__to_group()`, check: `._is_group()`
- **RoundOf32** — filter: `._is_round_of32()` / entity: `.update__to_round_of32()`, check: `._is_round_of32()`
- **RoundOf16** — filter: `._is_round_of16()` / entity: `.update__to_round_of16()`, check: `._is_round_of16()`
- **QuarterFinal** — filter: `._is_quarter_final()` / entity: `.update__to_quarter_final()`, check: `._is_quarter_final()`
- **SemiFinal** — filter: `._is_semi_final()` / entity: `.update__to_semi_final()`, check: `._is_semi_final()`
- **ThirdPlace** — filter: `._is_third_place()` / entity: `.update__to_third_place()`, check: `._is_third_place()`
- **Final** — filter: `._is_final()` / entity: `.update__to_final()`, check: `._is_final()`

**Constant values for `match_status` (`MatchStatus`):**

- **Scheduled** — filter: `._is_scheduled()` / entity: `.update__to_scheduled()`, check: `._is_scheduled()`
- **Live** — filter: `._is_live()` / entity: `.update__to_live()`, check: `._is_live()`
- **Finished** — filter: `._is_finished()` / entity: `.update__to_finished()`, check: `._is_finished()`
- **Postponed** — filter: `._is_postponed()` / entity: `.update__to_postponed()`, check: `._is_postponed()`

### `MatchGoal`

| Attribute | Value |
|-----------|-------|
| Module | `match_goal` |
| Query entry | `Q::match_goals()` |
| Minimal query | `Q::match_goals_minimal()` |
| With-children query | `Q::match_goals_with_children()` |
| Expression | `E::match_goal(value)` |
| Graph save | `match_goal.audit_as("comment").save(&ctx).await` |
| New entity | `Q::match_goals().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `player_name`: `String` — read: `.player_name()`, update: `.update_player_name(value)`, changed: `.changed_player_name()`
- `minute_scored`: `i32` — read: `.minute_scored()`, update: `.update_minute_scored(value)`, changed: `.changed_minute_scored()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament_match` → `TournamentMatch` — load: `.select_tournament_match()`, filter: `.with_tournament_match_matching(Q::...)`
- `tournament_team` → `TournamentTeam` — load: `.select_tournament_team()`, filter: `.with_tournament_team_matching(Q::...)`
- `goal_category` → `GoalCategory` — load: `.select_goal_category()`, filter: `.with_goal_category_matching(Q::...)`
- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`


**Constant values for `goal_category` (`GoalCategory`):**

- **Normal** — filter: `._is_normal()` / entity: `.update__to_normal()`, check: `._is_normal()`
- **Penalty** — filter: `._is_penalty()` / entity: `.update__to_penalty()`, check: `._is_penalty()`
- **OwnGoal** — filter: `._is_own_goal()` / entity: `.update__to_own_goal()`, check: `._is_own_goal()`
- **FreeKick** — filter: `._is_free_kick()` / entity: `.update__to_free_kick()`, check: `._is_free_kick()`

### `MatchCard`

| Attribute | Value |
|-----------|-------|
| Module | `match_card` |
| Query entry | `Q::match_cards()` |
| Minimal query | `Q::match_cards_minimal()` |
| With-children query | `Q::match_cards_with_children()` |
| Expression | `E::match_card(value)` |
| Graph save | `match_card.audit_as("comment").save(&ctx).await` |
| New entity | `Q::match_cards().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `player_name`: `String` — read: `.player_name()`, update: `.update_player_name(value)`, changed: `.changed_player_name()`
- `minute_issued`: `i32` — read: `.minute_issued()`, update: `.update_minute_issued(value)`, changed: `.changed_minute_issued()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament_match` → `TournamentMatch` — load: `.select_tournament_match()`, filter: `.with_tournament_match_matching(Q::...)`
- `tournament_team` → `TournamentTeam` — load: `.select_tournament_team()`, filter: `.with_tournament_team_matching(Q::...)`
- `card_category` → `CardCategory` — load: `.select_card_category()`, filter: `.with_card_category_matching(Q::...)`
- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`


**Constant values for `card_category` (`CardCategory`):**

- **Yellow** — filter: `._is_yellow()` / entity: `.update__to_yellow()`, check: `._is_yellow()`
- **Red** — filter: `._is_red()` / entity: `.update__to_red()`, check: `._is_red()`
- **SecondYellow** — filter: `._is_second_yellow()` / entity: `.update__to_second_yellow()`, check: `._is_second_yellow()`

### `GroupStanding`

| Attribute | Value |
|-----------|-------|
| Module | `group_standing` |
| Query entry | `Q::group_standings()` |
| Minimal query | `Q::group_standings_minimal()` |
| With-children query | `Q::group_standings_with_children()` |
| Expression | `E::group_standing(value)` |
| Graph save | `group_standing.audit_as("comment").save(&ctx).await` |
| New entity | `Q::group_standings().purpose("purpose").new_entity(&ctx)` |
| Filter prefix | `with_` (thing) |

**Properties:**

- `id`: `u64` — read: `.id()`, update: `.update_id(value)`, changed: `.changed_id()`
- `played`: `i32` — read: `.played()`, update: `.update_played(value)`, changed: `.changed_played()`
- `won`: `i32` — read: `.won()`, update: `.update_won(value)`, changed: `.changed_won()`
- `drawn`: `i32` — read: `.drawn()`, update: `.update_drawn(value)`, changed: `.changed_drawn()`
- `lost`: `i32` — read: `.lost()`, update: `.update_lost(value)`, changed: `.changed_lost()`
- `goals_for`: `i32` — read: `.goals_for()`, update: `.update_goals_for(value)`, changed: `.changed_goals_for()`
- `goals_against`: `i32` — read: `.goals_against()`, update: `.update_goals_against(value)`, changed: `.changed_goals_against()`
- `goal_difference`: `i32` — read: `.goal_difference()`, update: `.update_goal_difference(value)`, changed: `.changed_goal_difference()`
- `points`: `i32` — read: `.points()`, update: `.update_points(value)`, changed: `.changed_points()`
- `standing_rank`: `i32` — read: `.standing_rank()`, update: `.update_standing_rank(value)`, changed: `.changed_standing_rank()`
- `create_time`: `chrono::DateTime<chrono::Utc>` — read: `.create_time()`, update: `.update_create_time(value)`, changed: `.changed_create_time()`
- `update_time`: `chrono::DateTime<chrono::Utc>` — read: `.update_time()`, update: `.update_update_time(value)`, changed: `.changed_update_time()`
- `version`: `i64` — read: `.version()`, update: `.update_version(value)`, changed: `.changed_version()`

**Relations (outgoing):**

- `tournament_team` → `TournamentTeam` — load: `.select_tournament_team()`, filter: `.with_tournament_team_matching(Q::...)`
- `match_group` → `MatchGroup` — load: `.select_match_group()`, filter: `.with_match_group_matching(Q::...)`
- `tournament` → `Tournament` — load: `.select_tournament()`, filter: `.with_tournament_matching(Q::...)`

