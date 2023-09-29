# very_primitive_tables
This library is basically for pretty-printing two dimensional vectors of strings.
I created it because I had nothing to do, so issues probably won't be fixed.

It also has some basic csv loading ability.
It does nothing else.
It has no dependencies.
It is slightly annoying to use.

## Example code
main.rs:
```rust
use vec2d::csv_to_vec2d_ref;
use very_primitive_tables::Table;

fn main() {
    let table = csv_to_vec2d_ref(include_str!("test.csv"));
    let table = Table::from_vec2d(&table).expect("All rows have the same number of columns.");
    print!("{}", table.render());
}
```

test.csv:
```csv
x,y,x_error
0.06,0.60,0.0024
0.11,1.20,0.0033
0.17,1.80,0.0083
0.23,2.40,0.0048
0.31,3.00,0.0056
0.38,3.60,0.0124
```

produces:
```
+------+------+---------+
| x    | y    | x_error |
+------+------+---------+
| 0.06 | 0.60 | 0.0024  |
+------+------+---------+
| 0.11 | 1.20 | 0.0033  |
+------+------+---------+
| 0.17 | 1.80 | 0.0083  |
+------+------+---------+
| 0.23 | 2.40 | 0.0048  |
+------+------+---------+
| 0.31 | 3.00 | 0.0056  |
+------+------+---------+
| 0.38 | 3.60 | 0.0124  |
+------+------+---------+
```
