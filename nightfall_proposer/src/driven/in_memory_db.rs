use std::collections::HashMap;

use crate::ports::db::{NullifierDB, NullifierDBEntry};
use ark_bn254::Fr as Fr254;
use ark_ff::Zero;

impl NullifierDB<Fr254> for HashMap<Fr254, NullifierDBEntry<Fr254>> {
    fn new() -> Self {
        let mut out = HashMap::<Fr254, NullifierDBEntry<Fr254>>::new();
        out.insert(Fr254::zero(), NullifierDBEntry::<Fr254>::default());
        out
    }

    fn store_nullifier(&mut self, nullifier: Fr254) -> Option<()> {
        // If the new nullifier is already in the db then we shouldn't store it.
        if self.get(&nullifier).is_some() {
            return None;
        }

        // If the new nullifier is not in the db then we should update the next value of its low nullifier.
        let low_nullifier = *self.get_low_nullifier(&nullifier)?;
        let index = self.len() as u32;
        let entry = NullifierDBEntry::<Fr254>::new(
            nullifier,
            index,
            low_nullifier.next_index,
            low_nullifier.next_value,
        );
        let val = self.insert(nullifier, entry);
        // If we somehow got to here and the value was already in the db then we should return `None`.
        if val.is_some() {
            return None;
        }
        self.update_nullifier(low_nullifier.value, Fr254::from(index), nullifier)?;

        Some(())
    }

    fn get_nullifier(
        &self,
        value: Option<Fr254>,
        next_value: Option<Fr254>,
    ) -> Option<&NullifierDBEntry<Fr254>> {
        match (value, next_value) {
            (Some(value), None) => self.get(&value),
            (None, Some(next_value)) => self.values().find(|v| v.next_value == next_value),

            (Some(value), Some(next_value)) => self.get(&value).and_then(|v| {
                if v.next_value == next_value {
                    Some(v)
                } else {
                    None
                }
            }),
            (None, None) => None,
        }
    }

    fn get_low_nullifier(&self, nullifier_value: &Fr254) -> Option<&NullifierDBEntry<Fr254>> {
        let low_nullifier = self
            .keys()
            .filter(|k| *k < nullifier_value)
            .max()
            .copied()?;
        let next_value = self.get(&low_nullifier)?;
        if *nullifier_value < next_value.next_value || next_value.next_value == Fr254::zero() {
            Some(next_value)
        } else {
            None
        }
    }

    fn update_nullifier(
        &mut self,
        nullifier: Fr254,
        new_next_index: Fr254,
        new_next_value: Fr254,
    ) -> Option<()> {
        self.get_mut(&nullifier).map(|v| {
            *v = NullifierDBEntry::<Fr254>::new(nullifier, v.index, new_next_index, new_next_value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_nullifier() {
        let mut db = <HashMap<Fr254, NullifierDBEntry<Fr254>> as NullifierDB<Fr254>>::new();
        let nullifier = Fr254::from(1u8);
        let next_value = Fr254::from(0u8);
        let result = db.store_nullifier(nullifier);
        assert_eq!(result, Some(()));
        assert_eq!(db.len(), 2);
        let expected_entry =
            NullifierDBEntry::<Fr254>::new(nullifier, 1, Fr254::zero(), next_value);
        assert_eq!(db.get(&nullifier), Some(&expected_entry));

        // We should check that the low nullifier has been updated
        let low_nullifier_next_value = db.get(&Fr254::zero());
        let expected_entry =
            NullifierDBEntry::<Fr254>::new(Fr254::zero(), 0, Fr254::from(1u8), Fr254::from(1u8));
        assert_eq!(low_nullifier_next_value, Some(&expected_entry));

        // Storing the same nullifier again should fail
        let result = db.store_nullifier(nullifier);
        assert_eq!(result, None);

        // Now if we store two more nullifiers we should see that the low nullifier is updated
        let nullifier_2 = Fr254::from(2u8);
        let nullifier_3 = Fr254::from(243u8);
        let res = db.store_nullifier(nullifier_3);
        assert_eq!(res, Some(()));
        let query = db.get(&nullifier);
        let expected_entry =
            NullifierDBEntry::<Fr254>::new(nullifier, 1, Fr254::from(2u8), nullifier_3);
        assert_eq!(query, Some(&expected_entry));

        let res = db.store_nullifier(nullifier_2);
        assert_eq!(res, Some(()));
        let query = db.get(&nullifier);
        let expected_entry =
            NullifierDBEntry::<Fr254>::new(nullifier, 1, Fr254::from(3u8), nullifier_2);
        assert_eq!(query, Some(&expected_entry));
    }

    #[test]
    fn test_get_nullifier() {
        let mut db = <HashMap<Fr254, NullifierDBEntry<Fr254>> as NullifierDB<Fr254>>::new();
        let nullifier = Fr254::from(1u8);
        let nullifier_2 = Fr254::from(3u8);
        let result = db.store_nullifier(nullifier);
        assert_eq!(result, Some(()));
        let result = db.store_nullifier(nullifier_2);
        assert_eq!(result, Some(()));

        // Get the next value of a nullifier
        let result = db.get_nullifier(Some(nullifier), None);
        assert_eq!(result.unwrap().value, nullifier);

        // Get the nullifier of a next value
        let result = db.get_nullifier(None, Some(nullifier_2));
        assert_eq!(result.unwrap().value, nullifier);
    }

    #[test]
    fn test_low_nullifier() {
        let mut db = <HashMap<Fr254, NullifierDBEntry<Fr254>> as NullifierDB<Fr254>>::new();
        let nullifiers = (1..100)
            .map(|i| Fr254::from(i * 10))
            .collect::<Vec<Fr254>>();
        for nullifier in nullifiers.iter() {
            let result = db.store_nullifier(*nullifier);
            assert_eq!(result, Some(()));
        }

        let search_nullifiers = (0..100)
            .map(|i| Fr254::from(i * 10 + 5))
            .collect::<Vec<Fr254>>();

        for (index, search_nullifier) in search_nullifiers.iter().take(99).enumerate() {
            let low_nullifier = db.get_low_nullifier(search_nullifier);
            let expected_entry = NullifierDBEntry::<Fr254>::new(
                *search_nullifier - Fr254::from(5u8),
                index as u32,
                Fr254::from(index as u32 + 1),
                *search_nullifier + Fr254::from(5u8),
            );
            assert_eq!(low_nullifier, Some(&expected_entry));
        }

        let low_nullifier = db.get_low_nullifier(&search_nullifiers[99]);
        let expected_entry = NullifierDBEntry::<Fr254>::new(
            search_nullifiers[99] - Fr254::from(5u8),
            99u32,
            Fr254::zero(),
            Fr254::zero(),
        );
        assert_eq!(low_nullifier, Some(&expected_entry));
    }
}
