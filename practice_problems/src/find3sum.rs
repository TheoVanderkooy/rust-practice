
use crate::find2sum::search_2sum;


#[allow(dead_code)]
pub fn find_3sum(a: &Vec<i32>, sum: i32) -> Option<(usize, usize, usize)> {

    let mut arr = a.iter()
            .enumerate() // produces pairs (index, value)
            .map( |(i, &val)| (i, val) ) // copy the values
            .collect::<Vec<(usize, i32)>>(); // copy into new vector
    arr.sort_by( |x, y| x.1.cmp(&y.1) );

    for idx in 1..arr.len() {
        if let Some( (j, k) ) = search_2sum( &arr[idx..], sum - arr[idx-1].1 ) {
            let i = arr[idx-1].0;
            let ret = if i < j { (i, j, k) } else if i < k { (j, i, k) } else { (j, k, i) };
            return Some( ret );
        }
    }

    Option::None
}


#[cfg(test)]
mod tests {
    use super::*;

    fn check_3sum(arr: Vec<i32>, sum: i32, should_find: bool) {
        let res = find_3sum(&arr, sum);
        if should_find {
            assert!( res.is_some() );
            let (i, j, k) = res.unwrap();
            assert!( i < j );
            assert!( j < k );
            assert!( k < arr.len() );
            assert_eq!( arr[i] + arr[j] + arr[k], sum );
        } else {
            assert!( res.is_none() );
        }
    }


    #[test]
    fn test_3sum() {
        check_3sum( vec![], 0, false );
        check_3sum( vec![1], 1, false );
        check_3sum( vec![1, 2], 3, false );
        check_3sum( vec![1, 2], 2, false );
        check_3sum( vec![1, 2, 3], 3, false );
        check_3sum( vec![1, 2, 3], 4, false );
        check_3sum( vec![1, 2, 3], 6, true );
        check_3sum( vec![1, 1, 0, -3, 5, 10, 3, -2], 0, true );
        check_3sum( vec![1, 1, 0, -3, 5, 10, 3, -2], 15, true );
        check_3sum( vec![1, 1, 0, -3, 5, 10, 3, -2], 1, true );
        check_3sum( vec![1, 1, 0, -3, 5, 10, 3, -2], -3, false );
        check_3sum( vec![1, 1, 0, -3, 5, 10, 3, -2], 11, true );
        check_3sum( vec![1, 1, 0, -3, 5, 10, 3, -2], 2, true );
        check_3sum( vec![1, 1, 0, 6, 10, 3, 11], 2, true );
        check_3sum( vec![1, 1, 0, 6, 10, 3, 11], 1, false );
        check_3sum( vec![1, 1, 0, 6, 10, 3, 11], 32, false );
        check_3sum( vec![1, 1, 0, 6, 10, 3, 11], 33, false );
    }
}
