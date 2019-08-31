
use crate::list_node::*;
use crate::merge_lists::merge_two_lists_inplace;

#[allow(dead_code)]
fn count_list(head: &Option<Box<ListNode>>) -> u32 {
    if let Some(n) = head.as_ref() {
        1 + count_list(&n.next)
    } else {
        0
    }
}

fn split_list(list: Option<Box<ListNode>>, n: u32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut list = list;
    if n <= 1 {
        return (Option::None, list);
    }

    let mut l2 = &mut list;
    for _ in 0..(n/2) {
        l2 = &mut l2.as_mut().unwrap().next;
    }
    let l2 = l2.take();
    let l1 = list;

    (l1, l2)
}

#[allow(dead_code)]
fn sort_list_rec(list: Option<Box<ListNode>>, n: u32) -> Option<Box<ListNode>> {
    if n <= 1 {
        return list;
    }
    let (l1, l2) =split_list(list, n);
    merge_two_lists_inplace(sort_list_rec(l1, n/2), sort_list_rec(l2, n - (n/2)))
}

#[allow(dead_code)]
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let n = count_list(&head);
    sort_list_rec(head, n)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let mut list = Option::None;
        assert_eq!(0, count_list(&list));

        list = ListNode::from_vec(vec![]);
        assert_eq!(0, count_list(&list));

        list = ListNode::from_vec(vec![1]);
        assert_eq!(1, count_list(&list));

        list = ListNode::from_vec(vec![1,2,3,4]);
        assert_eq!(4, count_list(&list));

        list = ListNode::from_vec(vec![4,76,34,-1,3]);
        assert_eq!(5, count_list(&list));
    }

    #[test]
    fn test_split() {
        let mut list = Option::None;
        let mut expect = (Option::None, Option::None);
        assert_eq!( expect, split_list(list, 0) );

        list = ListNode::from_vec(vec![1]);
        expect =  (Option::None, ListNode::from_vec(vec![1]));
        assert_eq!( expect, split_list(list, 1) );

        list = ListNode::from_vec(vec![1, 2]);
        expect =  (ListNode::from_vec(vec![1]), ListNode::from_vec(vec![2]));
        assert_eq!( expect, split_list(list, 2) );

        list = ListNode::from_vec(vec![1, 2, 3]);
        expect =  (ListNode::from_vec(vec![1]), ListNode::from_vec(vec![2, 3]));
        assert_eq!( expect, split_list(list, 3) );

        list = ListNode::from_vec(vec![1, 2, 3, 4]);
        expect =  (ListNode::from_vec(vec![1, 2]), ListNode::from_vec(vec![3, 4]));
        assert_eq!( expect, split_list(list, 4) );

        list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        expect =  (ListNode::from_vec(vec![1, 2]), ListNode::from_vec(vec![3, 4, 5]));
        assert_eq!( expect, split_list(list, 5) );
    }

    #[allow(dead_code)]
    fn check_sorted(mut input: Vec<i32>) {
        let list = ListNode::from_slice(&input[..]);
        input.sort();
        let expect = ListNode::from_vec(input);
        assert_eq!( expect, sort_list(list) );
    }

    #[test]
    fn test_sort() {
        check_sorted(vec![]);
        check_sorted(vec![1]);
        check_sorted(vec![1,2]);
        check_sorted(vec![2,1]);
        check_sorted(vec![1,2,3,4]);
        check_sorted(vec![4,3,2,1,0]);
        check_sorted(vec![6,4,3,2,6,8,9,2,1]);
        check_sorted(vec![1,1,1]);
    }
}
