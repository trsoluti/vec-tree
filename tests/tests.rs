extern crate vec_tree;
use vec_tree::VecTree;
use vec_tree::Index;
use std::fmt::Debug;

#[test]
fn try_insert_root() {
    let mut tree = VecTree::with_capacity(1);
    let root = tree.try_insert_root(42).unwrap();
    assert_eq!(tree[root], 42);
}

#[test]
fn insert_root() {
    let mut tree = VecTree::with_capacity(1);
    let root = tree.insert_root(42);
    assert_eq!(tree[root], 42);
}

#[test]
fn try_insert() {
    let mut tree = VecTree::with_capacity(3);
    let root = tree.try_insert_root(0).unwrap();
    let child_1 = tree.try_insert(1, root).unwrap();
    let child_2 = tree.try_insert(2, root).unwrap();
    assert_eq!(tree[root], 0);
    assert_eq!(tree[child_1], 1);
    assert_eq!(tree[child_2], 2);
}

#[test]
#[should_panic]
fn try_insert_root_twice() {
    let mut tree = VecTree::with_capacity(2);
    let _root = tree.try_insert_root(42).unwrap();
    let _root2 = tree.try_insert_root(43).unwrap();
}

#[test]
fn insert_root_twice() {
    let mut tree = VecTree::with_capacity(2);
    let root = tree.insert_root(42);
    let root2 = tree.insert_root(43);
    assert_eq!(tree.get_root_index(), Some(root2));
    assert_eq!(tree.parent(root), Some(root2));
}

#[test]
fn remove_a_root_node() {
    let mut tree = VecTree::with_capacity(1);
    let root_node1 = tree.try_insert_root(42).unwrap();
    tree.remove(root_node1);
    let root_node2 = tree.try_insert_root(43).unwrap();
    assert_eq!(tree[root_node2], 43);
}

#[test]
fn cannot_get_free_value() {
    let mut tree = VecTree::with_capacity(1);
    let i = tree.try_insert_root(42).unwrap();
    assert_eq!(tree.remove(i).unwrap(), 42);
    assert!(!tree.contains(i));
}

#[test]
fn cannot_get_other_generation_value() {
    let mut tree = VecTree::with_capacity(2);
    let root_node = tree.try_insert_root(42).unwrap();
    let i = tree.try_insert(42, root_node).unwrap();
    assert_eq!(tree.remove(i).unwrap(), 42);
    assert!(!tree.contains(i));
    let j = tree.try_insert(42, root_node).unwrap();
    assert!(!tree.contains(i));
    assert_eq!(tree[j], 42);
    assert!(i != j);
}

#[test]
fn try_insert_when_full() {
    let mut tree = VecTree::with_capacity(2);
    let root_node = tree.try_insert_root(42).unwrap();
    let _child = tree.try_insert(42, root_node).unwrap();
    assert_eq!(tree.try_insert(42, root_node).unwrap_err(), 42);
}

#[test]
fn insert_many_and_cause_doubling() {
    let mut tree = VecTree::new();

    let root = tree.try_insert_root(0).unwrap();

    let indices: Vec<_> = (0..1000).map(|i| tree.insert(i * i, root)).collect();
    for (i, idx) in indices.iter().cloned().enumerate() {
        assert_eq!(tree.remove(idx).unwrap(), i * i);
        assert!(!tree.contains(idx));
    }
}

#[test]
fn capacity_and_reserve() {
    let mut tree: VecTree<usize> = VecTree::with_capacity(42);
    assert_eq!(tree.capacity(), 42);
    tree.reserve(10);
    assert_eq!(tree.capacity(), 52);
}

#[test]
fn get_mut() {
    let mut tree = VecTree::new();
    let idx = tree.insert_root(5);
    tree[idx] += 1;
    assert_eq!(tree[idx], 6);
}

#[test]
#[should_panic]
fn index_deleted_item() {
    let mut tree = VecTree::new();
    let idx = tree.insert_root(42);
    tree.remove(idx);
    tree[idx];
}

#[test]
fn check_the_validity_of_the_tree_after_remove() {
    let mut tree: VecTree<usize> = VecTree::with_capacity(4);
    let root = tree.try_insert_root(0).unwrap();
    let child1 = tree.try_insert(1, root).unwrap();
    let child2 = tree.try_insert(2, root).unwrap();
    let child3 = tree.try_insert(3, root).unwrap();

    tree.remove(child3);
    tree.try_insert(4, root).unwrap();

    assert_eq!(
        tree.children(root)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [1, 2, 4]
    );

    tree.remove(child2);
    tree.try_insert(5, root).unwrap();

    assert_eq!(
        tree.children(root)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [1, 4, 5]
    );

    tree.remove(child1);
    tree.try_insert(6, root).unwrap();

    assert_eq!(
        tree.children(root)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [4, 5, 6]
    );
}

#[test]
fn check_remove_with_one_child() {
    let mut tree: VecTree<usize> = VecTree::with_capacity(2);
    let root = tree.try_insert_root(0).unwrap();

    let child1 = tree.try_insert(1, root).unwrap();
    tree.remove(child1);

    assert_eq!(
        tree.children(root)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        []
    );

    let child2 = tree.try_insert(2, root).unwrap();

    assert_eq!(
        tree.children(root)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [2]
    );

    tree.remove(child2);

    assert_eq!(
        tree.children(root)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        []
    );
}

#[test]
fn out_of_bounds_get_with_index_from_other_tree() {
    let mut tree1 = VecTree::with_capacity(1);
    let mut tree2 = VecTree::with_capacity(1);
    let root_tree1 = tree1.insert_root(1);
    let _root_tree2 = tree2.insert_root(2);
    let child_tree1 = tree1.insert(2, root_tree1);
    assert!(tree2.get(child_tree1).is_none());
}

#[test]
fn out_of_bounds_remove_with_index_from_other_tree() {
    let mut tree1 = VecTree::with_capacity(1);
    let mut tree2 = VecTree::with_capacity(1);
    let root_tree1 = tree1.insert_root(1);
    let _root_tree2 = tree2.insert_root(2);
    let child_tree1 = tree1.insert(2, root_tree1);
    assert!(tree2.remove(child_tree1).is_none());
}

#[test]
fn get_parent() {
    let mut tree = VecTree::new();

    let grand_child = {
        let root_node = tree.insert_root(1);
        let child = tree.insert(2, root_node);

        tree.insert(3, child)
    };

    let child = tree.parent(grand_child).unwrap();
    assert_eq!(tree[child], 2);
    let root_node = tree.parent(child).unwrap();
    assert_eq!(tree[root_node], 1);
    assert_eq!(tree.parent(root_node), None);
}

#[test]
fn add_children_and_iterate_over_it() {
    let mut tree = VecTree::new();

    let root_node = tree.insert_root(1);
    let child_node_1 = tree.insert(2, root_node);
    let child_node_2 = tree.insert(3, root_node);
    let child_node_3 = tree.insert(4, root_node);
    let _grandchild = tree.insert(5, child_node_3);

    assert_eq!(
        tree.children(root_node)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [2, 3, 4]
    );

    assert_eq!(
        tree.children(child_node_1)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        []
    );

    assert_eq!(
        tree.children(child_node_2)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        []
    );

    assert_eq!(
        tree.children(child_node_3)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [5]
    );
}

#[test]
fn iterate_over_preceding_siblings() {
    let mut tree = VecTree::new();

    let root_node = tree.insert_root(1);
    let child_node_1 = tree.insert(2, root_node);
    let child_node_2 = tree.insert(3, root_node);
    let child_node_3 = tree.insert(4, root_node);

    assert_eq!(
        tree.preceding_siblings(root_node)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [1]
    );

    assert_eq!(
        tree.preceding_siblings(child_node_1)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [2]
    );

    assert_eq!(
        tree.preceding_siblings(child_node_2)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [3, 2]
    );

    assert_eq!(
        tree.preceding_siblings(child_node_3)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [4, 3, 2]
    );
}

#[test]
fn iterate_over_following_siblings() {
    let mut tree = VecTree::new();

    let root_node = tree.insert_root(1);
    let child_node_1 = tree.insert(2, root_node);
    let child_node_2 = tree.insert(3, root_node);
    let child_node_3 = tree.insert(4, root_node);

    assert_eq!(
        tree.following_siblings(root_node)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [1]
    );

    assert_eq!(
        tree.following_siblings(child_node_1)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [2, 3, 4]
    );

    assert_eq!(
        tree.following_siblings(child_node_2)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [3, 4]
    );

    assert_eq!(
        tree.following_siblings(child_node_3)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [4]
    );
}

#[test]
fn iterate_over_ancestors() {
    let mut tree = VecTree::new();

    let root_node = tree.insert_root(1);
    let child_node_1 = tree.insert(2, root_node);
    let child_node_2 = tree.insert(3, root_node);
    let grandchild = tree.insert(5, child_node_2);

    assert_eq!(
        tree.ancestors(root_node)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [1]
    );

    assert_eq!(
        tree.ancestors(child_node_1)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [2, 1]
    );

    assert_eq!(
        tree.ancestors(child_node_2)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [3, 1]
    );

    assert_eq!(
        tree.ancestors(grandchild)
            .map(|node_id| tree[node_id])
            .collect::<Vec<_>>(),
        [5, 3, 1]
    );
}

#[test]
fn iterate_over_descendants() {
    let mut tree = VecTree::new();

    // 0-1-4-6
    // | `-5
    // `-2
    // `-3
    let root_node = tree.insert_root(0);
    let node_1 = tree.insert(1, root_node);
    let node_2 = tree.insert(2, root_node);
    let _node_3 = tree.insert(3, root_node);
    let node_4 = tree.insert(4, node_1);
    let _node_5 = tree.insert(5, node_1);
    let _node_6 = tree.insert(6, node_4);
    let _node_7 = tree.insert(7, node_2);

    let descendants = tree
        .descendants(root_node)
        .map(|node| tree[node])
        .collect::<Vec<i32>>();

    let expected_result = [0, 1, 4, 6, 5, 2, 7, 3];

    assert_eq!(descendants, expected_result);
}

#[test]
fn iterate_over_descendants_with_depth() {
    let mut tree = VecTree::new();

    // 0-1-4-6
    // | `-5
    // `-2
    // `-3
    let root_node = tree.insert_root(0);
    let node_1 = tree.insert(1, root_node);
    let node_2 = tree.insert(2, root_node);
    let _node_3 = tree.insert(3, root_node);
    let node_4 = tree.insert(4, node_1);
    let _node_5 = tree.insert(5, node_1);
    let _node_6 = tree.insert(6, node_4);
    let _node_7 = tree.insert(7, node_2);

    let descendants = tree
        .descendants_with_depth(root_node)
        .map(|(node, depth)| (tree[node], depth))
        .collect::<Vec<(i32, u32)>>();

    let expected_result = [
        (0, 0),
        (1, 1),
        (4, 2),
        (6, 3),
        (5, 2),
        (2, 1),
        (7, 2),
        (3, 1),
    ];

    assert_eq!(descendants, expected_result);
}

#[test]
// It would panic when adding node_5 if the nodes where not recursively removed.
fn check_descendants_are_removed() {
    let mut tree = VecTree::with_capacity(5);

    // 0-1-3-4
    //   `-2
    let root_node = tree.try_insert_root(0).unwrap();
    let node_1 = tree.try_insert(1, root_node).unwrap();
    let _node_2 = tree.try_insert(2, node_1).unwrap();
    let node_3 = tree.try_insert(3, node_1).unwrap();
    let _node_4 = tree.try_insert(4, node_3).unwrap();

    let descendants = tree
        .descendants(root_node)
        .map(|node| tree[node])
        .collect::<Vec<i32>>();

    assert_eq!(descendants, [0, 1, 2, 3, 4]);

    // 0
    tree.remove(node_1);

    // 0-5-7-8
    //   `-6
    let node_5 = tree.try_insert(5, root_node).unwrap();
    let _node_6 = tree.try_insert(6, node_5).unwrap();
    let node_7 = tree.try_insert(7, node_5).unwrap();
    let _node_8 = tree.try_insert(8, node_7).unwrap();

    let descendants = tree
        .descendants(root_node)
        .map(|node| tree[node])
        .collect::<Vec<i32>>();

    assert_eq!(descendants, [0, 5, 6, 7, 8]);
}

#[test]
fn move_a_node() {
    let mut tree = VecTree::with_capacity(3);
    let root_node = tree.try_insert_root(0).unwrap();
    let node_1 = tree.try_insert(1, root_node).unwrap();
    let _node_2 = tree.try_insert(2, root_node).unwrap();

    let descendants = tree
        .descendants(root_node)
        .map(|node| tree[node])
        .collect::<Vec<i32>>();

    assert_eq!(descendants, [0, 1, 2]);

    tree.append_child(root_node, node_1);

    let descendants = tree
        .descendants(root_node)
        .map(|node| tree[node])
        .collect::<Vec<i32>>();

    assert_eq!(descendants, [0, 2, 1]);
}


#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn test_insert_root() {
    let mut tree = VecTree::<i32>::new();
    let no_root = tree.get_root_index();
    assert_eq!(true, no_root.is_none());
    let old_root_index = tree.insert_root(19);
    let new_root_index = tree.insert_root(20);
    assert_eq!(true, tree.get_root_index().is_some());
    assert_eq!(new_root_index, tree.get_root_index().unwrap());
    let children: Vec<Index> = tree.children(new_root_index).collect();
    assert_eq!(1, children.len());
    assert_eq!(old_root_index, children[0]);
}
#[test]
fn test_fork() {
    // Test forking the only node in the tree
    let mut tree = VecTree::<i32>::new();
    let insert_1 = tree.insert_root(101);
    let old_root = tree.get_root_index().unwrap();
    assert_eq!(insert_1, old_root);
    let insert_2 = tree.fork(old_root, 100, 102).unwrap();
    let new_root = tree.get_root_index().unwrap();
    assert_ne!(old_root, new_root);
    assert_ne!(insert_2, new_root);
    let children = tree.children(new_root).collect::<Vec<Index>>();
    assert_eq!(2, children.len());
    assert_eq!(insert_1, children[0]);
    assert_eq!(insert_2, children[1]);

    // Test forking the left child
    let mut tree = VecTree::<i32>::new();
    let root = tree.insert_root(1);
    let old_parent = tree.insert(111, root);
    let uncle = tree.insert(12, root);
    let new_brother = tree.fork(old_parent, 11, 112);
    assert_eq!(true, new_brother.is_some());
    assert_eq!(root, tree.get_root_index().unwrap());
    assert_eq!(tree.parent(old_parent), tree.parent(new_brother.unwrap()));
    assert_ne!(tree.parent(old_parent), tree.parent(uncle));
    let older_generation = tree.children(tree.get_root_index().unwrap()).collect::<Vec<Index>>();
    assert_eq!(2, older_generation.len());
    assert_eq!(uncle, older_generation[1]);
    assert_eq!(tree.parent(old_parent).unwrap(), older_generation[0]);

    // Test forking the right child
    let mut tree = VecTree::<i32>::new();
    let root = tree.insert_root(1);
    let auntie = tree.insert(11, root);
    let old_parent = tree.insert(121, root);
    let new_brother = tree.fork(old_parent, 12, 122);
    assert_eq!(true, new_brother.is_some());
    assert_eq!(root, tree.get_root_index().unwrap());
    assert_eq!(tree.parent(old_parent), tree.parent(new_brother.unwrap()));
    assert_ne!(tree.parent(old_parent), tree.parent(auntie));
    let older_generation = tree.children(tree.get_root_index().unwrap()).collect::<Vec<Index>>();
    assert_eq!(2, older_generation.len());
    assert_eq!(auntie, older_generation[0]);
    assert_eq!(tree.parent(old_parent).unwrap(), older_generation[1]);

    // Test forking the middle child
    let mut tree = VecTree::<i32>::new();
    let root = tree.insert_root(1);
    let auntie = tree.insert(11, root);
    let old_parent = tree.insert(121, root);
    let uncle = tree.insert(13, root);
    let new_brother = tree.fork(old_parent, 12, 122);
    assert_eq!(true, new_brother.is_some());
    assert_eq!(root, tree.get_root_index().unwrap());
    assert_eq!(tree.parent(old_parent), tree.parent(new_brother.unwrap()));
    assert_ne!(tree.parent(old_parent), tree.parent(auntie));
    assert_ne!(tree.parent(old_parent), tree.parent(uncle));
    let older_generation = tree.children(tree.get_root_index().unwrap()).collect::<Vec<Index>>();
    assert_eq!(3, older_generation.len());
    assert_eq!(auntie, older_generation[0]);
    assert_eq!(tree.parent(old_parent).unwrap(), older_generation[1]);
    assert_eq!(uncle, older_generation[2]);

    // Test that the original children are unharmed
    // Test forking the root of a non-empty tree
    let mut tree = VecTree::<i32>::new();
    let ma = tree.insert_root(1);
    let baby_bro = tree.insert(11, ma);
    let older_sis = tree.insert(12, ma);
    let new_uncle = tree.fork(ma, 0, 2);
    assert_eq!(true, new_uncle.is_some());
    assert_ne!(ma, tree.get_root_index().unwrap());
    let children = tree.children(ma).collect::<Vec<Index>>();
    assert_eq!(2, children.len());
    assert_eq!(baby_bro, children[0]);
    assert_eq!(older_sis, children[1]);
    // Test forking when the node is not there
    let mut tree = VecTree::<i32>::new();
    let grandpa = tree.insert_root(1);
    let pa = tree.insert(11, grandpa);
    let dead_uncle = tree.insert(12, grandpa);
    let estate = tree.remove(dead_uncle);
    assert_eq!(true, tree.contains(pa));
    assert_eq!(true, estate.is_some());
    assert_eq!(12, estate.unwrap());
    let cousin = tree.fork(dead_uncle, 9, 99);
    assert_eq!(true, cousin.is_none());

    // Test forking when it means increasing the capacity
    let mut tree = VecTree::<i32>::with_capacity(2);
    let item_1 = tree.insert_root(1);
    let item_2 = tree.fork(item_1, 0, 2);
    assert_eq!(true, item_2.is_some());
    assert_ne!(2, tree.capacity());
}

#[test]
fn test_merge() {
    // Test trying to merge when first node not in tree
    let mut tree = VecTree::new();
    let missing_node_id = tree.insert_root(0);
    tree.remove(missing_node_id);
    let real_root = tree.insert_root(1);
    let real_child = tree.insert(11, real_root);
    let node_count = tree.descendants(real_root)
        .collect::<Vec<Index>>()
        .len();
    assert_eq!(2, node_count);
    tree.merge(missing_node_id, real_child);
    assert_eq!(node_count,
               tree.descendants(real_root)
                   .collect::<Vec<Index>>()
                   .len()
    );

    // Test trying to merge when second node not in tree
    tree.merge(real_child, missing_node_id);
    assert_eq!(node_count,
               tree.descendants(real_root)
                   .collect::<Vec<Index>>()
                   .len()
    );

    // Test trying to merge when don't have same parent
    let real_child_sibling = tree.insert(12, real_root);
    let node_count = tree.descendants(real_root)
        .collect::<Vec<Index>>()
        .len();
    assert_eq!(3, node_count);
    tree.merge(real_root, real_child);
    assert_eq!(node_count,
               tree.descendants(real_root)
                   .collect::<Vec<Index>>()
                   .len()
    );

    // Test trying to merge a node with itself
    tree.merge(real_child, real_child);
    assert_eq!(node_count,
               tree.descendants(real_root)
                   .collect::<Vec<Index>>()
                   .len()
    );

    // Test merging two nodes without children and without siblings
    tree.merge(real_child, real_child_sibling);
    assert_ne!(Some(real_root), tree.get_root_index());
    let new_root = tree.get_root_index().unwrap();
    assert_eq!(1,
               tree.descendants(new_root)
                   .collect::<Vec<Index>>()
                   .len()
    );
    assert_eq!(false, tree.contains(real_child));
    assert_eq!(None, tree.parent(real_child));

    tree.clear();
    let root_id = tree.insert_root(1);
    let parent_id = tree.insert(11, root_id);
    let _uncle_id = tree.insert(12, root_id);
    let child_1_id = tree.insert(111, parent_id);
    let child_2_id = tree.insert(112, parent_id);
    /*+ for debugging
    println!("*******BEFORE MERGE:************");
    for &node_id in &[ root_id, parent_id, uncle_id, child_1_id, child_2_id] {
        if !tree.contains(node_id) {
            println!("node {:#?} ABSENT", node_id);
        } else {
            println!("node {:#?}:\n{:#?}",node_id, tree.nodes[node_id]);
        }
    }
    */
    tree.merge(child_1_id, child_2_id);
    assert_eq!(Some(root_id), tree.get_root_index());
    /*+ for debugging
    println!("*******AFTER MERGE:************");
    for &node_id in &[ root_id, parent_id, uncle_id, child_1_id, child_2_id] {
        if !tree.contains(node_id) {
            println!("node {:#?} ABSENT", node_id);
        } else {
            println!("node {:#?}:\n{:#?}",node_id, tree.nodes[node_id]);
        }
    }
    */
    println!("About to descend...");
    assert_eq!(3,
               tree.descendants(root_id)
                   .collect::<Vec<Index>>()
                   .len()
    );

    // Test merging two nodes where only the first has children
    tree.clear();
    let root_id = tree.insert_root(1);
    let parent_id = tree.insert(11, root_id);
    let uncle_id = tree.insert(12, root_id);
    let child_1_id = tree.insert(111, parent_id);
    let child_2_id = tree.insert(112, parent_id);
    tree.merge(parent_id, uncle_id);
    let new_root = tree.get_root_index().unwrap();
    assert_ne!(root_id, new_root);
    assert_eq!(uncle_id, new_root);
    assert_eq!(Some(uncle_id), tree.parent(child_1_id));
    /*+ for debugging
    for &node_id in &[ root_id, parent_id, uncle_id, child_1_id, child_2_id] {
        if !tree.contains(node_id) {
            println!("node {:#?} ABSENT", node_id);
        } else {
            println!("node {:#?}:\n{:#?}",node_id, tree.nodes[node_id]);
        }
    }
    */
    assert_eq!(Some(uncle_id), tree.parent(child_2_id));


    // Test merging where only the second has children
    tree.clear();
    let root_id = tree.insert_root(1);
    let auntie_id = tree.insert(11, root_id);
    let parent_id = tree.insert(12, root_id);
    let child_1_id = tree.insert(121, parent_id);
    let child_2_id = tree.insert(122, parent_id);
    tree.merge(parent_id, auntie_id);
    /*+ for debugging
    for &node_id in &[ root_id, auntie_id, parent_id, child_1_id, child_2_id] {
        if !tree.contains(node_id) {
            println!("node {:#?} ABSENT", node_id);
        } else {
            println!("node {:#?}:\n{:#?}",node_id, tree.nodes[node_id]);
        }
    }
    */
    let new_root = tree.get_root_index().unwrap();
    assert_ne!(root_id, new_root);
    assert_eq!(auntie_id, new_root);
    assert_eq!(Some(auntie_id), tree.parent(child_1_id));
    assert_eq!(Some(auntie_id), tree.parent(child_2_id));

    // Test merging where both have children
    tree.clear();
    let root = tree.insert_root(1000);
    let gp1 = tree.insert(1100, root);
    let _gp2 = tree.insert(1200, root);
    let p1 = tree.insert(1110, gp1);
    let p2 = tree.insert(1120, gp1);
    let c11 = tree.insert(1111, p1);
    let c12 = tree.insert(1112, p1);
    let c21 = tree.insert(1121, p2);
    let c22 = tree.insert(1122, p2);
    tree.merge(p1, p2);
    assert_eq!(Some(root), tree.get_root_index());
    assert_eq!(false, tree.contains(p1));
    assert_eq!(false, tree.contains(gp1));
    //+print_nodes(&tree, &[root, gp1, gp2, p1, p2, c11, c12, c21, c22]);
    assert_eq!(Some(root), tree.parent(p2));
    assert_eq!(Some(p2), tree.parent(c11));
    let children = tree.children(p2).collect::<Vec<Index>>();
    let expected_children = vec!(c11, c12, c21, c22);
    assert_eq!(expected_children, children);

    // Test merging where there are siblings before
    tree.clear();
    let root = tree.insert_root(1000);
    let _gp1 = tree.insert(1100, root);
    let gp2 = tree.insert(1200, root);
    let _gp3 = tree.insert(1300, root);
    let p1 = tree.insert(1110, gp2);
    let p2 = tree.insert(1120, gp2);
    let p3 = tree.insert(1130, gp2);
    let c11 = tree.insert(1111, p2);
    let c12 = tree.insert(1112, p2);
    let c21 = tree.insert(1121, p3);
    let c22 = tree.insert(1122, p3);
    tree.merge(p2, p3);
    assert_eq!(true, tree.contains(gp2)); // shouldn't collapse.
    let children = tree.children(gp2).collect::<Vec<Index>>();
    assert_eq!(vec!(p1,p3), children);
    let children = tree.children(p3).collect::<Vec<Index>>();
    assert_eq!(vec!(c11,c12,c21,c22), children);
    // Test merging where there are siblings between
    tree.clear();
    let root = tree.insert_root(1000);
    let _gp1 = tree.insert(1100, root);
    let gp2 = tree.insert(1200, root);
    let _gp3 = tree.insert(1300, root);
    let p1 = tree.insert(1110, gp2);
    let p2 = tree.insert(1120, gp2);
    let p3 = tree.insert(1130, gp2);
    let c11 = tree.insert(1111, p1);
    let c12 = tree.insert(1112, p1);
    let _c21 = tree.insert(1121, p2);
    let _c22 = tree.insert(1122, p2);
    let c31 = tree.insert(1121, p3);
    let c32 = tree.insert(1122, p3);
    tree.merge(p1, p3);
    assert_eq!(true, tree.contains(gp2)); // shouldn't collapse.
    let children = tree.children(gp2).collect::<Vec<Index>>();
    assert_eq!(vec!(p2,p3), children);
    let children = tree.children(p3).collect::<Vec<Index>>();
    assert_eq!(vec!(c11,c12,c31,c32), children);

    // Test merging where there are siblings after
    tree.clear();
    let root = tree.insert_root(1000);
    let _gp1 = tree.insert(1100, root);
    let gp2 = tree.insert(1200, root);
    let _gp3 = tree.insert(1300, root);
    let p1 = tree.insert(1110, gp2);
    let p2 = tree.insert(1120, gp2);
    let p3 = tree.insert(1130, gp2);
    let c11 = tree.insert(1111, p1);
    let c12 = tree.insert(1112, p1);
    let c21 = tree.insert(1121, p2);
    let c22 = tree.insert(1122, p2);
    let _c31 = tree.insert(1121, p3);
    let _c32 = tree.insert(1122, p3);
    tree.merge(p1, p2);
    assert_eq!(true, tree.contains(gp2)); // shouldn't collapse.
    let children = tree.children(gp2).collect::<Vec<Index>>();
    assert_eq!(vec!(p2,p3), children);
    let children = tree.children(p2).collect::<Vec<Index>>();
    assert_eq!(vec!(c11,c12,c21,c22), children);

    // Test merging where the parent is root
    tree.clear();
    let root = tree.insert_root(10);
    let c1 = tree.insert(11,root);
    let c2 = tree.insert(12, root);
    tree.merge(c1, c2);
    assert_ne!(Some(root), tree.get_root_index());
    assert_eq!(Some(c2), tree.get_root_index());
    assert_eq!(false, tree.contains(root));
    assert_eq!(false, tree.contains(c1));
}
// For debugging. Print the value of specific nodes
fn print_nodes<T>(tree: &VecTree<T>, node_ids: &[Index])
    where
        T: Debug
{
    for &node_id in node_ids {
        if !tree.contains(node_id) {
            println!("node {:#?} ABSENT", node_id);
        } else {
            println!("node {:#?}:\n{:#?}",node_id, tree[node_id]);
        }
    }
}

#[test]
fn test_ancestors() {
    // We seem to have some problem if we
    // merge the two children of a root node
    // then try to traverse up.
    let mut tree = VecTree::new();
    let root = tree.insert_root(1);
    let new_sibling = tree.fork(root, 0, 2).unwrap();
    let new_root = tree.get_root_index().unwrap();
    println!("Before merge:");
    print_nodes(&tree, &[new_root, root, new_sibling]);
    tree.merge(root, new_sibling);
    println!("After merge:");
    print_nodes(&tree, &[new_root, root, new_sibling]);
    assert_eq!(None, tree.parent(new_sibling));
}
