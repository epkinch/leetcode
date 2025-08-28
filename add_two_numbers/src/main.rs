fn main() {
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

  let mut l1 = l1;
  let mut l2 = l2;

  let mut head: Option<Box<ListNode>> = None;
  let mut tail = &mut head;
  let mut carry = 0;
  let mut sum;
  loop {
    match (l1.as_mut(), l2.as_mut()) {
      (Some(_), Some(_)) => {
        let node1 = l1.take().unwrap();
        let node2 = l2.take().unwrap();
        sum = node1.val + node2.val + carry;   
        l1 = node1.next;
        l2 = node2.next;   
      },
      (Some(_), None) => {
        let node1 = l1.take().unwrap();
        sum = node1.val + carry;
        l1 = node1.next;
      }
      (None, Some(_)) => {
        let node2 = l2.take().unwrap();
        sum = node2.val + carry;
        l2 = node2.next;
      }
      (None, None) => {
        if carry == 1 {
          *tail = Some(Box::new(ListNode::new(1)));
          tail = &mut tail.as_mut().unwrap().next;
        }
        break;
      }
    }
    carry = sum / 10;
    *tail = Some(Box::new(ListNode::new(sum % 10)));
    tail = &mut tail.as_mut().unwrap().next;
  }

  head
   
}
