extern crate hex_slice;
use crate::node_template::MTreeNodeSmt;
use sha2::Digest;
use sha2::Sha256;
use smtree::{node_template, traits::Serializable, tree::SparseMerkleTree, utils::print_output};
type SMT<P> = SparseMerkleTree<P>;
use smtree::index::{TreeIndex};
use smtree::proof::{MerkleProof};
use smtree::traits::InclusionProvable;
use std::str;


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
        let node_hash = hex::encode(&((x.1).get_value().serialize()));
        if s.eq(&node_hash) {
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

fn _verify_proof() -> bool {
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
fn hash_sanity() {
    let secret = b"leaf0leaf0leaf0leaf0leaf0leaf0le";
    let mut a = sha2::Sha256::new();
    a.update(secret);
    let sha_hash = &hex::encode(a.finalize());
    dbg!(sha_hash);
    assert_eq!("96f1ce1008b5c50024edbd0652c0e3b6213d38b8ee55c9b44a34cb95e5d05892", sha_hash);

    let mut b = blake2::Blake2b::new();
    b.update(secret);
    let blake_hash = &hex::encode(b.finalize());
    dbg!(blake_hash);
    assert_eq!("852c54b76e614e31ab2246ac8994a5ed38a4101940495a6478551a25ea0b7496bfce1a848c1679e1ee8a256ebd2c7bb46b98ab9752f94c28fc9ee4140037d7e5", blake_hash)
}

#[test]
// run with " cargo test -- --nocapture " to see print outs on tests that pass.
fn correct_hasher() {
    
    let leaf0 = MTreeNodeSmt::<Sha256>::new(b"leaf0leaf0leaf0leaf0leaf0leaf0le".to_vec());    
    let leaf0alt = MTreeNodeSmt::<blake2::Blake2b>::new(b"leaf0leaf0leaf0leaf0leaf0leaf0le".to_vec());

    // NOTE: weird, these outputs are neither the Hash of Sha2 nor a blake2b Hash
    dbg!(&hex::encode(leaf0.serialize()));  // not a sha hash
    dbg!(&hex::encode(leaf0alt.serialize())); // not a blake hash

    // NOTE: weird, these hashes are the same, with blake or sha
    assert_ne!(&hex::encode(leaf0.serialize()), &hex::encode(leaf0alt.serialize()));
    assert_ne!(leaf0.serialize(), leaf0alt.serialize());
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

