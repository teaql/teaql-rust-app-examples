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
