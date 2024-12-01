use std::{any::TypeId, collections::HashSet};

use rust_ecs::{events::EventListener, systems::System, ComponentSignature, Entity, EntityManager};

use crate::{
    components::{HealthComponent, ProjectileComponent},
    events::CollisionEvent,
};

pub struct DamageSystem {
    signature: ComponentSignature,
    entities: HashSet<Entity>,
    event_types: [TypeId; 1],
}

impl Default for DamageSystem {
    fn default() -> Self {
        let event_types = [std::any::TypeId::of::<CollisionEvent>()];
        Self { signature: ComponentSignature::default(), entities: Default::default(), event_types }
    }
}

impl System for DamageSystem {
    fn signature(&self) -> &ComponentSignature {
        &self.signature
    }

    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn get_event_type(&self) -> &[std::any::TypeId] {
        self.event_types.as_slice()
    }
}

impl DamageSystem {
    fn on_player_projectile_collision(
        em: EntityManager,
        player: Entity,
        projectile: Entity,
    ) {
        let health_component = em
            .get_component::<HealthComponent>(&player)
            .unwrap();
        let projectile_component = em
            .get_component::<ProjectileComponent>(&projectile)
            .unwrap();

        let mut health_component = health_component.borrow_mut();
        let projectile_component = projectile_component.borrow();

        if projectile_component.friendly {
            return;
        }

        health_component.health -= projectile_component.damage;

        let played_is_dead = health_component.health <= 0;

        drop(health_component);
        drop(projectile_component);

        em.destroy_entity(projectile);
        if played_is_dead {
            em.destroy_entity(player);
        }
    }

    fn on_enemy_projectile_collision(
        em: EntityManager,
        enemy: Entity,
        projectile: Entity,
    ) {
        let health_component = em
            .get_component::<HealthComponent>(&enemy)
            .unwrap();
        let projectile_component = em
            .get_component::<ProjectileComponent>(&projectile)
            .unwrap();

        let mut health_component = health_component.borrow_mut();
        let projectile_component = projectile_component.borrow();

        if !projectile_component.friendly {
            return;
        }
        health_component.health -= projectile_component.damage;

        let is_dead = health_component.health <= 0;

        drop(health_component);
        drop(projectile_component);

        em.destroy_entity(projectile);
        if is_dead {
            em.destroy_entity(enemy);
        }
    }
}
impl EventListener for DamageSystem {
    fn on_event(&self, em: EntityManager, event: &rust_ecs::events::Event) {
        let event = event.get_data::<CollisionEvent>().unwrap();
        let entity_a = event.entity_a;
        let entity_b = event.entity_b;

        if em.tag_manager().has_tag(entity_a, "player")
            && em.group_manager().entity_in_group(&entity_b, "projectile")
        {
            DamageSystem::on_player_projectile_collision(em.clone(), entity_a, entity_b);
        }

        if em.tag_manager().has_tag(entity_b, "player")
            && em.group_manager().entity_in_group(&entity_a, "projectile")
        {
            DamageSystem::on_player_projectile_collision(em.clone(), entity_b, entity_a);
        }

        if em.group_manager().entity_in_group(&entity_a, "enemy")
            && em.group_manager().entity_in_group(&entity_b, "projectile")
        {
            DamageSystem::on_enemy_projectile_collision(em.clone(), entity_a, entity_b);
        }

        if em.group_manager().entity_in_group(&entity_b, "enemy")
            && em.group_manager().entity_in_group(&entity_a, "projectile")
        {
            DamageSystem::on_enemy_projectile_collision(em.clone(), entity_b, entity_a);
        }
    }
}
