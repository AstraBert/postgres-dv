# postgres-dv

Rich Postgres data viewer CLI.

## Overview

**postgres-dv** is a simple yet powerful command-line tool for visualizing data from remote and local Postgres databases. It provides a rich, interactive console interface for running and viewing the results of `SELECT` queries.

### Features
- Connect to any Postgres database using a connection string
- Only allows safe `SELECT` queries (no data modification)
- Pretty-prints query results in a table format
- Interactive console with commands to clear or exit
- Secure connection string input via stdin

## Installation

Install with npm:

```bash
npm install @cle-does-things/postgres-dv
```

Install with cargo:

```bash
cargo install postgres-dv
```

Build from source:

```bash
git clone https://github.com/AstraBert/postgres-dv
cd postgres-dv
cargo build --release
```

The binary will be located at `target/release/postgres-dv`.

## Usage

You can run `postgres-dv` with either a connection string argument or by securely entering it via stdin.

### Command-line Options

```
USAGE:
    postgres-dv [OPTIONS]

OPTIONS:
    -u, --uri <URI>            Postgres database URI (not recommended for sensitive credentials)
    -c, --connections <N>      Number of connections in the pool [default: 1]
    -s, --stdin                Read the connection string from stdin (recommended)
    -h, --help                 Print help information
    -V, --version              Print version information
```

### Examples

**Recommended (secure):**

```sh
postgres-dv --stdin
```
You will be prompted to enter your Postgres connection string securely.

**Or, with URI (less secure):**

```sh
postgres-dv --uri "postgresql://user:password@host:5432/database"
```

### Interactive Console
- Type a valid `SELECT` query ending with a semicolon to view results.
- Type `c` or `clear` to clear the console.
- Type `q`, `quit`, `e`, or `exit` to leave the console.
- If you use `SELECT *`, you will be prompted for confirmation.

## Query Restrictions
- Only `SELECT` queries are allowed (no `INSERT`, `UPDATE`, `DELETE`, etc.).
- Queries must match the pattern: `SELECT ... FROM ...;`

## License

See [LICENSE](LICENSE).