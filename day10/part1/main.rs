use std::io;

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
    let row_len = map[0].len();
    let col_len = map.len();

    let mut cur_pos = starting_pos.unwrap();
    let mut last_pos = starting_pos.unwrap();
    let mut loop_len = 0;
    loop {
        let mut next_pos = (0, 0);
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
        loop_len += 1;
        if map[next_pos.0][next_pos.1] == 'S' {
            break;
        }
        last_pos = cur_pos;
        cur_pos = next_pos;
    }

    println!("{}", loop_len / 2);
}
