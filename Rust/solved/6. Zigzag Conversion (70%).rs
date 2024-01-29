impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
        return s;
    }
    let mut zigzag = vec![vec![]; num_rows as usize];
    let mut idx = 0i32;
    let mut arah = 1;
    for c in s.chars() {
        zigzag[idx as usize].push(c);

        if idx == 0 {
            arah = 1
        }
        if idx == num_rows - 1 {
            arah = -1
        }
        idx = idx + arah;
    }
    zigzag.iter().flatten().collect()
    }
}