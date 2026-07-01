# Linux System Info using TeaQL

This is an example application demonstrating how to use the `teaql-provider-linux` along with TeaQL's code generator to safely and ergonomically query Linux system information.

## Overview

The `teaql-provider-linux` allows reading from the `/proc` filesystem on Linux. However, manually assembling the data models can be error-prone. 
This example shows the recommended approach:
1. **Define a Model**: We define a `MODEL.xml` that models `SystemInfo`, `Process`, and `Thread` entities.
2. **Generate Code**: Using `teaql-code-gen`, we generate the `rust-lib-core` crate which contains type-safe Rust structs and querying APIs for these entities.
3. **Use in App**: The `rust-app-console` integrates the generated library with the `teaql-provider-linux` executor to print out system information.

## Project Structure

- `modeling/MODEL.xml`: The TeaQL XML model defining the OS entities.
- `rust-lib-core/`: The generated Rust library containing the domain model and query API.
- `rust-app-console/`: A simple CLI application that uses the generated library to fetch system info.

## Running the Example

Make sure you are on a Linux system, then run the console application:

```bash
cd rust-app-console
cargo run
```

This will initialize the `LinuxDataServiceExecutor`, mount it in the generated `UserContext`, and query the underlying system.

## What Else Can You Do With This?

Beyond just a simple console printout, combining `teaql-provider-linux` with TeaQL's powerful query engine and generated code opens up many possibilities:

1. **Custom System Monitors (Dashboards/TUIs)**: Build your own `htop`, `top`, or `glances` alternatives in Rust. Use TeaQL to easily filter, sort, and paginate processes and threads to feed into a UI framework like `ratatui`.
2. **Automated Process Management**: Write scripts or background daemons that periodically query for misbehaving processes (e.g., processes consuming >90% memory or running for too long) and automatically alert you or terminate them.
3. **Metrics Collection Agents**: Serve as a lightweight node agent to collect structured system metrics and export them to monitoring systems like Prometheus, Grafana, or an ELK stack.
4. **Security & Auditing**: Continuously query the system for unexpected or unauthorized processes based on complex criteria (e.g., specific command-line arguments, user IDs, or unexpected thread counts).
5. **Cross-Platform Abstractions**: You can swap out `teaql-provider-linux` with other platform-specific providers in the future without changing your core business logic or TeaQL queries.

## Expressive API Examples (Q:: and E::)

Because this example uses the **TeaQL Code Generator**, you don't need to write raw SQL or complex `/proc` parsing logic. Instead, you get a fully type-safe, fluent API using the `Q::` (Query Builder) and `E::` (Expression/Condition) namespaces.

### Basic Querying with `Q::`

The `Q::` API provides auto-completed methods for fetching and filtering entities. Here is how you can fetch the top 50 processes taking up the most physical memory:

```rust
use linux_system_info_core::Q;

// Fetch the top 50 processes ordered by Resident Set Size (RSS) memory
let top_memory_procs = Q::processes()
    .with_memory_vms_kb_greater_than(0_i64)
    .order_by_memory_rss_kb_desc()
    .limit(50)
    .execute_for_list(&ctx)
    .await?;

for p in top_memory_procs {
    println!("PID: {} | Name: {} | RSS: {} KB", p.pid(), p.name(), p.memory_rss_kb());
}
```

### Advanced Filtering with `E::`

For more complex filters, you can use the `E::` (Expression) API to build rich logical conditions using `and`, `or`, and custom predicates:

```rust
use linux_system_info_core::{Q, E};

// Find potentially rogue background processes
let rogue_procs = Q::processes()
    .filter(
        E::process::memory_rss_kb().gt(1024_000_i64) // Over 1GB memory
            .and(E::process::cpu_user_ticks().gt(1000_i64)) // High CPU usage
            .and(E::process::state().eq("S".to_string())) // Currently sleeping
    )
    .execute_for_list(&ctx)
    .await?;
```

These generated APIs guarantee compile-time safety and make navigating system data as easy as querying a local database!
