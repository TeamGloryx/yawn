use std::default::default;
use super::impls::*;

pub struct Slot {}

pub(crate) struct Anchor {
    pub(crate) location: i32,
}

impl Anchor {
    pub(crate) fn valid(&self) -> bool {
        self.location != i32::MIN
    }
}

pub(crate) struct SlotTable {
    groups: Vec<i32>,
    groups_size: i32,
    slots: Vec<Option<Slot>>,
    slots_size: i32,
    readers: i32,
    writer: bool,
    version: i32,
    anchors: Vec<Anchor>,
}

impl SlotTable {
    pub(crate) fn new() -> Self {
        SlotTable {
            groups: default(),
            groups_size: default(),
            slots: default(),
            slots_size: default(),
            readers: default(),
            writer: default(),
            version: default(),
            anchors: default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.groups_size == 0
    }

    pub fn read<T, F: Fn(SlotReader) -> T>(block: F) -> T {}
}

pub(crate) struct SlotReader<'a> {
    table: &'a SlotTable,
    groups: &'a Vec<i32>,
    groups_size: i32,
    slots: &'a Vec<Option<Slot>>,
    slots_size: i32,
    closed: bool,
    current_group: i32,
    current_end: i32,
    parent: i32,
    empty_count: u32,
    current_slot: u32,
    current_slot_end: u32,
}

impl<'a> SlotReader<'a> {
    pub(crate) fn new(table: &SlotTable) -> Self {
        SlotReader {
            table,
            groups: &table.groups,
            groups_size: table.groups_size,
        }
    }
}