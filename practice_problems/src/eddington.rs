
/// # eddington
/// takes a list sorted in descending order, and returns a the maximum number E such
/// that at least E items in the list are >= E
///
/// ## Examples
///
/// ```
/// use practice_problems::eddington::eddington;
/// assert_eq!( eddington(vec![5,4,3,2,1]), 3);
/// ```
pub fn eddington( nums: Vec<u32> ) -> u32 {
    if nums.len() == 0 { 0 } else {
        eddington_rec( &nums[..], 0 )
    }
}

fn eddington_rec( nums: &[u32], offset: usize ) -> u32 {
    if nums.len() <= 1 {
        return if offset == 0 && nums[0] < 1 {
            0
        } else {
            offset as u32 + 1
        };
    }

    // index in the slise
    let m: usize = nums.len()/2 + nums.len()%2; // ceil
    let moffset = m + offset; // new offset

    if nums[m] > moffset as u32 {
        eddington_rec( &nums[m..], moffset )
    } else {
        eddington_rec( &nums[..m], offset )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eddington() {

        assert_eq!( eddington( vec![5,4,3,2,1] ), 3 );
        assert_eq!( eddington( vec![5,4,3,2] ), 3 );
        assert_eq!( eddington( vec![4,3,2,1] ), 2 );
        assert_eq!( eddington( vec![1,1,1] ), 1 );
        assert_eq!( eddington( vec![0,0,0,0] ), 0 );
        assert_eq!( eddington( vec![11,7,5,3,2,1] ), 3 );
        assert_eq!( eddington( vec![10,9,8] ), 3 );
        assert_eq!( eddington( vec![] ), 0 );
    }
}
