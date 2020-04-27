pub fn raindrops(n: u32) -> String {
    // if n % 105 == 0 {
    //     "PlingPlangPlong".to_string()
    // } else if n % 35 == 0 {
    //     "PlangPlong".to_string()
    // } else if n % 21 == 0 {
    //     "PlingPlong".to_string()
    // } else if n % 15 == 0 {
    //     "PlingPlang".to_string()
    // } else if n % 7 == 0 {
    //     "Plong".to_string()
    // } else if n % 5 == 0 {
    //     "Plang".to_string()
    // } else if n % 3 == 0 {
    //     "Pling".to_string()
    // } else {
    //     n.to_string()
    // }

    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => "PlingPlangPlong".to_string(),
        (_, 0, 0) => "PlangPlong".to_string(),
        (0, _, 0) => "PlingPlong".to_string(),
        (0, 0, _) => "PlingPlang".to_string(),
        (_, _, 0) => "Plong".to_string(),
        (_, 0, _) => "Plang".to_string(),
        (0, _, _) => "Pling".to_string(),
        (_, _, _) => n.to_string(),
    }
}
