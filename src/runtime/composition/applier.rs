use crate::runtime::Composer;

pub trait Applier<N: Node> {
    /// The node that operations will be applied on at any given time. It is expected that the
    /// value of this property will change as [Applier::down] and [Applier::up] are called.
    fn current(&self) -> &N;

    /// Called when the [Composer] is about to begin applying changes using this applier.
    /// [Applier::on_end_changes] will be called when changes are complete.
    fn on_begin_changes(&mut self);

    /// Called when the [Composer] is finished applying changes using this applier.
    /// A call to [Applier::on_begin_changes] will always precede a call to [Applier::on_end_changes].
    fn on_end_changes(&mut self);

    /// Indicates that the applier is getting traversed "down" the tree. When this gets called,
    /// `node` is expected to be a child of `current`, and after this operation, `node` is
    /// expected to be the new `current`.
    fn down(&mut self, node: N);

    /// Indicates that the applier is getting traversed "up" the tree. After this operation
    /// completes, the [current] should return the "parent" of the [current] node at the beginning
    /// of this operation.
    fn up(&mut self);

    /// Indicates that [instance] should be inserted as a child to [current] at [index]. An applier
    /// should insert the node into the tree either in [insertTopDown] or [insertBottomUp], not both.
    ///
    /// The [insertTopDown] method is called before the children of [instance] have been created and
    /// inserted into it. [insertBottomUp] is called after all children have been created and
    /// inserted.
    ///
    /// Some trees are faster to build top-down, in which case the [insertTopDown] method should
    /// be used to insert the [instance]. Other tress are faster to build bottom-up in which case
    /// [insertBottomUp] should be used.
    ///
    /// To give example of building a tree top-down vs. bottom-up consider the following tree,
    ///
    /// ```md
    ///      R
    ///      |
    ///      B
    ///     / \
    ///    A   C
    ///  ```
    ///
    ///  where the node `B` is being inserted into the tree at `R`. Top-down building of the tree
    ///  first inserts `B` into `R`, then inserts `A` into `B` followed by inserting `C` into B`.
    ///  For example,
    ///
    ///  ```md
    ///      1           2           3
    ///      R           R           R
    ///      |           |           |
    ///      B           B           B
    ///                 /           / \
    ///                A           A   C
    /// ```
    ///
    /// A bottom-up building of the tree starts with inserting `A` and `C` into `B` then inserts
    /// `B` tree into `R`.
    ///
    /// ```md
    ///    1           2           3
    ///    B           B           R
    ///    |          / \          |
    ///    A         A   C         B
    ///                           / \
    ///                          A   C
    /// ```
    ///
    /// To see how building top-down vs. bottom-up can differ significantly in performance
    /// consider a tree where whenever a child is added to the tree all parent nodes, up to the root,
    /// are notified of the new child entering the tree. If the tree is built top-down,
    ///
    ///  1. `R` is notified of `B` entering.
    ///  2. `B` is notified of `A` entering, `R` is notified of `A` entering.
    ///  3. `B` is notified of `C` entering, `R` is notified of `C` entering.
    ///
    ///  for a total of 5 notifications. The number of notifications grows exponentially with the
    ///  number of inserts.
    ///
    ///  For bottom-up, the notifications are,
    ///
    ///  1. `B` is notified `A` entering.
    ///  2. `B` is notified `C` entering.
    ///  3. `R` is notified `B` entering.
    ///
    ///  The notifications are linear to the number of nodes inserted.
    ///
    ///  If, on the other hand, all children are notified when the parent enters a tree, then the
    ///  notifications are, for top-down,
    ///
    ///  1. `B` is notified it is entering `R`.
    ///  2. `A` is notified it is entering `B`.
    ///  3. `C` is notified it is entering `B`.
    ///
    ///  which is linear to the number of nodes inserted.
    ///
    ///  For bottom-up, the notifications look like,
    ///
    ///  1. `A` is notified it is entering `B`.
    ///  2. `C` is notified it is entering `B`.
    ///  3. `B` is notified it is entering `R`, `A` is notified it is entering `R`,
    ///     `C` is notified it is entering `R`.
    ///
    ///  which exponential to the number of nodes inserted.
    fn insert_top_down(&mut self, index: u32, instance: N);

    /// Indicates that [instance] should be inserted as a child of [current] at [index]. An applier
    /// should insert the node into the tree either in [insertTopDown] or [insertBottomUp], not
    /// both. See the description of [insertTopDown] to which describes when to implement
    /// [insertTopDown] and when to use [insertBottomUp].
    fn insert_bottom_up(&mut self, index: u32, instance: N);

    /// Indicates that the children of `current` from `index` to `index` + `count` should be removed.
    fn remove(&mut self, index: u32, count: u32);

    /// Indicates that `count` children of `current` should be moved from index `from` to index `to`.
    ///
    /// The `to` index is relative to the position before the change, so, for example, to move an
    /// element at position 1 to after the element at position 2, `from` should be `1` and `to`
    /// should be `3`. If the elements were A B C D E, calling `move_nodes(1, 3, 1)` would result in the
    /// elements being reordered to A C B D E.
    fn move_nodes(&mut self, from: u32, to: u32, count: u32);

    /// Move to the root and remove all nodes from the root, preparing both this [Applier]
    /// and its root to be used as the target of a new composition in the future.
    fn clear(&mut self);
}

/// A trait that all nodes, that could be used in a `yawn` tree, implement.
pub trait Node {}