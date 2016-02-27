#![cfg(test)]

use first::BST;

#[test]
fn test_new() -> () {
    let tree = BST::new();
    println!("{:#?}", tree);
}

#[test]
fn test_simple() -> () {
    let mut tree = BST::new();
    tree.insert(3);
    tree.insert(2);
    println!("{:#?}", tree);
    assert_eq!(tree.search(3), true);
}
