use super::slot_table::{Anchor, SlotTable};
use super::{Composer, Composition};
use std::any::Any;
use std::collections::HashSet;
use std::error::Error;
use std::ops::Deref;
use std::sync::Mutex;
use tokio::runtime::{Handle, Runtime};
use crate::runtime::Slot;

pub enum RecomposerState {
    ShutDown = 0,
    ShuttingDown,
    Inactive,
    InactivePendingWork,
    Idle,
    PendingWork,
}

impl Default for RecomposerState {
    fn default() -> Self {
        RecomposerState::Inactive
    }
}

#[derive(Default)]
pub struct RecomposerStates {
    pub(self) state: RecomposerState,
    pub(self) runner_job: Option<Handle>,
    pub(self) close_cause: Option<Box<dyn Error>>,
    pub(self) known_compositions: Vec<Composition>,
    pub(self) snapshot_invalidations: Vec<HashSet<Slot>>,
}

pub struct Recomposer {
    effect: Runtime,
    change_count: u64,
    state_lock: Mutex<RecomposerStates>,
}

impl Recomposer {
    pub fn new(effect: Runtime) -> Recomposer {
        Recomposer {
            effect,
            change_count: 0,
            state_lock: Mutex::new(RecomposerStates::default()),
        }
    }

    pub fn get_state(&self) -> &RecomposerStates {
        &self.state_lock.lock().unwrap().deref()
    }

    pub fn change_count(&self) -> u64 {
        self.change_count
    }

    pub(crate) fn compound_hash_key(&self) -> i32 {}
    pub(crate) fn collecting_parameter_information(&self) -> bool {}
    pub(crate) fn effect_coroutine_context(&self) -> &Runtime {
        &self.effect
    }
    pub(crate) fn recompose_coroutine_context(&self) -> Runtime {
        Runtime::new().unwrap()
    }
    pub(crate) fn compose_initial<F: Fn(Composer, u64)>(&mut self, composition: &Composition, content: F) {}
    pub(crate) fn invalidate(&mut self, composition: Composition) {}
    pub(crate) fn invalidate_scope(&mut self, scope: RecomposeScope) {}

    pub(crate) fn record_inspection_table(&mut self, table: HashSet<SlotTable>) {}
    pub(crate) fn register_composer(&mut self, composer: &Composer) {}
    pub(crate) fn unregister_composer(&mut self, composer: &Composer) {}
    pub(crate) fn register_composition(&mut self, composition: &Composition) {}
    pub(crate) fn unregister_composition(&mut self, composition: &Composition) {}

    pub(crate) fn start_composing() {}
    pub(crate) fn done_composing() {}
}

pub struct RecomposeScope {
    flags: u32,
    composition: Option<Composition>,
    anchor: Option<Anchor>,
}

impl RecomposeScope {
    pub fn new(composition: Option<Composition>) -> RecomposeScope {
        RecomposeScope {
            flags: 0u32,
            composition,
            anchor: None,
        }
    }

    pub fn valid(&self) -> bool {
        self.composition.is_some() && self.anchor.is_some_and(|a| a.valid())
    }
}
