use crate::ports::trees::{CommitmentTree, HistoricRootTree, NullifierTree};
use ark_bn254::Fr as Fr254;
use ark_ff::Zero;
use lib::merkle_trees::trees::MutableTree;
use mongodb::Client;

pub(super) async fn ensure_commitment_tree_initialized(client: &Client) {
    if <mongodb::Client as CommitmentTree<Fr254>>::get_root(client)
        .await
        .is_ok()
    {
        return;
    }

    <mongodb::Client as CommitmentTree<Fr254>>::new_commitment_tree(client, 29, 3)
        .await
        .expect("Could not create commitment tree");
}

pub(super) async fn ensure_nullifier_tree_initialized(client: &Client) {
    if <mongodb::Client as MutableTree<Fr254>>::get_root(
        client,
        <mongodb::Client as NullifierTree<Fr254>>::TREE_NAME,
    )
    .await
    .is_ok()
    {
        return;
    }

    <mongodb::Client as NullifierTree<Fr254>>::new_nullifier_tree(client, 29, 3)
        .await
        .expect("Could not create nullifier tree");
}

pub(super) async fn ensure_historic_root_tree_initialized(client: &Client) {
    let zero_leaf = Fr254::from(0u8);
    let root = match <mongodb::Client as MutableTree<Fr254>>::get_root(
        client,
        <mongodb::Client as HistoricRootTree<Fr254>>::TREE_NAME,
    )
    .await
    {
        Ok(root) => root,
        Err(_) => {
            <mongodb::Client as HistoricRootTree<Fr254>>::new_historic_root_tree(client, 32)
                .await
                .expect("Could not create historic root tree");
            Fr254::zero()
        }
    };

    let has_zero_leaf =
        <mongodb::Client as HistoricRootTree<Fr254>>::is_historic_root(client, &zero_leaf)
            .await
            .expect("Could not query historic root tree");

    if has_zero_leaf {
        return;
    }

    if !root.is_zero() {
        panic!("Historic root tree exists without zero leaf in a non-empty state");
    }

    <Client as HistoricRootTree<Fr254>>::append_historic_commitment_root(client, &zero_leaf, true)
        .await
        .expect("Couldn't insert zero leaf into the historic root tree");
}

pub(super) async fn ensure_proposer_db_initialized(client: &Client) {
    ensure_commitment_tree_initialized(client).await;
    ensure_historic_root_tree_initialized(client).await;
    ensure_nullifier_tree_initialized(client).await;
    crate::driven::db::mongo_db::ensure_deposit_indexes(client)
        .await
        .expect("Could not create deposit indexes");
    crate::driven::db::mongo_db::ensure_transfer_receipt_indexes(client)
        .await
        .expect("Could not create transfer receipt indexes");
}
