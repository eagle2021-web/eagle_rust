#[path="./treenode.rs"]
mod treenode;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Solution {
    pub mask: usize,
    pub res: Vec<Vec<String>>,
    pub record: Vec<usize>,
    pub n: usize,
}


pub fn dfs2(row: usize, col: usize, pie: usize, na: usize, n: usize, mask: usize, record: &mut Vec<usize>, res: &mut Vec<Vec<String>>) {
    if row == n {
        let mut s_arr = vec![];
        for v in record.iter().enumerate() {
            let mut cs = vec!['.' as u8; n];
            cs[*v.1] = 'Q' as u8;
            s_arr.push(String::from_utf8(cs).unwrap());
        }
        res.push(s_arr);
    } else {
        let used = col | pie | na;
        let mut can_used = mask & !used;
        while can_used != 0 {
            let tmp_can_used = can_used & (can_used - 1);
            let used_col = tmp_can_used ^ can_used;
            can_used = tmp_can_used;

            record[row] = (used_col - 1).count_ones() as usize;
            dfs2( row + 1, used_col | col, (pie | used_col) << 1, (na | used_col) >> 1, n, mask, record, res);
        }
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let (n, mask, mut record, mut res) = (n as usize, (1 << n) - 1, vec![0; n as usize], vec![]);
        dfs2(0, 0, 0, 0, n, mask, &mut record, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let a = Solution::solve_n_queens(4);
        println!("a = {:?}", a);
    }

    #[test]
    fn it_works3() {
        let a = 1;
        let b = a & (a - 1);
        let c = a & !a;
        println!("b = {:?}", b);
        println!("c = {:?}", c);

        assert_eq!(7_usize.count_ones(), 3);
        assert_eq!(6_i32.count_ones(), 2);
        assert_eq!(1_usize.count_ones(), 1);
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ptr;
use crate::treenode::TreeNode;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];
        if root.is_none() {
            return v;
        }
        let mut stk = VecDeque::new();
        let ref_root = root.as_ref().unwrap();
        let mut p = Some(Rc::clone(ref_root));
        while !stk.is_empty() || p.is_some() {
            while p.is_some() {
                let node = p.unwrap();
                stk.push_back(Some(Rc::clone(&node)));
                p = node.borrow_mut().left.clone();
            }
            let top = stk.pop_back().unwrap();
            let node = top.unwrap();
            v.push(node.borrow().val);
            p = node.borrow_mut().right.clone();
        }
        v
    }


    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];
        if root.is_none() {
            return v;
        }
        let mut stk = Vec::new();
        let ref_root = root.as_ref().unwrap();
        let mut p = Some(Rc::clone(ref_root));
        while !stk.is_empty() || p.is_some() {
            if p.is_some() {
                let node = p.unwrap();
                v.push(node.borrow().val);
                let right = node.borrow().right.clone();
                if right.is_some() {
                    stk.push(right);
                }
                p = node.borrow().left.clone();
                continue;
            }
            let top = stk.pop().unwrap();
            p = Some(top.unwrap().clone());

        }
        v
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];
        if root.is_none() {
            return v;
        }
        let mut stk = Vec::new();
        let ref_root = root.as_ref().unwrap();
        let mut p = Some(Rc::clone(ref_root));
        let mut prev = None;
        while !stk.is_empty() || p.is_some() {
            while p.is_some() {
                let node = p.unwrap();
                prev = Some(Rc::clone(&node));
                stk.push(Some(Rc::clone(&node)));
                p = node.borrow_mut().left.clone();
            }
            let top = stk.pop().unwrap();
            let node = top.unwrap();
            let right = node.borrow().right.clone();
            if prev.is_none() || right.is_none() {
                v.push(node.borrow().val);
                prev = Some(Rc::clone(&node));
                continue
            }
            let prev_node = prev.unwrap();
            let prev_node_ref = prev_node.as_ptr();
            let right_node = right.unwrap();
            let right_node_ref = right_node.as_ptr();
            if ptr::eq(right_node_ref, prev_node_ref){
                v.push(node.borrow().val);
                prev = Some(Rc::clone(&node));
                continue
            }
            stk.push(Some(Rc::clone(&node)));
            prev = Some(Rc::clone(&node));
            p = node.borrow_mut().right.clone();
        }
        v
    }
}
#[test]
fn testa(){
    let a = TreeNode{
        val: 0,
        left: None,
        right: None,
    };
    let b = TreeNode{
        val: 0,
        left: None,
        right: None,
    };
    let c = Rc::new(a);
    let d = Rc::clone(&c);
    println!( "{}", ptr::eq(&c, &d));
    // let c = Some(a);
    // let d: Option<TreeNode> = None;
    // println!( "{}", ptr::eq(&c, &d));
}