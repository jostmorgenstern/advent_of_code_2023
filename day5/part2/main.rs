use std::io;
use std::ops::Range;
use std::cmp;

fn insert_range(vec: &mut Vec<Range<u64>>, range: &Range<u64>) {
    if range.is_empty() { return; }

    let mut replace_range = range.clone();

    vec.retain (|&Range{start: lo, end: hi}| {
        if range.end < lo || range.start > hi {
            return true
        } else {
            replace_range.start = cmp::min(lo, replace_range.start);
            replace_range.end = cmp::max(hi, replace_range.end);
            return false;
        }
    });

    let insert_idx = vec.binary_search_by_key(&replace_range.start, |&Range{start: lo, end: hi}| hi).unwrap_err();
    vec.insert(insert_idx, replace_range.clone());
}

fn remove_range(vec: &mut Vec<Range<u64>>, range: &Range<u64>) {
    if range.is_empty() { return; }

    let mut left_non_overlap: Option<Range<u64>> = None;
    let mut right_non_overlap: Option<Range<u64>> = None;

    vec.retain_mut (|&mut Range{start: lo, end: hi}| {
        if range.end < lo || range.start > hi {
            // no overlap
            return true
        }
        if range.end < hi {
            right_non_overlap = Some(range.end..hi);
            return false;
        }
        if range.start > lo {
            left_non_overlap = Some(lo..range.start);
            return false;
        }
        // no non-overlap
        return false;
    });
    if let Some(non_overlap) = left_non_overlap {
        let insert_idx = vec.binary_search_by_key(&non_overlap.start, |&Range{start: lo, end: hi}| hi).unwrap_err();
        vec.insert(insert_idx, non_overlap.clone());
    }
    if let Some(non_overlap) = right_non_overlap {
        let insert_idx = vec.binary_search_by_key(&non_overlap.start, |&Range{start: lo, end: hi}| hi).unwrap_err();
        vec.insert(insert_idx, non_overlap.clone());
    }
}

fn get_first_overlapping_range<'a>(range_vec: &'a Vec<Range<u64>>, range: &Range<u64>) -> Option<(usize, &'a Range<u64>)> {
    if range.is_empty() { return None; }

    for (i, cur_range) in range_vec.iter().enumerate() {
        if !(range.end <= cur_range.start || range.start >= cur_range.end) {
            return Some( (i, cur_range) );
        }
    }
    return None;
}


fn parse_mapping(input_line: &str) -> (Range<u64>, Range<u64>,) {
    let mut iter = input_line.split_whitespace().map( | num_str | u64::from_str_radix(num_str, 10).unwrap() );
    let dst_range_start = iter.next().unwrap();
    let src_range_start = iter.next().unwrap();
    let range_len = iter.next().unwrap();
    return (dst_range_start..dst_range_start+range_len, src_range_start..src_range_start+range_len);
}

fn parse_seeds(seeds_line: &str) -> Vec<Range<u64>> {
    let str_split = seeds_line.split_whitespace().skip(1).map( | num_str | u64::from_str_radix(num_str, 10).unwrap() );
    let mut seeds = vec![];
    let mut range_start: Option<u64> = None;
    for num in str_split {
        match range_start {
            None => { range_start = Some(num); },
            Some(cur_range_start) => {
                insert_range(&mut seeds, &(cur_range_start..cur_range_start+num));
                range_start = None;
            }
        }
    }
    return seeds;
}

fn main() {
    let mut vec = vec![];
    //insert into empty vec
    insert_range(&mut vec, &(100..200));
    assert_eq!(vec, vec![100u64..200]);
    // insert on the left, no overlap
    insert_range(&mut vec, &(0..50));
    assert_eq!(vec, vec![0u64..50, 100..200]);
    // insert on the right, no overlap
    insert_range(&mut vec, &(250..300));
    assert_eq!(vec, vec![0u64..50, 100..200, 250..300]);
    // insert with two overlaps
    insert_range(&mut vec, &(25..115));
    assert_eq!(vec, vec![0u64..200, 250..300]);
    // insert on the right withoverlap
    insert_range(&mut vec, &(280..320));
    assert_eq!(vec, vec![0u64..200, 250..320]);

    // remove with two non-overlaps
    remove_range(&mut vec, &(100..280));
    assert_eq!(vec, vec![0u64..100, 280..320]);
    // remove with no overlap
    remove_range(&mut vec, &(321..322));
    assert_eq!(vec, vec![0u64..100, 280..320]);
    // remove with right non-overlap
    remove_range(&mut vec, &(0..25));
    assert_eq!(vec, vec![25u64..100, 280..320]);

    let vec = vec![52u64..95];
    assert_eq!(get_first_overlapping_range(&vec, &(15..52)), None);


    let stdin = io::stdin();
    let mut lines_iter = stdin.lines();

    let seeds_line = lines_iter.next().unwrap().unwrap();
    let mut seeds_in_progress = parse_seeds(&seeds_line);
    let mut transformed_seeds = vec![];

    lines_iter.next();
    lines_iter.next();

    for line in lines_iter {
        let line_str = line.unwrap();
        if line_str.len() == 0 {
            continue;
        }
        if !line_str.starts_with( |c: char| c.is_digit(10) ) {
            for range in &transformed_seeds {
                insert_range(&mut seeds_in_progress, &range);
            }
            transformed_seeds.clear();
            continue;
        }
        let (dst_range, src_range) = parse_mapping(&line_str);
        while let Some( (i, &Range{start: lo, end: hi}) ) = get_first_overlapping_range(&seeds_in_progress, &src_range) {
            seeds_in_progress.remove(i);
            let mut right_non_overlap: Option<Range<u64>> = None;
            let mut left_non_overlap: Option<Range<u64>> = None;
            if src_range.end < hi {
                right_non_overlap = Some(src_range.end..hi);
            }
            if lo < src_range.start {
                left_non_overlap = Some(lo..src_range.start);
            }
            let mut overlap = cmp::max(lo, src_range.start)..cmp::min(hi, src_range.end);
            overlap.start = overlap.start + dst_range.start - src_range.start;
            overlap.end = overlap.end + dst_range.start - src_range.start;
            insert_range(&mut transformed_seeds, &overlap);
            if let Some(non_overlap) = left_non_overlap {
                insert_range(&mut seeds_in_progress, &non_overlap);
            }
            if let Some(non_overlap) = right_non_overlap {
                insert_range(&mut seeds_in_progress, &non_overlap);
            }
        }
    }

    for range in &seeds_in_progress {
        insert_range(&mut transformed_seeds, &range);
    }
    println!("{:?}", transformed_seeds.iter().min_by( | x, y | x.start.cmp(&y.start) ).unwrap().start);


}


