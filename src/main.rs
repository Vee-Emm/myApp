extern crate hex_slice;
use crate::node_template::MTreeNodeSmt;
use smtree::{node_template, traits::Serializable, tree::SparseMerkleTree, utils::print_output};
type SMT<P> = SparseMerkleTree<P>;

fn main() {
    println!("Hello, world!");

    let list: Vec<MTreeNodeSmt<blake3::Hasher>> = vec![
        MTreeNodeSmt::new(vec![0; 32]),
        MTreeNodeSmt::new(vec![1; 32]),
        MTreeNodeSmt::new(vec![2; 32]),
        MTreeNodeSmt::new(vec![3; 32]),
        MTreeNodeSmt::new(vec![4; 32]),
        MTreeNodeSmt::new(vec![5; 32]),
        MTreeNodeSmt::new(vec![6; 32]),
        MTreeNodeSmt::new(vec![7; 32]),
        MTreeNodeSmt::new(vec![8; 32]),
        MTreeNodeSmt::new(vec![9; 32]),
    ];

    create(&list);

    //Validate a correct root hash for a list of leaves.This will return true
    let x = validate(
        &list,
        "0ACC9779AB6EBD977EF3B27A5E0A66E9A335F7BC7B341DD093F8E21F4F0FAC7F",
    );
    dbg!(x);

    //Validate an incorrect root hash for a list of leaves.This will return false.
    let y = validate(
        &list,
        "0ACC9779AB6EBD977EF3B27A5E0A66E9A335F7BC7C341DD093F8E21F4F0FAC7F",
    );
    dbg!(y);

    //Search for a node hash that exists in MT. This will return true.
    let a = search(
        &list,
        "251A810A1D5E96EB9A1AEA361672A61B24DEC8351F77B26A13217CE52B3BE55D",
    );
    dbg!(a);

    //Search for a node hash that does not exist in MT. This will return false.
    let b = search(
        &list,
        "351A810A1D5E96EB9A1AEA361672A61B24DEC8351F77B26A13217CE52B3BE55D",
    );
    dbg!(b);
}

//Input: All leaves in a vector,a hash as string
//Output: Returns a boolean depending on weather or not the hash belongs to the MT
fn search(list: &Vec<MTreeNodeSmt<blake3::Hasher>>, s: &str) -> bool {
    let tree = SMT::<MTreeNodeSmt<blake3::Hasher>>::new_merkle_tree(&list);
    let res = &tree.get_index_node_pairs();

    for x in res {
        let nodeHash = to_hex_string(&((x.1).get_value().serialize()));
        if s.eq(&nodeHash) {
            return true;
        }
    }

    return false;
}

//Input: All leaves in a vector.
//Output: Root hash as a string.
fn create(list: &Vec<MTreeNodeSmt<blake3::Hasher>>) -> String {
    let tree = SMT::<MTreeNodeSmt<blake3::Hasher>>::new_merkle_tree(&list);

    dbg!(to_hex_string(&tree.get_root_raw().serialize()));
    print_output(&tree);

    return to_hex_string(&tree.get_root_raw().serialize());
}

// Input :All leaves in a vector, root hash as a string.
// Output : boolean that decides this is a valid MT
fn validate(list: &Vec<MTreeNodeSmt<blake3::Hasher>>, s: &str) -> bool {
    let tree = SMT::<MTreeNodeSmt<blake3::Hasher>>::new_merkle_tree(&list);
    let root = to_hex_string(&tree.get_root_raw().serialize());

    return s.eq(&root);
}

fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.connect("")
}