use diesel::{PgConnection, sql_query, RunQueryDsl};

pub fn divider(
  connection: &mut PgConnection,
  divider_value: f32,
) {
    let partitioned_table = "objects_s";
    let below = "_below_";
    let above = "_above_";
    let partition_name_below = format!("{}{}{}", partitioned_table, below, divider_value);
    let partition_name_above = format!("{}{}{}", partitioned_table, above, divider_value);

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


pub struct PopulationData {
    pub m: f32,
    pub n: f32,
}

pub fn calculate_proportions(
    overall: PopulationData,
    group1: PopulationData,
    group2: PopulationData,
) -> (f32, f32, f32) {
    println!(
        "Numbers m {:?}, m1: {:?}, m2: {:?}, n: {:?}, n1: {:?}, n2: {:?}",
        overall.m, group1.m, group2.m, overall.n, group1.n, group2.n
    );
    let p1: f32 = group1.m / group1.n;
    let p2: f32 = group2.m / group2.n;
    let p_population: f32 = overall.m / overall.n;
    println!(
        "Proportion for whole population {:?}, population 1 {:?}, population 2 {:?}",
        p_population, p1, p2
    );
    (p_population, p1, p2)
}

pub fn calculate_pooled_estimate(n1: f32, n2: f32, p1: f32, p2: f32) -> f32 {
    let p = ((n1 * p1) + (n2 * p2)) / (n1 + n2);
    println!("Pooled estimate: {:?}", p);
    p
}

pub fn calculate_z_statistics(
    n1: f32,
    n2: f32,
    p1: f32,
    p2: f32,
    pooled_estimate: f32,
) -> f32 {
    let to_be_sqrt = (pooled_estimate * (1.0 - pooled_estimate)) * ((1.0 / n1) + (1.0 / n2));
    let z = (p1 - p2) / (to_be_sqrt as f32).sqrt();
    println!("Z statistics: {:?}", z);
    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_proportions_division_by_zero() {
        let (p_pop, p1, p2) = calculate_proportions(10.0, 5.0, 5.0, 0.0, 0.0, 0.0);

        assert!(p_pop.is_infinite());
        assert!(p_pop.is_sign_positive());

        assert!(p1.is_infinite());
        assert!(p1.is_sign_positive());

        assert!(p2.is_infinite());
        assert!(p2.is_sign_positive());
    }
    #[test]
    fn test_calculate_z_statistics_happy_path() {
        let n1 = 100.0;
        let n2 = 100.0;
        let p1 = 0.6;
        let p2 = 0.4;
        let pooled_estimate = 0.5;

        let result = calculate_z_statistics(n1, n2, p1, p2, pooled_estimate);

        assert!((result - 2.828427).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_z_statistics_zero_difference() {
        let n1 = 50.0;
        let n2 = 50.0;
        let p1 = 0.5;
        let p2 = 0.5;
        let pooled_estimate = 0.5;

        let result = calculate_z_statistics(n1, n2, p1, p2, pooled_estimate);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_z_statistics_negative_difference() {
        let n1 = 100.0;
        let n2 = 100.0;
        let p1 = 0.4;
        let p2 = 0.6;
        let pooled_estimate = 0.5;

        let result = calculate_z_statistics(n1, n2, p1, p2, pooled_estimate);

        assert!((result - (-2.828427)).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_z_statistics_different_sample_sizes() {
        let n1 = 200.0;
        let n2 = 100.0;
        let p1 = 0.7;
        let p2 = 0.5;
        let pooled_estimate = 0.6; // Not necessarily mathematically perfect but for test case

        let result = calculate_z_statistics(n1, n2, p1, p2, pooled_estimate);

        // to_be_sqrt = (0.6 * 0.4) * (1/200 + 1/100) = 0.24 * 0.015 = 0.0036
        // sqrt(0.0036) = 0.06
        // z = 0.2 / 0.06 = 3.333333...
        assert!((result - 3.333333).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_pooled_estimate_equal_sample_sizes() {
        let n1 = 100.0;
        let n2 = 100.0;
        let p1 = 0.5;
        let p2 = 0.3;
        let result = calculate_pooled_estimate(n1, n2, p1, p2);
        assert!((result - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_pooled_estimate_unequal_sample_sizes() {
        let n1 = 200.0;
        let n2 = 100.0;
        let p1 = 0.6;
        let p2 = 0.3;
        let result = calculate_pooled_estimate(n1, n2, p1, p2);
        assert!((result - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_pooled_estimate_zero_proportions() {
        let n1 = 50.0;
        let n2 = 50.0;
        let p1 = 0.0;
        let p2 = 0.0;
        let result = calculate_pooled_estimate(n1, n2, p1, p2);
        assert!((result - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_pooled_estimate_one_proportions() {
        let n1 = 50.0;
        let n2 = 50.0;
        let p1 = 1.0;
        let p2 = 1.0;
        let result = calculate_pooled_estimate(n1, n2, p1, p2);
        assert!((result - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calculate_pooled_estimate_zero_samples() {
        let n1 = 0.0;
        let n2 = 0.0;
        let p1 = 0.5;
        let p2 = 0.5;
        let result = calculate_pooled_estimate(n1, n2, p1, p2);
        assert!(result.is_nan());
    }
}