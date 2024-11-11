# rbk-db

Import the Rebrickable LEGO database into SQLite.

```shell
rbk-db rebrickable.db
sqlite3 rebrickable.db
```

## General organization

![Database schema](https://rebrickable.com/static/img/diagrams/downloads_schema_v3.png)

An *inventory* represents a specific version of either:
* A LEGO set, which includes:
  - A list of subsets (table `inventory_sets`);
  - A list of parts, excluding parts of subsets and minifigures (table `inventory_parts`);
  - A list of minifigures (table `inventory_minifigs`);
* A minifigure, which includes:
  - A list of parts (table `inventory_parts`).

Therefore, `inventories.set_num` serves as a foreign key to either `sets.set_num` or `minifigs.fig_num`, depending on the inventory type.
In the case of minifigures, `inventories.set_num` is prefixed with `fig-`.
