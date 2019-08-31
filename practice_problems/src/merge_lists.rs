use crate::list_node::*;

// first recursive approach
#[allow(dead_code)]
pub fn merge_two_lists_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let ( Some(m1), Some(m2) ) = (&l1, &l2) {
        if m1.val < m2.val {
            Option::Some(Box::new(ListNode::cons(m1.val, merge_two_lists_rec(l1.unwrap().next, l2)) ) )
        } else {
            Option::Some(Box::new(ListNode::cons(m2.val, merge_two_lists_rec(l2.unwrap().next, l1)) ) )
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
            *tail = Option::Some(Box::new(ListNode::cons(n1.val, Option::None)));
            tail = &mut tail.as_mut().unwrap().next;
            l1 = & l1.as_ref().unwrap().next;
        } else {
            *tail = Option::Some(Box::new(ListNode::cons(n2.val, Option::None)));
            tail = &mut tail.as_mut().unwrap().next;
            l2 = & l2.as_ref().unwrap().next;
        }
    }

    // take from whatever is left of l1
    while let Some(n1) = l1 {
        *tail = Option::Some(Box::new(ListNode::cons(n1.val, Option::None)));
        tail = &mut tail.as_mut().unwrap().next;
        l1 = & l1.as_ref().unwrap().next;
    }

    // take from whatever is left of l2
    while let Some(n2) = l2 {
        *tail = Option::Some(Box::new(ListNode::cons(n2.val, Option::None)));
        tail = &mut tail.as_mut().unwrap().next;
        l2 = & l2.as_ref().unwrap().next;
    }

    ret
}

// third iterative approach that doesn't make new nodes (reuses existing nodes)
#[allow(dead_code)]
pub fn merge_two_lists_inplace(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut ret = Option::None;
    let mut tail = &mut ret;

    // take min from each
    while let ( Some(n1), Some(n2) ) = (&l1, &l2) {
        if n1.val < n2.val {
            let mut temp = l1.unwrap();
            l1 = temp.next.take();
            *tail = Option::Some(temp);
            tail = &mut tail.as_mut().unwrap().next;
        } else {
            let mut temp = l2.unwrap();
            l2 = temp.next.take();
            *tail = Option::Some(temp);
            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    // take from whatever is left of l1
    while l1.is_some() {
        let mut temp = l1.unwrap();
        l1 = temp.next.take();
        *tail = Option::Some(temp);
        tail = &mut tail.as_mut().unwrap().next;
    }

    // take from whatever is left of l2
    while l2.is_some() {
        let mut temp = l2.unwrap();
        l2 = temp.next.take();
        *tail = Option::Some(temp);
        tail = &mut tail.as_mut().unwrap().next;
    }

    ret
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_merge_rec() {
        let l1 = Option::Some(Box::new(ListNode::cons(1,
            Option::Some(Box::new(ListNode::cons(3,
                Option::Some(Box::new(ListNode::cons(5, Option::None)))
            )))
        )));

        let l2 = Option::Some(Box::new(ListNode::cons(2,
            Option::Some(Box::new(ListNode::cons(2,
                Option::Some(Box::new(ListNode::cons(5, Option::None)))
            )))
        )));

        let merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1, 2, 2, 3, 5, 5] );
    }

    #[test]
    fn basic_merge_it() {
        let l1 = Option::Some(Box::new(ListNode::cons(1,
            Option::Some(Box::new(ListNode::cons(3,
                Option::Some(Box::new(ListNode::cons(5, Option::None)))
            )))
        )));

        let l2 = Option::Some(Box::new(ListNode::cons(2,
            Option::Some(Box::new(ListNode::cons(2,
                Option::Some(Box::new(ListNode::cons(5, Option::None)))
            )))
        )));

        let merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1, 2, 2, 3, 5, 5] );
    }

    #[test]
    fn basic_merge_inplace() {
        let l1 = Option::Some(Box::new(ListNode::cons(1,
            Option::Some(Box::new(ListNode::cons(3,
                Option::Some(Box::new(ListNode::cons(5, Option::None)))
            )))
        )));

        let l2 = Option::Some(Box::new(ListNode::cons(2,
            Option::Some(Box::new(ListNode::cons(2,
                Option::Some(Box::new(ListNode::cons(5, Option::None)))
            )))
        )));

        let merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1, 2, 2, 3, 5, 5] );
    }

    #[test]
    fn test_merge_rec() {
        let l1 = ListNode::from_vec( vec![] );
        let l2 = ListNode::from_vec( vec![] );
        let merged = merge_two_lists_rec(l1, l2);
        assert!( merged.is_none() );

        let l1 = ListNode::from_vec( vec![] );
        let l2 = ListNode::from_vec( vec![1,2,3] );
        let mut merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1,2,3] );

        let l1 = ListNode::from_vec( vec![2,2] );
        let l2 = ListNode::from_vec( vec![] );
        merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![2,2] );

        let l1 = ListNode::from_vec( vec![1,1,1] );
        let l2 = ListNode::from_vec( vec![1,1] );
        merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1,1,1,1,1] );

        let l1 = ListNode::from_vec( vec![1,2,2] );
        let l2 = ListNode::from_vec( vec![3,4,5,8] );
        merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1,2,2,3,4,5,8] );

        let l1 = ListNode::from_vec( vec![3,4,4] );
        let l2 = ListNode::from_vec( vec![1,3] );
        merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1,3,3,4,4] );

        let l1 = ListNode::from_vec( vec![1,1,4,5] );
        let l2 = ListNode::from_vec( vec![2,3] );
        merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1,1,2,3,4,5] );

        let l1 = ListNode::from_vec( vec![3,3,5] );
        let l2 = ListNode::from_vec( vec![1,2,4,7,8,9] );
        merged = list_to_vec(merge_two_lists_rec(l1, l2));
        assert_eq!( merged, vec![1,2,3,3,4,5,7,8,9] );
    }

    #[test]
    fn test_merge_it() {
        let l1 = ListNode::from_vec( vec![] );
        let l2 = ListNode::from_vec( vec![] );
        let merged = merge_two_lists_it(l1, l2);
        assert!( merged.is_none() );

        let l1 = ListNode::from_vec( vec![] );
        let l2 = ListNode::from_vec( vec![1,2,3] );
        let mut merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1,2,3] );

        let l1 = ListNode::from_vec( vec![2,2] );
        let l2 = ListNode::from_vec( vec![] );
        merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![2,2] );

        let l1 = ListNode::from_vec( vec![1,1,1] );
        let l2 = ListNode::from_vec( vec![1,1] );
        merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1,1,1,1,1] );

        let l1 = ListNode::from_vec( vec![1,2,2] );
        let l2 = ListNode::from_vec( vec![3,4,5,8] );
        merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1,2,2,3,4,5,8] );

        let l1 = ListNode::from_vec( vec![3,4,4] );
        let l2 = ListNode::from_vec( vec![1,3] );
        merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1,3,3,4,4] );

        let l1 = ListNode::from_vec( vec![1,1,4,5] );
        let l2 = ListNode::from_vec( vec![2,3] );
        merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1,1,2,3,4,5] );

        let l1 = ListNode::from_vec( vec![3,3,5] );
        let l2 = ListNode::from_vec( vec![1,2,4,7,8,9] );
        merged = list_to_vec(merge_two_lists_it(l1, l2));
        assert_eq!( merged, vec![1,2,3,3,4,5,7,8,9] );
    }

    #[test]
    fn test_merge_inplace() {
        let l1 = ListNode::from_vec( vec![] );
        let l2 = ListNode::from_vec( vec![] );
        let merged = merge_two_lists_inplace(l1, l2);
        assert!( merged.is_none() );

        let l1 = ListNode::from_vec( vec![] );
        let l2 = ListNode::from_vec( vec![1,2,3] );
        let mut merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1,2,3] );

        let l1 = ListNode::from_vec( vec![2,2] );
        let l2 = ListNode::from_vec( vec![] );
        merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![2,2] );

        let l1 = ListNode::from_vec( vec![1,1,1] );
        let l2 = ListNode::from_vec( vec![1,1] );
        merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1,1,1,1,1] );

        let l1 = ListNode::from_vec( vec![1,2,2] );
        let l2 = ListNode::from_vec( vec![3,4,5,8] );
        merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1,2,2,3,4,5,8] );

        let l1 = ListNode::from_vec( vec![3,4,4] );
        let l2 = ListNode::from_vec( vec![1,3] );
        merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1,3,3,4,4] );

        let l1 = ListNode::from_vec( vec![1,1,4,5] );
        let l2 = ListNode::from_vec( vec![2,3] );
        merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1,1,2,3,4,5] );

        let l1 = ListNode::from_vec( vec![3,3,5] );
        let l2 = ListNode::from_vec( vec![1,2,4,7,8,9] );
        merged = list_to_vec(merge_two_lists_inplace(l1, l2));
        assert_eq!( merged, vec![1,2,3,3,4,5,7,8,9] );
    }
}
