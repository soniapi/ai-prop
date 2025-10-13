use infra::{establish_connection, fill_partitions};

fn main() {
    let connection = &mut establish_connection();
    prop::divider(connection, &100.00);
    fill_partitions();
}
