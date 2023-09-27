# ORM

**Objective**: Create a PostgreSQL database that preserves state of a stock. Insert a few items. Then select some of them and present the results.

### Python

ORM in use: [sqlalchemy](https://www.sqlalchemy.org/).
Migration tool: [alembic](https://alembic.sqlalchemy.org/).

```python
{{#include dborm_py/dborm/__main__.py}}
```

Restore commented lines to Initialize database.

### Rust

ORM in use: [diesel](https://diesel.rs/). Diesel comes with built-in migration tool.

```rs
// models.rs
{{#include dborm_rs/src/models.rs}}
```

```rs
// schema.rs
{{#include dborm_rs/src/schema.rs}}
```

```rust
// main.rs
{{#include dborm_rs/src/main.rs}}
```

### Crystal

ORM in use: [granite](https://github.com/amberframework/granite/tree/master).
Migration tool: [micrate](https://github.com/amberframework/micrate).
These are part of [amber](https://amberframework.org/) framework.

```crystal
{{#include dborm_cr/src/dborm.cr}}
```

* Initialize DB by calling: `shards run micrate -- up`
* Uninitialize DB by calling: `shards run micrate -- down`
