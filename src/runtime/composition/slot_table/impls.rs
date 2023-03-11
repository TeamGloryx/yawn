use super::constants::*;
use crate::util::macros::{map_of, yop};
use crate::util::magic::{GInto, GOption, Thing};
use crate::util::numbers::Step;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub(super) trait GVecExt<T: Clone>:
    Index<usize, Output = T> + Sized + IntoIterator<Item = T>
{
    fn at(&self, index: impl GInto<usize>) -> T {
        self.index(index.g_into()).clone()
    }
    fn set(&mut self, index: impl GInto<usize>, value: T);
    fn set_from(&mut self, sets: HashMap<impl GInto<usize>, T>) {
        sets.into_iter()
            .for_each(|(idx, val)| self.set(idx.g_into(), val))
    }
    fn len(&self) -> i32;
    fn slice(&self, range: impl Iterator<Item = impl GInto<usize>>) -> Self;
    fn from_iter(iter: impl Iterator<Item = T>) -> Self;
}

pub(super) trait GroupTable: GVecExt<i32> {
    fn group_info(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET)
    }
    fn is_node(&self, addr: i32) -> bool {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & NODE_BIT_MASK != 0
    }
    fn node_index(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE + DATA_ANCHOR_OFFSET)
    }
    fn has_obj_key(&self, addr: i32) -> bool {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & OBJECT_KEY_MASK != 0
    }
    fn obj_key_idx(&self, addr: i32) -> i32 {
        let slot = addr * GROUP_FIELDS_SIZE;

        self.at(slot + DATA_ANCHOR_OFFSET)
            + count_one_bits(self.at(slot + GROUP_INFO_OFFSET) >> (OBJECT_KEY_SHIFT + 1))
    }
    fn has_aux(&self, addr: i32) -> bool {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & AUX_MASK != 0
    }
    fn add_aux(&mut self, addr: i32) {
        let array_idx = addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET;

        self.set(array_idx, self.at(array_idx) | AUX_MASK)
    }

    fn aux_index(&self, addr: i32) -> i32 {
        let slot = addr * GROUP_FIELDS_SIZE;
        let size = self.len() as i32;
        if slot >= size {
            size
        } else {
            self.at(slot + DATA_ANCHOR_OFFSET)
                + count_one_bits(self.at(slot + GROUP_INFO_OFFSET) >> (AUX_SHIFT + 1))
        }
    }
    fn has_mark(&self, addr: i32) -> bool {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & MARK_MASK != 0
    }
    fn update_mark(&mut self, addr: i32, value: bool) {
        let array_idx = addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET;

        if value {
            self.set(array_idx, self.at(array_idx) | MARK_MASK)
        } else {
            self.set(array_idx, self.at(array_idx) & !MARK_MASK)
        }
    }
    fn contains_mark(&self, addr: i32) -> bool {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & CONTAINS_MARK_MASK != 0
    }
    fn update_contains_mark(&mut self, addr: i32, value: bool) {
        let array_idx = addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET;
        if value {
            self.set(array_idx, self.at(array_idx) | CONTAINS_MARK_MASK)
        } else {
            self.set(array_idx, self.at(array_idx) & !CONTAINS_MARK_MASK)
        }
    }

    fn contains_any_mark(&self, addr: i32) -> bool {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & (CONTAINS_MARK_MASK | MARK_MASK)
            != 0
    }

    fn slot_anchor(&self, addr: i32) -> i32 {
        let slot = addr * GROUP_FIELDS_SIZE;

        self.at(slot + DATA_ANCHOR_OFFSET)
            + count_one_bits(self.at(slot + GROUP_INFO_OFFSET) >> SLOTS_SHIFT)
    }

    fn group_sizes(&self, len: GOption<i32>) -> Self {
        self.slice((SIZE_OFFSET..len.g_into().unwrap_or(self.len())).step(GROUP_FIELDS_SIZE))
    }

    fn data_anchor(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE + DATA_ANCHOR_OFFSET)
    }

    fn update_data_anchor(&mut self, addr: i32, anchor: i32) {
        self.set(addr * GROUP_FIELDS_SIZE + DATA_ANCHOR_OFFSET, anchor)
    }

    fn data_anchors(&self, len: GOption<i32>) -> Self {
        self.slice((DATA_ANCHOR_OFFSET..len.g_into().unwrap_or(self.len())).step(GROUP_FIELDS_SIZE))
    }

    fn init_group(
        &mut self,
        addr: i32,
        key: i32,
        is_node: bool,
        has_data_key: bool,
        has_data: bool,
        parent_anchor: i32,
        data_anchor: i32,
    ) {
        let node_bit = yop!(is_node => NODE_BIT_MASK; 0);
        let data_key_bit = yop!(has_data_key => OBJECT_KEY_SHIFT; 0);
        let data_bit = yop!(has_data => AUX_MASK; 0);
        let arr_idx = addr * GROUP_FIELDS_SIZE;

        self.set_from(map_of![
            arr_idx + KEY_OFFSET => key,
            arr_idx + GROUP_INFO_OFFSET => node_bit | data_key_bit | data_bit,
            arr_idx + PARENT_ANCHOR_OFFSET => parent_anchor,
            arr_idx + SIZE_OFFSET => 0,
            arr_idx + DATA_ANCHOR_OFFSET => data_anchor
        ])
    }

    fn update_group_key(&mut self, addr: i32, key: i32) {
        self.set(addr * GROUP_FIELDS_SIZE + KEY_OFFSET, key)
    }
}

pub(super) trait KeyTable: GVecExt<i32> {
    fn key(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE)
    }
    fn keys(&self, len: GOption<i32>) -> Self {
        self.slice((KEY_OFFSET..len.g_into().unwrap_or(self.len())).step(GROUP_FIELDS_SIZE))
    }

    fn node_count(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & NODE_COUNT_MASK
    }

    fn update_node_count(&mut self, addr: i32, value: i32) {
        assert!(value >= 0 && value < NODE_COUNT_MASK);

        self.set(
            addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET,
            (self.at(addr * GROUP_FIELDS_SIZE + GROUP_INFO_OFFSET) & !NODE_COUNT_MASK) | value,
        )
    }

    fn node_counts(&self, len: GOption<i32>) -> Self {
        Self::from_iter(
            self.slice(
                (GROUP_INFO_OFFSET..len.g_into().unwrap_or(self.len())).step(GROUP_FIELDS_SIZE),
            )
            .into_iter()
            .map(|it| it & NODE_COUNT_MASK),
        )
    }

    fn parent_anchor(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE + PARENT_ANCHOR_OFFSET)
    }
    fn update_parent_anchor(&mut self, addr: i32, value: i32) {
        self.set(addr * GROUP_FIELDS_SIZE + PARENT_ANCHOR_OFFSET, value)
    }
    fn parent_anchors(&self, len: GOption<i32>) -> Self {
        self.slice(
            (PARENT_ANCHOR_OFFSET..len.g_into().unwrap_or(self.len())).step(GROUP_FIELDS_SIZE),
        )
    }

    fn group_size(&self, addr: i32) -> i32 {
        self.at(addr * GROUP_FIELDS_SIZE + SIZE_OFFSET)
    }
    fn update_group_size(&mut self, addr: i32, value: i32) {
        assert!(value >= 0);

        self.set(addr * GROUP_FIELDS_SIZE + SIZE_OFFSET, value)
    }
}

// -----------------------------------------------------------------------------------------------------\\
// **!!**                                       NEVER                                             **!!**\\
// **!!**                                       NEVER                                             **!!**\\
//                                                                                                      \\
// !!!!!!                      DO NOT EVER FUCKING TOUCH THIS PIECE OF CRAP                       !!!!!!\\
//                                                                                                      \\
// **!!**                                       NEVER                                             **!!**\\
// **!!**                                       NEVER                                             **!!**\\
// -----------------------------------------------------------------------------------------------------\\
impl<T: Clone + Thing> GVecExt<T> for Vec<T> {
    fn set(&mut self, index: impl GInto<usize>, value: i32) {
        *self.index_mut(index.g_into()) = value
    }
    fn len(&self) -> i32 {
        Vec::len(self) as i32
    }
    fn slice(&self, range: impl Iterator<Item = impl GInto<usize>>) -> Self {
        let mut tmp = Vec::<i32>::new();
        range
            .map(|i| self.get(i.g_into()) as Option<&T>)
            .for_each(|Some(i)| tmp.push(i.clone()));

        tmp
    }
    fn from_iter(iter: Self::IntoIter) -> Self {
        iter.collect()
    }
}
impl GroupTable for Vec<i32> {}
impl KeyTable for Vec<i32> {}
