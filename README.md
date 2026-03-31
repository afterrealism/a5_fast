# a5_fast

Fast Python bindings for the [A5 discrete global grid system](https://github.com/afterrealism/a5), powered by Rust and PyO3.

## Installation

```bash
pip install a5_fast
```

## Usage

```python
import a5_fast

# Convert (lon, lat) to a cell index at resolution 5
cell = a5_fast.lonlat_to_cell(13.4, 52.5, 5)

# Get the centre of a cell
lon, lat = a5_fast.cell_to_lonlat(cell)

# Get the boundary polygon of a cell
boundary = a5_fast.cell_to_boundary(cell)

# Batch convert coordinates (flat list: [lon0, lat0, lon1, lat1, ...])
cells = a5_fast.lonlat_to_cell_batch([13.4, 52.5, -73.9, 40.7], 5)

# Traverse the hierarchy
parent = a5_fast.cell_to_parent(cell)
children = a5_fast.cell_to_children(cell)

# Neighbours within k hops
disk = a5_fast.grid_disk(cell, 1)

# Compact / uncompact cell sets
compacted = a5_fast.compact(cells)
expanded = a5_fast.uncompact(compacted, 5)

# Hex conversion
hex_str = a5_fast.u64_to_hex(cell)
cell = a5_fast.hex_to_u64(hex_str)

# Metadata
res = a5_fast.get_resolution(cell)
area = a5_fast.cell_area(5)           # area in m² at resolution 5
n = a5_fast.get_num_cells(5)          # total cells at resolution 5
base = a5_fast.get_res0_cells()       # the 12 resolution-0 cells
```

## License

MIT
