PostgreSQL update table checker
===

build!
---

```cmd
cargo build --release
```

run!
---

```cmd
SET PGHOST=localhost
SET PGPORT=5432
SET PGUSER=user
SET PGDATABASE=test_database
SET EXCLUDE_TABLE='table1','table2'

target\release\postgres-update-table-check.exe
```
