pub mod vec2d;
use vec2d::csv_to_vec2d_ref;
use very_primitive_tables::Table;

fn main() {
    let table = csv_to_vec2d_ref(include_str!("../test.csv"));
    let table = Table::from_vec2d(&table).expect("Column numbers match.");
    print!("table =\n{}", table.render());
}
