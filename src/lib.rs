// usize-ary tree


#![feature(int_log)]
use std::mem::size_of;
struct SuccTree
{
    tree: Vec<Vec<usize>>
}

impl SuccTree
{
    fn new(size: usize) -> SuccTree
    {
        let k = (size_of::<usize>() * 8) as f64;
        let layers = (size as f64).log(k).ceil() as usize + 1;
        let mut tree = Vec::with_capacity(layers);
        for i in 0..layers
        {
            let layer_size = ((size as f64) / k.powi(i as i32)).ceil();
            let layer_size = (layer_size / k).ceil() as usize;
            tree.push(vec![0usize; layer_size])
        }
        SuccTree {  tree }
    }

    fn insert(&mut self, mut item: usize)
    {
        for i in 0..=(self.tree.len() - 2)
        {
            self.tree[i][item / 64] |= 1 << item % 64;
            item = ((item as f64) / 64.).floor() as usize;
            if self.is_parent_set(i + 1, item)
            {
                break;
            }
        }
    }

    fn is_parent_set(&mut self, layer: usize, item: usize) -> bool
    {
        self.tree[layer][item / 64] & 1 << item % 64 != 0
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
        tree.insert(64);
        assert_eq!(1, tree.tree[0][1]);
    }
}
