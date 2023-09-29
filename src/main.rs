mod vec2d;
use table_test::Table;
use vec2d::*;

fn main() {
    let table = csv_to_vec2d_ref(include_str!("../test.csv"));
    let table = Table::from_vec2d(&table).expect("Column numbers match.");
    print!("table =\n{}", table.render());
}
