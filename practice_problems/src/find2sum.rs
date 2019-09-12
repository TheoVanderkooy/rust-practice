
/// Searches a sorted array for 2 elements that add to a specific value.
///
/// arr: must be sorted (by the values, the second element), where the first element in
///      each pair is the index within the original array
pub fn search_2sum(arr: &[(usize, i32)], sum: i32) -> Option<(usize, usize)> {
    if arr.len() == 0 {
        return None
    }
    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        let s = arr[i].1 + arr[j].1;
        if s == sum {
            let (r1, r2) = (arr[i].0, arr[j].0);
            return Option::Some( if r1 < r2 { (r1, r2) } else { (r2, r1) } );
        } else if s < sum {
            i += 1;
        } else {
            j -= 1;
        }
    }

    Option::None
}

#[allow(dead_code)]
pub fn find_2sum(a: &Vec<i32>, sum: i32) -> Option<(usize, usize)> {

    let mut arr = a.iter()
            .enumerate() // produces pairs (index, value)
            .map( |(i, &val)| (i, val) ) // copy the values
            .collect::<Vec<(usize, i32)>>(); // copy into new vector
    arr.sort_by( |x, y| x.1.cmp(&y.1) );

    search_2sum(&arr[..], sum)
}



#[cfg(test)]
mod tests {
    use super::*;

    fn check_2sum(arr: Vec<i32>, sum: i32, should_find: bool) {
        let res = find_2sum(&arr, sum);
        if should_find {
            assert!( res.is_some() );
            let (i, j) = res.unwrap();
            assert!( i < j );
            assert!( j < arr.len() );
            assert_eq!( arr[i] + arr[j], sum );
        } else {
            assert!( res.is_none() );
        }
    }

    // TODO test for search separately

    #[test]
    fn test_2sum() {
        check_2sum( vec![1, 1], 2, true );
        check_2sum( vec![1, 2], 3, true );
        check_2sum( vec![1, 2], 0, false );
        check_2sum( vec![1, 2], 1, false );
        check_2sum( vec![1, 2], 2, false );
        check_2sum( vec![1], 1, false );
        check_2sum( vec![], 0, false );
        check_2sum( vec![1, 1], 2, true );
        check_2sum( vec![1, 1, -1, -2, 3, 5], 0, true );
        check_2sum( vec![1, 1, -1, -2, 3, 5], 1, true );
        check_2sum( vec![1, 1, -1, -2, 3, 5], 6, true );
        check_2sum( vec![1, 1, -1, -2, 3, 5], 2, true );
        check_2sum( vec![1, 1, -1, -2, 3, 5], 7, false );
        check_2sum( vec![1, 1, -1, -2, 3, 5], -4, false );
        check_2sum( vec![1, 1, -1, -2, 3, 5], 10, false );
    }
}
