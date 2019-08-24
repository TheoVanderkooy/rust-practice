use std::convert::{Into};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32, next: Option<Box<ListNode>> ) -> Self {
        ListNode {
            next,
            val
        }
    }

    // create a list from a slice
    #[allow(dead_code)]
    fn from_slice( arr: &[i32] ) -> Option<Box<Self>> {
        if arr.len() == 0 {
            Option::None
        } else {
            Option::Some(Box::new(ListNode::new( arr[0], ListNode::from_slice(&arr[1..] ))))
        }
    }
}

// std::convert::Into
impl Into<Vec<i32>> for ListNode {
    fn into(self) -> Vec<i32> {
        let mut ret = Vec::new();
        ret.push(self.val);

        let mut l = & self.next;
        while let Some(n) = &l {
            ret.push(n.val);
            l = &n.next;
        }

        ret
    }
}


// first recursive approach
#[allow(dead_code)]
pub fn merge_two_lists_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let ( Some(m1), Some(m2) ) = (&l1, &l2) {
        if m1.val < m2.val {
            Option::Some(Box::new(ListNode::new(m1.val, merge_two_lists_rec(l1.unwrap().next, l2)) ) )
        } else {
            Option::Some(Box::new(ListNode::new(m2.val, merge_two_lists_rec(l2.unwrap().next, l1)) ) )
        }
    } else {
        if l1.is_none() {
            l2
        } else {
            l1
        }
    }
}

// second iterative approach. actually worked!
#[allow(dead_code)]
pub fn merge_two_lists_it(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;

    let mut ret = Option::None;
    let mut tail = &mut ret;

    // take min from each
    while let ( Some(n1), Some(n2) ) = (l1, l2) {
        if n1.val < n2.val {
            *tail = Option::Some(Box::new(ListNode::new(n1.val, Option::None)));
            tail = &mut tail.as_mut().unwrap().next;
            l1 = & l1.as_ref().unwrap().next;
        } else {
            *tail = Option::Some(Box::new(ListNode::new(n2.val, Option::None)));
            tail = &mut tail.as_mut().unwrap().next;
            l2 = & l2.as_ref().unwrap().next;
        }
    }

    // take from whatever is left of l1
    while let Some(n1) = l1 {
        *tail = Option::Some(Box::new(ListNode::new(n1.val, Option::None)));
        tail = &mut tail.as_mut().unwrap().next;
        l1 = & l1.as_ref().unwrap().next;
    }

    // take from whatever is left of l2
    while let Some(n2) = l2 {
        *tail = Option::Some(Box::new(ListNode::new(n2.val, Option::None)));
        tail = &mut tail.as_mut().unwrap().next;
        l2 = & l2.as_ref().unwrap().next;
    }

    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_merge_rec() {
        let l1 = Option::Some(Box::new(ListNode::new(1,
            Option::Some(Box::new(ListNode::new(3,
                Option::Some(Box::new(ListNode::new(5, Option::None)))
            )))
        )));

        let l2 = Option::Some(Box::new(ListNode::new(2,
            Option::Some(Box::new(ListNode::new(2,
                Option::Some(Box::new(ListNode::new(5, Option::None)))
            )))
        )));

        let merged: Vec<i32> = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1, 2, 2, 3, 5, 5] );
    }

    #[test]
    fn basic_merge_it() {
        let l1 = Option::Some(Box::new(ListNode::new(1,
            Option::Some(Box::new(ListNode::new(3,
                Option::Some(Box::new(ListNode::new(5, Option::None)))
            )))
        )));

        let l2 = Option::Some(Box::new(ListNode::new(2,
            Option::Some(Box::new(ListNode::new(2,
                Option::Some(Box::new(ListNode::new(5, Option::None)))
            )))
        )));

        let merged: Vec<i32> = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1, 2, 2, 3, 5, 5] );
    }

    #[test]
    fn test_merge_rec() {
        let l1 = ListNode::from_slice( &vec![][..] );
        let l2 = ListNode::from_slice( &vec![][..] );
        let merged = merge_two_lists_rec(l1, l2);
        assert!( merged.is_none() );

        let l1 = ListNode::from_slice( &vec![][..] );
        let l2 = ListNode::from_slice( &vec![1,2,3][..] );
        let mut merged: Vec<i32> = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,2,3] );

        let l1 = ListNode::from_slice( &vec![2,2][..] );
        let l2 = ListNode::from_slice( &vec![][..] );
        merged = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![2,2] );

        let l1 = ListNode::from_slice( &vec![1,1,1][..] );
        let l2 = ListNode::from_slice( &vec![1,1][..] );
        merged = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,1,1,1,1] );

        let l1 = ListNode::from_slice( &vec![1,2,2][..] );
        let l2 = ListNode::from_slice( &vec![3,4,5,8][..] );
        merged = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,2,2,3,4,5,8] );

        let l1 = ListNode::from_slice( &vec![3,4,4][..] );
        let l2 = ListNode::from_slice( &vec![1,3][..] );
        merged = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,3,3,4,4] );

        let l1 = ListNode::from_slice( &vec![1,1,4,5][..] );
        let l2 = ListNode::from_slice( &vec![2,3][..] );
        merged = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,1,2,3,4,5] );

        let l1 = ListNode::from_slice( &vec![3,3,5][..] );
        let l2 = ListNode::from_slice( &vec![1,2,4,7,8,9][..] );
        merged = (*merge_two_lists_rec(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,2,3,3,4,5,7,8,9] );
    }

        #[test]
    fn test_merge_it() {
        let l1 = ListNode::from_slice( &vec![][..] );
        let l2 = ListNode::from_slice( &vec![][..] );
        let merged = merge_two_lists_it(l1, l2);
        assert!( merged.is_none() );

        let l1 = ListNode::from_slice( &vec![][..] );
        let l2 = ListNode::from_slice( &vec![1,2,3][..] );
        let mut merged: Vec<i32> = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,2,3] );

        let l1 = ListNode::from_slice( &vec![2,2][..] );
        let l2 = ListNode::from_slice( &vec![][..] );
        merged = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![2,2] );

        let l1 = ListNode::from_slice( &vec![1,1,1][..] );
        let l2 = ListNode::from_slice( &vec![1,1][..] );
        merged = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,1,1,1,1] );

        let l1 = ListNode::from_slice( &vec![1,2,2][..] );
        let l2 = ListNode::from_slice( &vec![3,4,5,8][..] );
        merged = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,2,2,3,4,5,8] );

        let l1 = ListNode::from_slice( &vec![3,4,4][..] );
        let l2 = ListNode::from_slice( &vec![1,3][..] );
        merged = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,3,3,4,4] );

        let l1 = ListNode::from_slice( &vec![1,1,4,5][..] );
        let l2 = ListNode::from_slice( &vec![2,3][..] );
        merged = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,1,2,3,4,5] );

        let l1 = ListNode::from_slice( &vec![3,3,5][..] );
        let l2 = ListNode::from_slice( &vec![1,2,4,7,8,9][..] );
        merged = (*merge_two_lists_it(l1, l2).unwrap()).into();
        assert_eq!( merged, vec![1,2,3,3,4,5,7,8,9] );
    }
}
