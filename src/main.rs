extern crate hex_slice;
use crate::node_template::MTreeNodeSmt;
use digest::generic_array::typenum::private::IsEqualPrivate;
use sha2::Digest;
use sha2::Sha256;
use smtree::node_template::HashNodeSmt;
use smtree::{node_template, traits::Serializable, tree::SparseMerkleTree, utils::print_output};
type SMT<P> = SparseMerkleTree<P>;
use smtree::index::TreeIndex;
use smtree::proof::MerkleProof;
use smtree::traits::InclusionProvable;
use std::ops::Index;
use std::str;

fn main() {
    println!("Hello, world!");
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

    let proof =
        MerkleProof::<MTreeNodeSmt<sha2::Sha256>>::generate_inclusion_proof(&tree, &index_list)
            .unwrap();
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
    assert_eq!(
        "96f1ce1008b5c50024edbd0652c0e3b6213d38b8ee55c9b44a34cb95e5d05892",
        sha_hash
    );

    let mut b = blake2::Blake2b::new();
    b.update(secret);
    let blake_hash = &hex::encode(b.finalize());
    dbg!(blake_hash);
    assert_eq!("852c54b76e614e31ab2246ac8994a5ed38a4101940495a6478551a25ea0b7496bfce1a848c1679e1ee8a256ebd2c7bb46b98ab9752f94c28fc9ee4140037d7e5", blake_hash)
}

// run with " cargo test -- --nocapture " to see print outs on tests that pass.
#[test]
fn correct_hasher() {
    let sha = (sha2::Sha256::digest(b"leaf0leaf0leaf0leaf0leaf0leaf0le")).to_vec();
    let blk = (blake2::Blake2b::digest(b"leaf0leaf0leaf0leaf0leaf0leaf0le")).to_vec();

    let leaf0 = HashNodeSmt::<sha2::Sha256>::new(sha);
    let leaf0alt = HashNodeSmt::<blake2::Blake2b>::new(blk);

    dbg!(&hex::encode(leaf0.serialize()));
    dbg!(&hex::encode(leaf0alt.serialize()));

    assert_ne!(
        &hex::encode(leaf0.serialize()),
        &hex::encode(leaf0alt.serialize())
    );
    assert_ne!(leaf0.serialize(), leaf0alt.serialize());
}

// run with " cargo test -- --nocapture " to see print outs on tests that pass.
#[test]
fn test_merkle_tree() {
    let leaf0_sha = (sha2::Sha256::digest(b"leaf0leaf0leaf0leaf0leaf0leaf0le")).to_vec();
    let leaf1_sha = (sha2::Sha256::digest(b"leaf1leaf1leaf1leaf1leaf1leaf1le")).to_vec();
    let leaf2_sha = (sha2::Sha256::digest(b"leaf2leaf2leaf2leaf2leaf2leaf2le")).to_vec();
    let leaf3_sha = (sha2::Sha256::digest(b"leaf3leaf3leaf3leaf3leaf3leaf3le")).to_vec();

    let leaf0 = MTreeNodeSmt::new(leaf0_sha);
    let leaf1 = MTreeNodeSmt::new(leaf1_sha);
    let leaf2 = MTreeNodeSmt::new(leaf2_sha);
    let leaf3 = MTreeNodeSmt::new(leaf3_sha);

    dbg!(&hex::encode(leaf0.serialize()));
    dbg!(&hex::encode(leaf1.serialize()));
    dbg!(&hex::encode(leaf2.serialize()));
    dbg!(&hex::encode(leaf3.serialize()));

    let list: Vec<MTreeNodeSmt<sha2::Sha256>> =
        vec![leaf0.clone(), leaf1.clone(), leaf2.clone(), leaf3.clone()];
    let tree = SMT::<MTreeNodeSmt<sha2::Sha256>>::new_merkle_tree(&list); 
    //new_merkle_tree claims to creates the tree with the intermeidary & rootnodes . The method is part of tree.rs

    assert_eq!(tree.get_height(), 2); // starting from zero
    assert_eq!(tree.get_paddings().len(), 0);

    print_output(&tree);
    dbg!(&hex::encode(tree.get_root().serialize())); //---------This root is incorrect--------------

    let a = tree.get_index_node_pairs();
    for x in a {
        //dbg!(x);
        dbg!(&hex::encode(x.1.get_value().serialize())); //--------The root and intermediary nodes are incorrect------.
    }
}
