#![feature(int_log)]
use std::mem::size_of;

const BLOCK_SIZE_BYTES: usize = size_of::<usize>() * 8;

/// usize-ary tree with Logk(n) + 1 layers, where 'k' is the size of usize in bits, and 'n' is the ammount of items.
/// 
/// Each layer is n/k.pow(layer) bits long with every node being 1 bit.
pub struct SuccTree
{
    tree: Vec<Vec<usize>>
}

impl SuccTree
{
    /// Create a new tree with `size` items
    /// 
    /// # Example
    /// ```
    /// # use succtree::SuccTree;
    /// let mut tree = SuccTree::new(1000000);      // n = 1000000, k = usize (u64)
    /// tree.insert(5);
    /// ```
    pub fn new(size: usize) -> SuccTree
    {
        let block_size_f64 = BLOCK_SIZE_BYTES as f64;
        let layers = (size as f64).log(block_size_f64).ceil() as usize + 1;
        let mut tree = Vec::with_capacity(layers);
        for i in 0..layers
        {
            let layer_size = ((size as f64) / block_size_f64.powi(i as i32)).ceil();
            let layer_size = (layer_size / block_size_f64).ceil() as usize;
            tree.push(vec![0usize; layer_size])
        }
        SuccTree {  tree }
    }

    /// Set a bit at `item` position
    pub fn insert(&mut self, mut item: usize)
    {
        for layer in 0..=(self.tree.len() - 2)
        {
            // set the bit
            self.tree[layer][item / BLOCK_SIZE_BYTES] |= 1 << item % BLOCK_SIZE_BYTES;
            item = SuccTree::move_up_layer(item);
            if self.is_parent_set(layer + 1, item)
            {
                break;
            }
        }
    }

    /// Unset a bit at `item` position
    pub fn delete(&mut self, mut item: usize)
    {
        for layer in 0..=(self.tree.len() - 2)
        {
            // unset the bit
            self.tree[layer][item / BLOCK_SIZE_BYTES] &= !(1 << item % BLOCK_SIZE_BYTES);
            if self.is_any_sibling_set(layer, item)
            {
                break;
            }
            item = SuccTree::move_up_layer(item);
        }
    }


    /// Find the next value after `item`
    pub fn successor(&self, mut item: usize) -> Option<usize>
    {
        let mut layer = 0;
        let mut next_sibling = self.greater_sibling_in_block(layer, item);
        if next_sibling != 0
        {
            return Some(next_sibling);
        }
        while next_sibling == 0 && layer < self.tree.len() - 1
        {
            item = SuccTree::move_up_layer(item);
            layer += 1;
            next_sibling = self.greater_sibling_in_block(layer, item);
        }
        if next_sibling == 0
        {
            return None;
        }
        while layer > 0 
        {
            item = SuccTree::move_down(next_sibling);
            layer -= 1;
            next_sibling = self.first_item_set_in_block(layer, item);
        }
        Some(next_sibling)
    }


    /// Returns the range of siblings with lower inclusive and upper exclusive
    /// 
    /// # Example
    /// ```
    /// # use succtree::SuccTree;
    /// let mut tree = SuccTree::new(64);
    /// for i in 0..64
    /// {
    ///    tree.insert(i);
    /// }
    /// assert_eq!(vec![5, 6, 7, 8, 9], tree.rquery(5, 10));
    /// ```
    pub fn rquery(&self, mut lower: usize, upper: usize) -> Vec<usize>
    {
        let mut result = Vec::new();
        if (self.tree[0][lower / BLOCK_SIZE_BYTES] & 1) == 1
        {
            result.push(lower);
        }
        while let Some(next_sibling) = self.successor(lower)
        {
            if next_sibling >= upper
            {
                break;
            }
            result.push(next_sibling);
            lower = next_sibling;
        }
        result
    }


    pub fn is_empty(&self) -> bool
    {
        self.tree[self.tree.len() - 2][0] == 0
    }

    pub fn min(&self) -> Option<usize>
    {
        if self.tree[0][0] & 1 != 0
        {
            Some(0)
        }
        else
        {
            self.successor(0)
        }
    }

    /// Get the first set bit in a block
    fn first_item_set_in_block(&self, layer: usize, block: usize) -> usize
    {
        block + self.tree[layer][block / BLOCK_SIZE_BYTES].trailing_zeros() as usize
    }

    /// Get the next set bit in a block or return 0
    fn greater_sibling_in_block(&self, layer: usize, item: usize) -> usize
    {
        let mut value = self.tree[layer][item / BLOCK_SIZE_BYTES];
        let mut mask = 0;
        for i in 0..=(item % BLOCK_SIZE_BYTES)
        {
            mask |= 1 << i;
        }
        value &= !mask;
        if value == 0
        {
            return 0;
        }
        // Go to the first index of a block and add trailing zeros
        ((item / BLOCK_SIZE_BYTES) * BLOCK_SIZE_BYTES) + value.trailing_zeros() as usize
    } 

    /// In an items block, is there any other bit set
    fn is_any_sibling_set(&self, layer: usize, item: usize) -> bool
    {
        self.tree[layer][item / BLOCK_SIZE_BYTES] != 0
    }

    /// Is the parent of the current block set
    fn is_parent_set(&self, layer: usize, item: usize) -> bool
    {
        self.tree[layer][item / BLOCK_SIZE_BYTES] & 1 << item % BLOCK_SIZE_BYTES != 0
    }

    fn move_up_layer(item: usize) -> usize
    {
        item / BLOCK_SIZE_BYTES
    }

    fn move_down(item: usize) -> usize
    {
        item * BLOCK_SIZE_BYTES
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_up()
    {
        assert_eq!(0, SuccTree::move_up_layer(10));
        assert_eq!(1, SuccTree::move_up_layer(64));
        assert_eq!(1, SuccTree::move_up_layer(127));
        assert_eq!(2, SuccTree::move_up_layer(128));
        assert_eq!(3, SuccTree::move_up_layer(192));
    }

    #[test]
    fn test_move_down()
    {
        assert_eq!(0, SuccTree::move_down(0));
        assert_eq!(64, SuccTree::move_down(1));
        assert_eq!(128, SuccTree::move_down(2));
        assert_eq!(192, SuccTree::move_down(3));
        assert_eq!(256, SuccTree::move_down(4));
    }

    #[test]
    fn test_rquery()
    {
        let mut tree = SuccTree::new(1000000);
        let mut r = Vec::with_capacity(1000000);
        for i in 0..999999
        {
            tree.insert(i);
            r.push(i);
        }
        assert_eq!(r, tree.rquery(0, 1000000));
    }

    #[test]
    fn test_even_rquery()
    {
        let mut tree = SuccTree::new(1000000);
        let mut r = Vec::with_capacity(1000000);
        for i in (0..999999).step_by(2)
        {
            tree.insert(i);
            r.push(i);
        }
        assert_eq!(r, tree.rquery(0, 1000000));
    }

    #[test]
    fn test_uneven_rquery()
    {
        let mut tree = SuccTree::new(1000000);
        let mut r = Vec::with_capacity(1000000);
        for i in (1..999999).step_by(2)
        {
            tree.insert(i);
            r.push(i);
        }
        assert_eq!(r, tree.rquery(0, 1000000));
    }

    #[test]
    fn test_succ()
    {
        let mut tree = SuccTree::new(1000000);
        tree.insert(5);
        assert_eq!(None, tree.successor(5));
        tree.insert(9);
        tree.insert(30);
        tree.insert(64);
        tree.insert(65);
        tree.insert(99);
        tree.insert(99999);
        tree.insert(100000);
        assert_eq!(Some(9), tree.successor(5));
        assert_eq!(Some(30), tree.successor(9));
        assert_eq!(Some(64), tree.successor(30));        
        assert_eq!(Some(65), tree.successor(64));
        assert_eq!(Some(99), tree.successor(65));
        assert_eq!(Some(100000), tree.successor(99999));
    }

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
        let mut tree = SuccTree::new(100);
        tree.insert(0);
        assert_eq!(1, tree.tree[0][0]);
        assert_eq!(1, tree.tree[1][0]);
        tree.insert(1);
        assert_eq!(3, tree.tree[0][0]);
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
    fn test_greater_sibling_in_block()
    {
        let mut tree = SuccTree::new(1000000);
        tree.insert(0);
        tree.insert(10);
        tree.insert(50);
        assert_eq!(10, tree.greater_sibling_in_block(0, 0), "Testing next number after 0");
        assert_eq!(50, tree.greater_sibling_in_block(0, 10), "Testing next number after 10");
        assert_eq!(0, tree.greater_sibling_in_block(0, 50), "Testing next number after 50");
        tree.insert(64);
        tree.insert(70);
        tree.insert(200);
        assert_eq!(70, tree.greater_sibling_in_block(0, 64), "Testing next number after 0");
        assert_eq!(0, tree.greater_sibling_in_block(0, 70), "Testing next number after 10");
        assert_eq!(0, tree.greater_sibling_in_block(0, 200), "Testing next number after 50");
        assert_eq!(1, tree.greater_sibling_in_block(1, 0));
        assert_eq!(3, tree.greater_sibling_in_block(1, 1));
    }

    #[test]
    fn test_is_empty()
    {
        let mut tree = SuccTree::new(1000000);
        assert_eq!(true, tree.is_empty());
        tree.insert(0);
        assert_eq!(false, tree.is_empty());
    }

    #[test]
    fn test_min()
    {
        let mut tree = SuccTree::new(1000000);
        assert_eq!(None, tree.min());
        tree.insert(5);
        assert_eq!(Some(5), tree.min());
        tree.insert(0);
        assert_eq!(Some(0), tree.min());
    }

}
