use crate::{
    domain::entities::{ClientTransactionWithMetaData, DepositDatawithFee, HistoricRoot},
    ports::db::{HistoricRootsDB, TransactionsDB},
};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, PrimeField, Zero};
use mongodb::bson::doc;
use nightfall_client::{
    domain::{entities::ClientTransaction, error::ConversionError},
    ports::proof::Proof,
};
use serde::{Deserialize, Serialize};

const DB: &str = "nightfall";
const COLLECTION: &str = "ClientTransactions";
const DEPOSIT_COLLECTION: &str = "Deposits";

#[async_trait::async_trait]
impl<'a, P> TransactionsDB<'a, P> for mongodb::Client
where
    P: Proof,
{
    async fn store_transaction(
        &mut self,
        transaction: ClientTransactionWithMetaData<P>,
    ) -> Option<()> {
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .insert_one(transaction)
            .await
            .ok()?;
        Some(())
    }

    async fn get_transaction(
        &mut self,
        key: &'a [u32],
    ) -> Option<ClientTransactionWithMetaData<P>> {
        let filter = doc! {"hash": key};
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find_one(filter)
            .await
            .ok()?
    }

    async fn get_all_transactions(
        &mut self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>> {
        let mut cursor: mongodb::Cursor<ClientTransactionWithMetaData<P>> = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find(doc! {})
            .await
            .unwrap();
        let mut result: Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)> = Vec::new();
        while cursor.advance().await.ok()? {
            let v: ClientTransactionWithMetaData<P> = cursor.deserialize_current().ok()?;
            result.push((v.hash.clone(), v));
        }
        if result.is_empty() {
            return None;
        };
        Some(result)
    }

    // add in all the remaining trait items
    async fn get_all_mempool_transactions(
        &mut self,
    ) -> Option<Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)>> {
        let filter = doc! {"in_mempool": true};
        let mut cursor: mongodb::Cursor<ClientTransactionWithMetaData<P>> = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find(filter)
            .await
            .expect("Database error"); // we can't really proceed at this point
        let mut result: Vec<(Vec<u32>, ClientTransactionWithMetaData<P>)> = Vec::new();
        while cursor.advance().await.ok()? {
            let v: ClientTransactionWithMetaData<P> = cursor.deserialize_current().ok()?;
            result.push((v.hash.clone(), v));
        }
        if result.is_empty() {
            return None;
        };
        Some(result)
    }

    async fn is_transaction_in_mempool(&mut self, k: &'a [u32]) -> bool {
        let filter = doc! {"hash": k};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find_one(filter)
            .await
            .expect("Database error"); // we can't really proceed at this point
        match result {
            Some(_v) => true,
            None => false,
        }
    }

    async fn check_nullifier(&self, _nullifier: Fr254) -> bool {
        todo!()
    }

    async fn check_commitment(&self, _commitment: Fr254) -> bool {
        todo!()
    }

    async fn update_commitment<M>(&mut self, mutator: M, key: &'a [u32]) -> Option<()>
    where
        M: Fn(&ClientTransactionWithMetaData<P>) -> ClientTransactionWithMetaData<P> + Send,
    {
        let filter = doc! {"hash": key};
        let old_tx = self.get_transaction(key).await?;
        let new_tx = mutator(&old_tx);

        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .replace_one(filter, new_tx)
            .await
            .expect("Database error"); // we can't really proceed at this point
        Some(())
    }

    async fn set_in_mempool(
        &mut self,
        txs: &[ClientTransactionWithMetaData<P>],
        in_mempool: bool,
    ) -> Option<u64> {
        let k: Vec<_> = txs.iter().map(|t| &t.hash).collect();
        let filter = doc! {"hash": { "$in": k }};
        let update = doc! {"$set": { "in_mempool": in_mempool }};
        let result = self
            .database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .update_many(filter, update)
            .await
            .expect("Database error"); // we can't really proceed at this point
        Some(result.modified_count)
    }

    async fn find_transaction(
        &self,
        v: &ClientTransaction<P>,
    ) -> Option<ClientTransactionWithMetaData<P>> {
        // we'll compute the hash of the transaction and then look it up in the database
        let hash = v.hash().ok()?;
        let filter = doc! {"hash": hash};
        self.database(DB)
            .collection::<ClientTransactionWithMetaData<P>>(COLLECTION)
            .find_one(filter)
            .await
            .expect("Database error") // we can't really proceed at this point
    }

    async fn find_deposit(&self, v: &DepositDatawithFee) -> Option<DepositDatawithFee> {
        // we'll compute the hash of the transaction and then look it up in the database
        let hash = v.hash().ok()?;
        let filter = doc! {"hash": hash};
        self.database(DB)
            .collection::<DepositDatawithFee>(COLLECTION)
            .find_one(filter)
            .await
            .expect("Database error") // we can't really proceed at this point
    }

    // Store unused deposits in the mempool
    async fn set_mempool_deposits(&mut self, deposits: Vec<DepositDatawithFee>) -> Option<u64> {
        if deposits.is_empty() {
            return Some(0);
        }

        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);

        // Directly insert Vec<DepositInfo> instead of converting to Document
        let result = collection.insert_many(deposits).await.ok()?;

        Some(result.inserted_ids.len() as u64)
    }

    // Retrieve deposits from the mempool
    async fn get_mempool_deposits(&mut self) -> Option<Vec<DepositDatawithFee>> {
        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);
        let mut cursor = collection.find(doc! {}).await.ok()?;

        let mut result: Vec<DepositDatawithFee> = Vec::new();
        while cursor.advance().await.ok()? {
            let deposit: DepositDatawithFee = cursor.deserialize_current().ok()?;
            result.push(deposit);
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    // Remove used deposits from the mempool
    async fn remove_mempool_deposits(
        &mut self,
        used_deposits: Vec<Vec<DepositDatawithFee>>,
    ) -> Option<u64> {
        let used_deposits: Vec<DepositDatawithFee> = used_deposits.into_iter().flatten().collect();
        if used_deposits.is_empty() {
            return Some(0);
        }

        let collection = self
            .database(DB)
            .collection::<DepositDatawithFee>(DEPOSIT_COLLECTION);

        // Fetch all documents in the collection
        let delete_conditions: Vec<_> = used_deposits
            .iter()
            .map(|d| {
                doc! {
                    "deposit_data.secret_hash": hex::encode(d.deposit_data.secret_hash.into_bigint().to_bytes_le()),
                    "deposit_data.nf_slot_id": hex::encode(d.deposit_data.nf_slot_id.into_bigint().to_bytes_le()),
                }
            })
            .collect();
        let filter = doc! {
            "$or": delete_conditions
        };
        // Delete matching documents
        let result = collection.delete_many(filter).await.ok()?;
        Some(result.deleted_count)
    }
}

#[async_trait::async_trait]
impl HistoricRootsDB for mongodb::Client {
    async fn store_historic_root(&mut self, historic_root: &HistoricRoot) -> Option<()> {
        let historic_root_entry = HistoricRootEntry::from(historic_root);
        self.database(DB)
            .collection::<HistoricRootEntry>("historic_roots")
            .insert_one(historic_root_entry)
            .await
            .expect("Database error"); // we can't really proceed at this point
        Some(())
    }
    async fn get_historic_root(&mut self, historic_root_hash: &Fr254) -> Option<HistoricRoot> {
        let filter = doc! {"historic_root_hash": historic_root_hash.to_string()};
        let historic_root = self
            .database(DB)
            .collection::<HistoricRootEntry>("historic_roots")
            .find_one(filter)
            .await
            .expect("Database error"); // we can't really proceed at this point
        historic_root.map(|historic_root| {
            historic_root
                .try_into()
                .expect("Conversion should always succeed")
        })
    }
}

// we need to store a slightly different struct because we can't easily turn
// HistoricRoot into a bson object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct HistoricRootEntry {
    historic_root_hash: String,
    index: u32,
}

impl From<&HistoricRoot> for HistoricRootEntry {
    fn from(historic_root: &HistoricRoot) -> Self {
        Self {
            historic_root_hash: historic_root.0.to_string(),
            index: historic_root.1,
        }
    }
}

impl TryFrom<HistoricRootEntry> for HistoricRoot {
    type Error = ConversionError;

    fn try_from(historic_root_entry: HistoricRootEntry) -> Result<Self, Self::Error> {
        // a value of Fr254::zero() gets converted to an empty string, rather than "0"
        // this then fails to parse, so we need to handle this case
        // ...

        if historic_root_entry.historic_root_hash.is_empty() {
            return Ok(Self(Fr254::zero(), 0));
        }
        Ok(Self(
            historic_root_entry
                .historic_root_hash
                .parse::<Fr254>()
                .map_err(|_| ConversionError::ParseFailed)?,
            historic_root_entry.index,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fr as Fr254;
    use ark_std::UniformRand;
    #[test]
    fn test_historic_root_type_conversion() {
        let rng = &mut ark_std::test_rng();
        let historic_root = HistoricRoot(Fr254::rand(rng), u32::rand(rng));
        let historic_root_entry = HistoricRootEntry::from(&historic_root);
        let historic_root_2 = HistoricRoot::try_from(historic_root_entry).unwrap();
        assert_eq!(historic_root, historic_root_2);
    }
}
