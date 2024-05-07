pub struct Area {
    pub fields: Vec<Vec<i32>>,  // Each field is a pair of strings
    pub value: i32,
    pub combinations: Vec<Vec<i32>>,
}

impl Area {
    pub fn combinations_all_fileds(&self) {
        // let N: usize = self.fields.len();
        // let s: i32 = self.value;
        // let combin_area: Vec<Vec<i32>>;
        // let upper_bound: f32 = s-(N-1)*(10-0.5*N);
        // for i in self.fields.iter().enumerate() {
        //     self.fields[i];
        // }
    }

    pub fn print_coords(&self) {
        for (i, row) in self.fields.iter().enumerate() {
            // Iterating over each integer in the sub-vector
            for (j, value) in row.iter().enumerate() {
                println!("Element at [{}][{}] is {}", i, j, value);
            }
        }
    }
}

fn remove_duplicates(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Filter the outer vector, retaining only those inner vectors without duplicates
    a.into_iter()
    .filter(|inner_vec| {
        // Create a HashSet to track seen elements
        let mut seen = std::collections::HashSet::new();
        // Check each element in the inner vector, ensuring it's not a duplicate
        inner_vec.iter().all(|&item| seen.insert(item))
    })
    .collect()
}
    
pub fn combinations(n: f32, s: f32) -> Vec<Vec<i32>> {
    let mut combin_area = Vec::new();
    
    if n!=0.0 {
        let lower_bound_raw: f32 = s-(n-1.0)*(10.0-0.5*n);
        let upper_bound_raw: f32 = s-0.5*n*(n-1.0);
        
        let lower_bound = lower_bound_raw.max(1.0).ceil() as i32;
        let upper_bound = upper_bound_raw.max(0.0).min(9.0).floor() as i32;
        
        if lower_bound <= upper_bound {
            for i in lower_bound..=upper_bound {
                let mut combin_part = combinations(n - 1.0, s - i as f32);
                
                if n==1.0 {
                    let combin = vec![i];
                    combin_part.push(combin);
                }
                else if combin_part.is_empty() {
                    continue;
                }
                else {
                    for inner_vec in combin_part.iter_mut() {
                        inner_vec.push(i);
                    }
                }
                
                combin_area.append(&mut combin_part);
            }
        }
        remove_duplicates(combin_area)
    }
    else {
        
        combin_area
    }
}

