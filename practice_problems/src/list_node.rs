
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn cons(val: i32, tail: Option<Box<Self>>) -> Self {
        ListNode {
            next: tail,
            val,
        }
    }

    // create a list from a slice
    #[allow(dead_code)]
    pub fn from_slice( arr: &[i32] ) -> Option<Box<Self>> {
        if arr.len() == 0 {
            Option::None
        } else {
            Option::Some(Box::new(ListNode::cons( arr[0], ListNode::from_slice(&arr[1..] ))))
        }
    }

    // create a list from a vector
    #[allow(dead_code)]
    pub fn from_vec( arr: Vec<i32> ) -> Option<Box<Self>> {
        Self::from_slice( &arr[..] )
    }
}

#[allow(dead_code)]
pub fn list_to_vec( l: Option<Box<ListNode>> ) -> Vec<i32> {
    let mut ret = Vec::new();
    let mut l = &l;

    while let Some(n) = &l {
        ret.push(n.val);
        l = &n.next;
    }

    ret
}
