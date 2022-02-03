extern crate hex_slice;
use crate::node_template::MTreeNodeSmt;
use smtree::{node_template, traits::Serializable, tree::SparseMerkleTree, utils::print_output};
type SMT<P> = SparseMerkleTree<P>;
use smtree::index::{TreeIndex};
use smtree::proof::{MerkleProof};
use smtree::traits::{InclusionProvable, ProofExtractable};
use std::str;
use hex::{encode,decode};



fn main() {
    println!("Hello, world!");

    let list: Vec<MTreeNodeSmt<sha2::Sha256>> = vec![
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

    test_merkle_tree();

}

//Input: All leaves in a vector,a hash as string
//Output: Returns a boolean depending on weather or not the hash belongs to the MT
fn search(list: &Vec<MTreeNodeSmt<sha2::Sha256>>, s: &str) -> bool {
    let tree = SMT::<MTreeNodeSmt<sha2::Sha256>>::new_merkle_tree(&list);
    let res = &tree.get_index_node_pairs();

    for x in res {
        let nodeHash = hex::encode(&((x.1).get_value().serialize()));
        if s.eq(&nodeHash) {
            return true;
        }
    }

    return false;
}

//Input: All leaves in a vector.
//Output: Root hash as a string.
fn create(list: &Vec<MTreeNodeSmt<sha2::Sha256>>) -> String {
    let tree = SMT::<MTreeNodeSmt<sha2::Sha256>>::new_merkle_tree(&list);

    dbg!(hex::encode(&tree.get_root_raw().serialize()));
    print_output(&tree);

    return hex::encode(&tree.get_root_raw().serialize());
}

// Input :All leaves in a vector, root hash as a string.
// Output : boolean that decides this is a valid MT
fn validate(list: &Vec<MTreeNodeSmt<sha2::Sha256>>, s: &str) -> bool {
    let tree = SMT::<MTreeNodeSmt<sha2::Sha256>>::new_merkle_tree(&list);
    let root = hex::encode(&tree.get_root_raw().serialize());

    return s.eq(&root);
}

fn verify_proof() -> bool{
    let example_leaf = MTreeNodeSmt::new(vec![0; 32]);
    let list: Vec<MTreeNodeSmt<sha2::Sha256>> = vec![example_leaf.clone(); 5];
    let tree = SMT::<MTreeNodeSmt<sha2::Sha256>>::new_merkle_tree(&list);
    assert_eq!(tree.get_height(), 3); // starting from zero
    assert_eq!(tree.get_paddings().len(), 2);

    // Add a single index in the proof generation (we prove one element only)
    let index_list = vec![TreeIndex::from_u64(tree.get_height(), 2)];

    let proof =  MerkleProof::<MTreeNodeSmt<sha2::Sha256>>::generate_inclusion_proof(&tree, &index_list).unwrap();
    
    dbg!(hex::encode(&(proof.serialize())));
    dbg!(&(proof.serialize()));
     
    return proof.verify(&example_leaf, &tree.get_root());
}

#[test]
// run with " cargo test -- --nocapture " to see print outs on tests that pass.
fn test_merkle_tree() {
    let leaf0 = MTreeNodeSmt::new("leaf0leaf0leaf0leaf0leaf0leaf0le".as_bytes().to_vec());
    let leaf1 = MTreeNodeSmt::new("leaf1leaf1leaf1leaf1leaf1leaf1le".as_bytes().to_vec());
    let leaf2 = MTreeNodeSmt::new("leaf2leaf2leaf2leaf2leaf2leaf2le".as_bytes().to_vec());
    let leaf3 = MTreeNodeSmt::new("leaf3leaf3leaf3leaf3leaf3leaf3le".as_bytes().to_vec());


    let list: Vec<MTreeNodeSmt<sha2::Sha256>> = vec![leaf0.clone(),leaf1.clone(),leaf2.clone(),leaf3.clone()];
    let tree = SMT::<MTreeNodeSmt<sha2::Sha256>>::new_merkle_tree(&list);


    assert_eq!(tree.get_height(), 2); // starting from zero
    assert_eq!(tree.get_paddings().len(), 0);

    print_output(&tree);


    // Add a single index in the proof generation (we prove one element at index 0 only)
    let index = vec![TreeIndex::from_u64(tree.get_height(), 0)];


    let proof =
        MerkleProof::<MTreeNodeSmt<sha2::Sha256>>::generate_inclusion_proof(&tree, &index)
            .unwrap();
    assert_eq!(proof.verify(&leaf0, &tree.get_root()), true);


    let serialized_proof = proof.serialize();
    let deserialized_proof = MerkleProof::<MTreeNodeSmt<sha2::Sha256>>::deserialize(&serialized_proof).unwrap();
    assert_eq!(serialized_proof, deserialized_proof.serialize());
    assert_eq!(
        deserialized_proof.verify(&leaf0, &tree.get_root()),
        true
    );


    let y = tree.get_root().serialize();
    // let z = y.as_slice();
    // dbg!(&z);
    dbg!(&hex::encode(y));
    // dbg!(&tree);

}

