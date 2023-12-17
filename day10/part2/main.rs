use std::io;

fn get_zero_border_tile(enlarged_map: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    let row_len = enlarged_map[0].len();
    let col_len = enlarged_map.len();

    // check first row
    for i in 0..row_len {
        if enlarged_map[0][i] == 0 {
            return Some((0, i));
        }
    }
    // check first column
    for i in 0..col_len {
        if enlarged_map[i][0] == 0 {
            return Some((i, 0));
        }
    }
    // check last row
    for i in 0..row_len {
        if enlarged_map[col_len - 1][i] == 0 {
            return Some((col_len - 1, i));
        }
    }
    // check last column
    for i in 0..col_len {
        if enlarged_map[i][row_len - 1] == 0 {
            return Some((i, row_len - 1));
        }
    }

    return None;
}

fn mark_all_reachable_zero_tiles(enlarged_map: &mut Vec<Vec<u8>>, border_tile: (usize, usize)) {
    let row_len = enlarged_map[0].len();
    let col_len = enlarged_map.len();

    let mut queue = vec![border_tile];

    //mark all adjacent zero tiles
    while queue.len() != 0 {
        let (i, j) = queue[0];
        queue.remove(0);
        enlarged_map[i][j] = 2;

        if i > 0 && j > 0 && enlarged_map[i - 1][j - 1] == 0 {
            if !queue.contains(&(i - 1, j - 1)) {
                queue.push((i - 1, j - 1));
            }
        }
        if j > 0 && enlarged_map[i][j - 1] == 0 {
            if !queue.contains(&(i, j - 1)) {
                queue.push((i, j - 1));
            }
        }
        if i < col_len - 1 && j > 0 && enlarged_map[i + 1][j - 1] == 0 {
            if !queue.contains(&(i + 1, j - 1)) {
                queue.push((i + 1, j - 1));
            }
        }
        if i < col_len - 1 && enlarged_map[i + 1][j] == 0 {
            if !queue.contains(&(i + 1, j)) {
                queue.push((i + 1, j));
            }
        }
        if i < col_len - 1 && j < row_len - 1 && enlarged_map[i + 1][j + 1] == 0 {
            if !queue.contains(&(i + 1, j + 1)) {
                queue.push((i + 1, j + 1));
            }
        }
        if j < row_len - 1 && enlarged_map[i][j + 1] == 0 {
            if !queue.contains(&(i, j + 1)) {
                queue.push((i, j + 1));
            }
        }
        if i > 0 && j < row_len - 1 && enlarged_map[i - 1][j + 1] == 0 {
            if !queue.contains(&(i - 1, j + 1)) {
                queue.push((i - 1, j + 1));
            }
        }
        if i > 0 && enlarged_map[i - 1][j] == 0 {
            if !queue.contains(&(i - 1, j)) {
                queue.push((i - 1, j));
            }
        }
    }
}


fn main() {
    let mut map = vec![];
    let stdin = io::stdin();
    let mut starting_pos = None;

    for (row, line) in stdin.lines().enumerate() {
        map.push(line.unwrap().chars().collect::<Vec<char>>());
        if starting_pos == None {
            if let Some(col) = map[row].iter().position(| &pipe | pipe == 'S' ) {
                starting_pos = Some((row, col));
            }
        }
    }
    let mut row_len = map[0].len();
    let mut col_len = map.len();

    let mut cur_pos = starting_pos.unwrap();
    let mut last_pos = starting_pos.unwrap();
    let mut loop_tiles = vec![];
    loop {
        let mut next_pos = (0, 0);
        loop_tiles.push(cur_pos);
        if cur_pos.0 > 0 && last_pos != (cur_pos.0 - 1, cur_pos.1)
            && (map[cur_pos.0 - 1][cur_pos.1] == '7'
                || map[cur_pos.0 - 1][cur_pos.1] == 'F'
                || map[cur_pos.0 - 1][cur_pos.1] == '|'
                || map[cur_pos.0 - 1][cur_pos.1] == 'S')
            && "|JLS".contains(map[cur_pos.0][cur_pos.1]){
            // go up
            next_pos = (cur_pos.0 - 1, cur_pos.1);

        } else if cur_pos.1 < row_len - 1 && last_pos != (cur_pos.0, cur_pos.1 + 1)
            && (map[cur_pos.0][cur_pos.1 + 1] == 'J'
                || map[cur_pos.0][cur_pos.1 + 1] == '7'
                || map[cur_pos.0][cur_pos.1 + 1] == '-'
                || map[cur_pos.0][cur_pos.1 + 1] == 'S')
            && "-FLS".contains(map[cur_pos.0][cur_pos.1]){
            // go right
            next_pos = (cur_pos.0, cur_pos.1 + 1);

        } else if cur_pos.0 < col_len - 1 && last_pos != (cur_pos.0 + 1, cur_pos.1)
            && (map[cur_pos.0 + 1][cur_pos.1] == 'J'
                || map[cur_pos.0 + 1][cur_pos.1] == 'L'
                || map[cur_pos.0 + 1][cur_pos.1] == '|'
                || map[cur_pos.0 + 1][cur_pos.1] == 'S')
            && "7|FS".contains(map[cur_pos.0][cur_pos.1]){
            // go down
            next_pos = (cur_pos.0 + 1, cur_pos.1);

        } else if cur_pos.1 > 0 && last_pos != (cur_pos.0, cur_pos.1 - 1)
            && (map[cur_pos.0][cur_pos.1 - 1] == 'L'
                || map[cur_pos.0][cur_pos.1 - 1] == 'F'
                || map[cur_pos.0][cur_pos.1 - 1] == '-'
                || map[cur_pos.0][cur_pos.1 - 1] == 'S')
            && "J-7S".contains(map[cur_pos.0][cur_pos.1]){
            // go left
            next_pos = (cur_pos.0, cur_pos.1 - 1);
        }
        if map[next_pos.0][next_pos.1] == 'S' {
            break;
        }
        last_pos = cur_pos;
        cur_pos = next_pos;
    }

    let mut enlarged_map = vec![vec![0; row_len * 3]; col_len * 3];
    for &(row, col) in &loop_tiles {
        if map[row][col] == 'F' {
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col + 2] = 1;
            enlarged_map[3 * row + 2][3 * col + 1] = 1;
        } else if map[row][col] == 'J' {
            enlarged_map[3 * row][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col] = 1;
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
        } else if map[row][col] == '7' {
            enlarged_map[3 * row + 1][3 * col] = 1;
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
            enlarged_map[3 * row + 2][3 * col + 1] = 1;
        } else if map[row][col] == 'L' {
            enlarged_map[3 * row][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col + 2] = 1;
        } else if map[row][col] == '|' {
            enlarged_map[3 * row][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
            enlarged_map[3 * row + 2][3 * col + 1] = 1;
        } else if map[row][col] == '-' {
            enlarged_map[3 * row + 1][3 * col] = 1;
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col + 2] = 1;
        } else if map[row][col] == 'S' {
            enlarged_map[3 * row][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col] = 1;
            enlarged_map[3 * row + 1][3 * col + 1] = 1;
            enlarged_map[3 * row + 1][3 * col + 2] = 1;
            enlarged_map[3 * row + 2][3 * col + 1] = 1;
        }
    }


    while let Some((i, j)) = get_zero_border_tile(&enlarged_map) {
        mark_all_reachable_zero_tiles(&mut enlarged_map, (i, j));
    }

    row_len = enlarged_map[0].len();
    col_len = enlarged_map.len();
    let mut num_enclosed_tiles = 0u32;

    for i in (0..col_len).step_by(3) {
        for j in (0..row_len).step_by(3) {
            let mut all_zero = true;
            for i_add in 0..2 {
                for j_add in 0..2 {
                    if enlarged_map[i + i_add][j + j_add] != 0 {
                        all_zero = false;
                    }
                }
            }
            if all_zero {
                num_enclosed_tiles += 1;
            }
        }
    }

    println!("{}", num_enclosed_tiles);

}
