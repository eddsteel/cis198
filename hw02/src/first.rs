use std::mem;

#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}


impl BST {
    pub fn new() -> Self {
        BST {root: Link::Empty}
    }

    pub fn insert(&mut self, i: i32) -> bool {
        let (new_root, change) = self.root.insert(i);
        if change {
            self.root = new_root;
        }
        change
    }

    pub fn search(self, i: i32) -> bool {
        self.root.search(i)
    }
}


impl Link {

    fn search(&self, i: i32) -> bool {
        match *self {
            Link::Empty => false,

            Link::More(ref n) => {
                if n.elem == i {
                    true
                } else {
                    if i < n.elem {
                        n.left.search(i)
                    } else {
                        n.right.search(i)
                    }
                }
            }
        }
    }


    // I don't see how to make this return just bool.
    // If I'm a Link::Empty, how do I replace myself?
    //
    fn insert(&mut self, i: i32) -> (Link, bool) {
        match *self {
            Link::Empty => { // "place element in list" How? reassign self? wat?
                let new_elem = Link::More(Box::new(Node {
                    elem: i,
                    left: Link::Empty,
                    right: Link::Empty,
                }));
                (new_elem, true)
            }
            // return false if the element is in this node
            // otherwise recurse to left if i < node's value
            // recurse to right if i > node's value
            Link::More(ref mut b) => {
                if b.elem == i {
                    (Link::Empty, false) // I hate this.
                }
                else {
                    if i < b.elem {
                        let (new_left, change) = b.left.insert(i);
                        (Link::More(Box::new(Node {
                            left: new_left,
                            right: mem::replace(&mut b.right, Link::Empty),
                            elem: b.elem,
                        })), change)
                    }
                    else {
                        let (new_right, change) = b.right.insert(i);
                        (Link::More(Box::new(Node {
                            left: mem::replace(&mut b.left, Link::Empty),
                            right: new_right,
                            elem: b.elem,
                        })), change)
                    }
                }
            }
        }
    }
}
