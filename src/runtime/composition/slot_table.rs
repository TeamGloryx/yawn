use std::default::default;

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
    pub(crate) fn new() -> SlotTable {
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
}
