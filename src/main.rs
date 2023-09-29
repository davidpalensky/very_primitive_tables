pub mod vec2d;
use vec2d::csv_to_vec2d_ref;
use very_primitive_tables::Table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = csv_to_vec2d_ref(include_str!("../test.csv"));
    let table = Table::from_vec2d(&table)?;
    print!("table =\n{}", table.render());
    Ok(())
}
