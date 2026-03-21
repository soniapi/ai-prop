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


pub fn calculate_proportions (
    m: &f32, m1: &f32, m2: &f32, n: &f32, n1: &f32, n2: &f32,
) -> (f32, f32, f32) {
    println!("Numbers m {:?}, m1: {:?}, m2: {:?}, n: {:?}, n1: {:?}, n2: {:?}", m, m1, m2, n, n1, n2);
    let p1: f32 = m1 / n1;
    let p2: f32 = m2 / n2;
    let p_population: f32 = m / n;
    println!("Proportion for whole population {:?}, population 1 {:?}, population 2 {:?}", p_population, p1, p2);
    (p_population, p1, p2)
}

pub fn calculate_pooled_estimate (n1: &f32, n2: &f32, p1: &f32, p2: &f32) -> f32 {
    let p = ((n1 * p1) + (n2 * p2)) / (n1 + n2);
    println!("Pooled estimate: {:?}", p);
    p
}

pub fn calculate_z_statistics (n1: &f32, n2: &f32, p1: &f32, p2: &f32, pooled_estimate: &f32) -> f32 {
    let to_be_sqrt = (pooled_estimate * (1.0 - pooled_estimate))*((1.0 / n1) +( 1.0 / n2));
    let z = (p1 - p2) / to_be_sqrt.sqrt();
    println!("Z statistics: {:?}", z);
    z
}