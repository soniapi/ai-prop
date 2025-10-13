use diesel::{PgConnection, sql_query, RunQueryDsl};

pub fn divider(
  connection: &mut PgConnection,
  divider_value: &f32, 
) {
    let partitioned_table = "objects_s";
    let below = "_below_";
    let above = "_above_";
    let partition_name_below = format!("{}{}{}", partitioned_table, below, divider_value.to_string());
    let partition_name_above = format!("{}{}{}", partitioned_table, above, divider_value.to_string());

    println!("Partition names: {:?} and {:?}", partition_name_below, partition_name_above);

    let sql = format!(
          "CREATE TABLE {} PARTITION OF objects_s FOR VALUES FROM (MINVALUE) TO ('{}')",
        partition_name_below,
        divider_value,
    );
    sql_query(sql)
        .execute(connection)
        .expect("Partition can't be created");

    let sql = format!(
          "CREATE TABLE {} PARTITION OF objects_s FOR VALUES FROM ('{}') TO (MAXVALUE)",
        partition_name_above,
        divider_value,
    );
    sql_query(sql)
        .execute(connection)
        .expect("Partition can't be created");
    
}