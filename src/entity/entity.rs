pub type EntityId = usize;

pub(super) fn get_next_entity_id() -> usize {
    static NEXT_ENTITY_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    NEXT_ENTITY_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Entity { id }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}
