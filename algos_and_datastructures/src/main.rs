mod linkedlists;
use linkedlists::LinkedList;

mod doublylinked;
use doublylinked::DbList;

mod binarytree;
use binarytree::BinTree;

mod balancedtree;
use balancedtree::BinTree as BalTree;

fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_front(1);
    ll.push_back(5);
    ll.insert_sorted(4);
    dbg!(ll);

    println!("\n\n");

    let mut dl = DbList::new();
    dl.push_back(3);
    dl.push_front(2);
    dl.push_back(4);
    dl.push_front(1);
    dbg!(dl);

    println!("\n\n");

    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(1);
    t.add_sorted(3);
    t.add_sorted(7);
    t.add_sorted(6);
    t.print_left_first(0);

    println!("\n\n");

    let mut bt = BalTree::new();
    bt.add_sorted(4);
    bt.add_sorted(6);
    bt.add_sorted(1);
    bt.add_sorted(3);
    bt.add_sorted(8);
    bt.add_sorted(7);
    bt.add_sorted(5);
    for i in 0..100000 {
        bt.add_sorted(i);
    }
    bt.print_left_first(0);
}
