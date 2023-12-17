use crate::util::common_file;
use itertools::Itertools;


pub fn part1(input: &str) {

    let file_string = common_file::read_file_into_buffer(input);
    let mut init_space: Vec<Vec<char>> = Vec::new();
    for line in file_string {
        init_space.push(line.chars().collect());
    }

    let mut rows: Vec<usize> = Vec::new();

    for (i, row) in init_space.iter().enumerate() {
        if !row.iter().any(|x| *x == '#') {
            rows.push(i);
        }
    }

    let mut cols: Vec<usize> = Vec::new();
    for col_idx in 0..init_space[0].len() {
        let mut found = false;
        for row in init_space.iter() {
            if row[col_idx] == '#' {
                found = true;
            }
        }
        if !found {
            cols.push(col_idx);
        }
    }

    //do cols first
    let mut added_space = 0usize;
    for col in cols {
        for rows in init_space.iter_mut() {
            rows.insert(col + added_space, '.');
        }
        added_space += 1;
    }

    added_space = 0usize;
    let added_space_row: Vec<char> = vec!['.'; init_space[0].len()];
    for row in rows {
        init_space.insert(row + added_space, added_space_row.clone());
        added_space += 1;
    }

    //Get the positions of #'s

    let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();

    let expanded_space = init_space; //just to rename

    for (row_idx, row) in expanded_space.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            match *col {
                '#' => galaxy_positions.push((row_idx, col_idx)),
                _ => (),
            }
        }
    }

    let mut total = 0usize;
    let pairs: Vec<(&(usize, usize), &(usize, usize))> = galaxy_positions.iter().tuple_combinations().collect_vec();

    for pair in pairs {
        let galaxy1_pos = pair.0;
        let galaxy2_pos = pair.1;
        let dx = (galaxy2_pos.0 as i32 - galaxy1_pos.0 as i32).abs() as usize;
        let dy = (galaxy2_pos.1 as i32 - galaxy1_pos.1 as i32).abs() as usize;

        let smallest_dist = dx + dy;
        total += smallest_dist;
    }
    
    println!("Sum of smallest distances is {total}");
}

pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let space_expansion = 1_000_000usize;
    let mut init_space: Vec<Vec<char>> = Vec::new();
    for line in file_string {
        init_space.push(line.chars().collect());
    }

    let mut rows: Vec<usize> = vec![1; init_space.len()];
    let mut cols: Vec<usize> = vec![1; init_space[0].len()];


    for (i, row) in init_space.iter().enumerate() {
        if !row.iter().any(|x| *x == '#') {
            rows[i] = space_expansion;
        }
    }

    for col_idx in 0..init_space[0].len() {
        let mut found = false;
        for row in init_space.iter() {
            if row[col_idx] == '#' {
                found = true;
            }
        }
        if !found {
            cols[col_idx] = space_expansion;
        }
    }


     //Get the positions of #'s

     let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();
 
     for (row_idx, row) in init_space.iter().enumerate() {
         for (col_idx, col) in row.iter().enumerate() {
             match *col {
                 '#' => galaxy_positions.push((col_idx, row_idx)),
                 _ => (),
             }
         }
     }

    let mut total = 0usize;
    let pairs: Vec<(&(usize, usize), &(usize, usize))> = galaxy_positions.iter().tuple_combinations().collect_vec();

    for pair in pairs {
        let galaxy1_pos = pair.0;
        let galaxy2_pos = pair.1;

        let min_x = galaxy1_pos.0.min(galaxy2_pos.0);
        let max_x = galaxy1_pos.0.max(galaxy2_pos.0);
        let min_y = galaxy1_pos.1.min(galaxy2_pos.1);
        let max_y = galaxy1_pos.1.max(galaxy2_pos.1);

        let mut smallest_distance = 0usize;

        for x in min_x+1..max_x+1 {
            smallest_distance += cols[x];
        }
        for y in min_y+1..max_y+1 {
            smallest_distance += rows[y];
        }
        total += smallest_distance;
    }

    println!("Total expanded distance = {total}");
}