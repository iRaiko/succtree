// usize-ary tree

#![feature(int_log)]
use std::mem::size_of;

const K: usize = size_of::<usize>() * 8;

pub struct SuccTree
{
    tree: Vec<Vec<usize>>
}

impl SuccTree
{
    pub fn new(size: usize) -> SuccTree
    {
        let k_f64 = K as f64;
        let layers = (size as f64).log(k_f64).ceil() as usize + 1;
        let mut tree = Vec::with_capacity(layers);
        for i in 0..layers
        {
            let layer_size = ((size as f64) / k_f64.powi(i as i32)).ceil();
            let layer_size = (layer_size / k_f64).ceil() as usize;
            tree.push(vec![0usize; layer_size])
        }
        SuccTree {  tree }
    }

    pub fn insert(&mut self, mut item: usize)
    {
        for i in 0..=(self.tree.len() - 2)
        {
            // set the bit
            self.tree[i][item / K] |= 1 << item % K;
            item = ((item as f64) / K as f64).floor() as usize;
            if self.is_parent_set(i + 1, item)
            {
                break;
            }
        }
    }

    pub fn delete(&mut self, mut item: usize)
    {
        for i in 0..=(self.tree.len() - 2)
        {
            // unset the bit
            self.tree[i][item / K] &= !(1 << item % K);
            if self.is_any_sibling_set(i, item)
            {
                break;
            }
            item = ((item as f64) / K as f64).floor() as usize;
        }
    }

    pub fn succ(&self, mut item: usize) -> usize
    {
        let mut layer = 0;
        let mut x = self.greater_sibling(layer, item);
        while x == 0 || layer < self.tree.len() - 1
        {
            item = ((item as f64) / K as f64).floor() as usize;
            layer += 1;
            x = self.greater_sibling(layer, item);
        }
        if x == 0
        {
            return usize::MAX;
        }
        while layer > 0
        {
            item = K * item + (x.trailing_zeros() as usize);
            layer -= 1;
            x = self.greater_sibling(layer, item);
        }
        K * item + (x.trailing_zeros() as usize)
    }

    fn greater_sibling(&self, layer: usize, item: usize) -> usize
    {
        let mut mask = 0;
        for i in 0..=(item % K)
        {
            mask |= 1 << i;
        }
        let mut x = self.tree[layer][item / K].clone();
        x &= !mask;
        x
    }

    fn is_any_sibling_set(&self, layer: usize, item: usize) -> bool
    {
        self.tree[layer][item / K] != 0
    }

    fn is_parent_set(&self, layer: usize, item: usize) -> bool
    {
        self.tree[layer][item / K] & 1 << item % K != 0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new()
    {
        let tree = SuccTree::new(1000000);
        assert_eq!(tree.tree.len(), 5);
        assert_eq!(tree.tree[0].len(), 15625);
        assert_eq!(tree.tree[1].len(), 245);
        assert_eq!(tree.tree[2].len(), 4);
        assert_eq!(tree.tree[3].len(), 1);
        assert_eq!(tree.tree[4].len(), 1);

        let tree = SuccTree::new(64);
        assert_eq!(tree.tree.len(), 2);
        assert_eq!(tree.tree[0].len(), 1);
        assert_eq!(tree.tree[1].len(), 1);
    }

    #[test]
    fn test_insert()
    {
        let mut tree = SuccTree::new(1000000);
        tree.insert(0);
        assert_eq!(1, tree.tree[0][0]);
        assert_eq!(1, tree.tree[1][0]);
        assert_eq!(1, tree.tree[2][0]);
        assert_eq!(1, tree.tree[3][0]);
        tree.insert(1);
        assert_eq!(3, tree.tree[0][0]);
        tree.insert(64);
        assert_eq!(1, tree.tree[0][1]);
    }

    #[test]
    fn test_delete()
    {
        let mut tree = SuccTree::new(1000000);
        tree.insert(0);
        assert_eq!(1, tree.tree[0][0]);
        assert_eq!(1, tree.tree[1][0]);
        assert_eq!(1, tree.tree[2][0]);
        assert_eq!(1, tree.tree[3][0]);
        tree.delete(0);
        assert_eq!(0, tree.tree[0][0]);
        assert_eq!(0, tree.tree[1][0]);
        assert_eq!(0, tree.tree[2][0]);
        assert_eq!(0, tree.tree[3][0]);
        tree.insert(0);
        tree.insert(1);
        tree.delete(0);
        assert_eq!(2, tree.tree[0][0]);
        assert_eq!(1, tree.tree[1][0]);
        assert_eq!(1, tree.tree[2][0]);
        assert_eq!(1, tree.tree[3][0]);
        tree.delete(1);
        assert_eq!(0, tree.tree[0][0]);
        assert_eq!(0, tree.tree[1][0]);
        assert_eq!(0, tree.tree[2][0]);
        assert_eq!(0, tree.tree[3][0]);
    }

    #[test]
    fn succ()
    {
        
    }

    #[test]
    fn test_greater_sibling()
    {
        let mut tree = SuccTree::new(1000000);
        tree.insert(5);
        assert_eq!(0, tree.greater_sibling(0, 5));
        tree.insert(6);
        assert_eq!(64, tree.greater_sibling(0, 5));
    }
}
