use std::fs;
use std::iter;

const DAY: i32 = 8;

fn file_path() -> String {
    format!("src/input{DAY}.txt")
}

pub fn part_a() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let grid: Vec<Vec<u8>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut seen: Vec<Vec<bool>> = grid
        .iter()
        .map(|r| iter::repeat(false).take(r.len()).collect())
        .collect();

    let width = seen[0].len();
    let height = seen.len();

    // Left
    for (row_idx, row) in grid.iter().enumerate() {
        let mut highest: u8 = 0;
        let mut first = true;
        for (col_idx, tree) in row.iter().enumerate() {
            if *tree > highest || first {
                first = false;
                seen[row_idx][col_idx] = true;
                highest = *tree;
            }
        }
    }

    // Right
    for (row_idx, row) in grid.iter().enumerate() {
        let mut highest: u8 = 0;
        let mut first = true;
        for (col_idx, tree) in row.iter().enumerate().rev() {
            if *tree > highest || first {
                first = false;
                seen[row_idx][col_idx] = true;
                highest = *tree;
            }
        }
    }
    // Top
    for col_idx in 0..width {
        let mut highest: u8 = 0;
        let mut first = true;
        for row_idx in 0..height {
            let tree = grid[row_idx][col_idx];
            if tree > highest || first {
                first = false;
                seen[row_idx][col_idx] = true;
                highest = tree;
            }
        }
    }
    // Bottom
    for col_idx in 0..width {
        let mut highest: u8 = 0;
        let mut first = true;
        for row_idx in (0..height).rev() {
            let tree = grid[row_idx][col_idx];
            if tree > highest || first {
                first = false;
                seen[row_idx][col_idx] = true;
                highest = tree;
            }
        }
    }

    let results: u32 = seen
        .iter()
        .map(|r| r.iter().map(|t| *t as u32).sum::<u32>())
        .sum();

    println!("{grid:?}");
    println!("{seen:?}");

    // Solution 1820
    println!("Day {DAY}a best:\n{results}");
}

pub fn score(grid: &Vec<Vec<u8>>, t_r: usize, t_c: usize) -> u32 {
    let mut result = 1;

    let height = grid[t_r][t_c];

    // Left
    let mut count: u32 = 0;
    for col_idx in (0..t_c).rev() {
        let n = grid[t_r][col_idx];
        count += 1;
        if n >= height {
            // height = n;
            break;
        }
    }
    result *= count;

    // Right
    let mut count: u32 = 0;
    for col_idx in (t_c + 1)..grid[t_r].len() {
        let n = grid[t_r][col_idx];
        count += 1;
        if n >= height {
            // height = n;
            break;
        }
    }
    result *= count;

    // Up
    let mut count: u32 = 0;
    for row_idx in (0..t_r).rev() {
        let n = grid[row_idx][t_c];
        count += 1;
        if n >= height {
            // height = n;
            break;
        }
    }
    result *= count;

    // Down
    let mut count: u32 = 0;
    for row_idx in (t_r + 1)..grid.len() {
        let n = grid[row_idx][t_c];
        count += 1;
        if n >= height {
            // height = n;
            break;
        }
    }
    result *= count;
    result
}

pub fn part_b() {
    let contents =
        fs::read_to_string(file_path()).expect("Should have been able to read {DAY} the file");

    let grid: Vec<Vec<u8>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let eg: Vec<Vec<u32>> = grid
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, _)| score(&grid, row_idx, col_idx))
                .collect()
        })
        .collect();
    for l in eg {
        println!("{l:?}");
    }
    // let eg: Vec<Vec<u32>> = grid.iter().enumerate().map(|(row_idx, row)| {
    //     row.iter()
    //         .enumerate()
    //         .map(|(col_idx, _)| score(&grid, row_idx, col_idx))
    // });

    let results: u32 = grid
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, _)| score(&grid, row_idx, col_idx))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    // Solution 385112
    println!("Day {DAY}b best:\n{results}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_part_a() {
        part_a();
    }

    #[test]
    fn do_part_b() {
        part_b();
    }
}
