CREATE TABLE colors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    rgb TEXT NOT NULL,
    is_trans INTEGER NOT NULL
) STRICT;

CREATE TABLE part_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE parts (
    part_num TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    part_cat_id INTEGER NOT NULL REFERENCES part_categories(id)
) STRICT;

-- NOTE: There might be several relationships between the same parts, with
-- different types.
CREATE TABLE part_relationships (
    rel_type TEXT NOT NULL,
    child_part_num TEXT NOT NULL REFERENCES parts(part_num),
    parent_part_num TEXT NOT NULL REFERENCES parts(part_num),
    UNIQUE (rel_type, child_part_num, parent_part_num)
) STRICT;

CREATE TABLE elements (
    element_id TEXT PRIMARY KEY,
    part_num TEXT NOT NULL REFERENCES parts(part_num),
    color_id INTEGER NOT NULL REFERENCES colors(id)
) STRICT;

CREATE TABLE minifigs (
    fig_num TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    num_parts INTEGER NOT NULL,
    img_url TEXT
) STRICT;

CREATE TABLE themes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id INTEGER REFERENCES themes(id)
) STRICT;

CREATE TABLE sets (
    set_num TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    year INTEGER NOT NULL,
    theme_id INTEGER NOT NULL REFERENCES themes(id),
    num_parts INTEGER NOT NULL,
    img_url TEXT
) STRICT;

-- NOTE: `set_num` references either `sets.set_num` or `minifigs.fig_num`. In
-- the latter case, it starts with `fig-`.
CREATE TABLE inventories (
    id INTEGER PRIMARY KEY,
    version INTEGER NOT NULL,
    set_num TEXT NOT NULL
) STRICT;

CREATE TABLE inventory_parts (
    inventory_id INTEGER NOT NULL REFERENCES inventories(id),
    part_num TEXT NOT NULL REFERENCES parts(part_num),
    color_id INTEGER NOT NULL REFERENCES colors(id),
    quantity INTEGER NOT NULL,
    is_spare INTEGER NOT NULL,
    img_url TEXT,
    UNIQUE (inventory_id, part_num, color_id, is_spare)
) STRICT;

CREATE TABLE inventory_minifigs (
    inventory_id INTEGER NOT NULL REFERENCES inventories(id),
    fig_num TEXT NOT NULL REFERENCES minifigs(fig_num),
    quantity INTEGER NOT NULL,
    UNIQUE (inventory_id, fig_num)
) STRICT;

CREATE TABLE inventory_sets (
    inventory_id INTEGER NOT NULL REFERENCES inventories(id),
    set_num TEXT NOT NULL REFERENCES sets(set_num),
    quantity INTEGER NOT NULL,
    UNIQUE (inventory_id, set_num)
) STRICT;
