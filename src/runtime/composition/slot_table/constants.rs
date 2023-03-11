use crate::util::magic::GInto;

pub(super) fn count_one_bits(value: impl GInto<usize>) -> i32 {
    match value.g_into() {
        0 => 0,
        1 | 2 | 4 => 1,
        3 | 5 | 6 => 2,
        _ => 3,
    }
}

// Parent -1 is reserved to be the root parent index so the anchor must pivot on -2.
pub(super) const PARENT_ANCHOR_PIVOT: i32 = -2;

// Group layout
//  0             | 1             | 2             | 3             | 4             |
//  Key           | Group info    | Parent anchor | Size          | Data anchor   |
pub(super) const KEY_OFFSET: i32 = 0;
pub(super) const GROUP_INFO_OFFSET: i32 = 1;
pub(super) const PARENT_ANCHOR_OFFSET: i32 = 2;
pub(super) const SIZE_OFFSET: i32 = 3;
pub(super) const DATA_ANCHOR_OFFSET: i32 = 4;
pub(super) const GROUP_FIELDS_SIZE: i32 = 5;

// Key is the key parameter passed into startGroup

// Group info is laid out as follows,
// 31 30 29 28_27 26 25 24_23 22 21 20_19 18 17 16_15 14 13 12_11 10 09 08_07 06 05 04_03 02 01 00
// 0  n  ks ds m  cm|                                node count                                    |
// where n is set when the group represents a node
// where ks is whether the group has a object key slot
// where ds is whether the group has a group data slot
// where m is whether the group is marked
// where cm is whether the group contains a mark

// Parent anchor is a group anchor to the parent, as the group gap is moved self value is updated to
// refer to the parent.

// Slot count is the total number of group slots, including itself, occupied by the group.

// Data anchor is an anchor to the group data. The value is positive if it is before the data gap
// and it is negative if it is after the data gap. As gaps are moved, these values are updated.

// Masks and flags
pub(super) const NODE_BIT_MASK: i32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;
pub(super) const OBJECT_KEY_MASK: i32 = 0b0010_0000_0000_0000_0000_0000_0000_0000;
pub(super) const OBJECT_KEY_SHIFT: i32 = 29;
pub(super) const AUX_MASK: i32 = 0b0001_0000_0000_0000_0000_0000_0000_0000;
pub(super) const AUX_SHIFT: i32 = 28;
pub(super) const MARK_MASK: i32 = 0b0000_1000_0000_0000_0000_0000_0000_0000;
pub(super) const CONTAINS_MARK_MASK: i32 = 0b0000_0100_0000_0000_0000_0000_0000_0000;
pub(super) const SLOTS_SHIFT: i32 = AUX_SHIFT;
pub(super) const NODE_COUNT_MASK: i32 = 0b0000_0011_1111_1111_1111_1111_1111_1111;

// Special values

// The minimum number of groups to allocate the group slot_table
pub(super) const MIN_GROUP_GROWTH_SIZE: i32 = 32;

// The minimum number of data slots to allocate in the data slot slot_table
pub(super) const MIN_SLOTS_GROWTH_SIZE: i32 = 32;

// The key to used for nodes
pub(super) const NODE_KEY: i32 = 125;
