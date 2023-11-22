mod commands;

use group_files_lib::builders::standart_builder::SortingFielsBuilder;
use commands::sort::SortArgs;

fn main() {
    let args = SortArgs::new();

    let app = match SortingFielsBuilder::new()
    .set_input(args.input)
    .set_output(args.output)
    .set_force(args.force)
    .set_show_hidden(args.show_hidden)
    .build() {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if let Err(e) = app.sort() {
        println!("{}", e)
    }
}