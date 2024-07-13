use std::sync::Arc;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::QueryData,
    query::{QueryFilter, ROQueryItem},
    world::World,
};
use parking_lot::Mutex;

pub trait EntityPredicate<Q: QueryData, Filter: QueryFilter> {
    fn find(&self, ecs_lock: Arc<Mutex<World>>) -> Option<Entity>;
}
impl<F, Q, Filter> EntityPredicate<Q, Filter> for F
where
    F: Fn(&ROQueryItem<Q>) -> bool,
    Q: QueryData,
    Filter: QueryFilter,
{
    fn find(&self, ecs_lock: Arc<Mutex<World>>) -> Option<Entity> {
        let mut ecs = ecs_lock.lock();
        let mut query = ecs.query_filtered::<(Entity, Q), Filter>();
        let entity = query.iter(&ecs).find(|(_, q)| (self)(q)).map(|(e, _)| e);

        entity
    }
}

// impl<'a, F, Q1, Q2> EntityPredicate<'a, (Q1, Q2)> for F
// where
//     F: Fn(&<Q1 as WorldQuery>::Item<'_>, &<Q2 as WorldQuery>::Item<'_>) ->
// bool,     Q1: QueryFilter,
//     Q2: QueryFilter,
// {
//     fn find(&self, ecs: &mut Ecs) -> Option<Entity> {
//         // (self)(query)
//         let mut query = ecs.query_filtered::<(Entity, Q1, Q2), ()>();
//         let entity = query
//             .iter(ecs)
//             .find(|(_, q1, q2)| (self)(q1, q2))
//             .map(|(e, _, _)| e);

//         entity
//     }
// }
