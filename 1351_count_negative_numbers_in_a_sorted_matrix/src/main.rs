impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut n: usize = {
            let Some(row) = grid.get(0) else { return 0; };
            row.len()
        };
        let mut count: i32 = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().take(n).enumerate() {
                if *cell < 0 {
                    count += ((n - j) * (grid.len() - i)) as i32;
                    n = j;
                    break;
                }
            }
        }
            
        count
    }
}