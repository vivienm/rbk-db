CREATE TABLE colors (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    rgb TEXT NOT NULL CHECK (rgb GLOB '[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]'),
    is_trans INTEGER NOT NULL CHECK (is_trans IN (0, 1))
) STRICT;

CREATE TABLE part_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE parts (
    part_num TEXT PRIMARY KEY CHECK (part_num NOT GLOB 'fig-[0-9][0-9][0-9][0-9][0-9][0-9]'),
    name TEXT NOT NULL,
    part_cat_id INTEGER NOT NULL REFERENCES part_categories(id),
    part_material TEXT NOT NULL CHECK (part_material IN ('cardboard/paper', 'cloth', 'flexible plastic', 'foam', 'metal', 'plastic', 'rubber'))
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
    color_id INTEGER NOT NULL REFERENCES colors(id),
    design_id INTEGER
) STRICT;

CREATE TABLE minifigs (
    fig_num TEXT PRIMARY KEY CHECK (fig_num GLOB 'fig-[0-9][0-9][0-9][0-9][0-9][0-9]'),
    name TEXT NOT NULL,
    num_parts INTEGER NOT NULL CHECK (num_parts >= 0),
    img_url TEXT NOT NULL
) STRICT;

CREATE TABLE themes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id INTEGER REFERENCES themes(id)
) STRICT;

CREATE TABLE sets (
    set_num TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    year INTEGER NOT NULL CHECK (year >= 1932),
    theme_id INTEGER NOT NULL REFERENCES themes(id),
    num_parts INTEGER NOT NULL CHECK (num_parts >= 0),
    img_url TEXT NOT NULL
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
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    is_spare INTEGER NOT NULL CHECK (is_spare IN (0, 1)),
    img_url TEXT,
    UNIQUE (inventory_id, part_num, color_id, is_spare)
) STRICT;

CREATE TABLE inventory_minifigs (
    inventory_id INTEGER NOT NULL REFERENCES inventories(id),
    fig_num TEXT NOT NULL REFERENCES minifigs(fig_num),
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    UNIQUE (inventory_id, fig_num)
) STRICT;

CREATE TABLE inventory_sets (
    inventory_id INTEGER NOT NULL REFERENCES inventories(id),
    set_num TEXT NOT NULL REFERENCES sets(set_num),
    quantity INTEGER NOT NULL CHECK (quantity >= 1),
    UNIQUE (inventory_id, set_num)
) STRICT;
