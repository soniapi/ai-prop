use ai_infra::{establish_connection, fill_partitions};

fn main() {
    let connection = &mut establish_connection();
    ai_prop::divider(connection, 100000.00);
    fill_partitions();
}
