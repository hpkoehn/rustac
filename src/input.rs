extern crate piston;

use piston::Button;
use piston::Key;

use crate::ecs::ECS;
use crate::gamelogic::*;
use crate::gamestate:: {
    actor::ActorState,
    movement::Direction
    };

///
pub fn handle_input(press_args: &Button, ecs_: &mut ECS) -> Option<Button> {
   if let Some(player) = ecs_.get_player_entity() {
       if let Some(actor_c) = ecs_.actor_component.get(player) {
           if actor_c.state == ActorState::WaitingForTurn {
               dungeon_actor_controls(press_args, ecs_);
               return None;
           } else {
               dungeon_passive_controls(press_args, ecs_);
               return Some(*press_args);
           }
       }
   }
    None
}

fn menu_controls(button: &Button, ecs_: &mut ECS) {

}

fn dungeon_actor_controls(button: &Button, ecs_: &mut ECS) {
    
    let player_option = ecs_.get_player_entity();
    if player_option.is_none() {
        return;
    }
    
    if let Button::Keyboard(key) = button {
        match key {
            Key::Down => {
                perform_player_action(ecs_, PlayerAction::Move(Direction::Down))
            },
            Key::Up => {
                perform_player_action(ecs_, PlayerAction::Move(Direction::Up))
            },
            Key::Left => {
                perform_player_action(ecs_, PlayerAction::Move(Direction::Left))
            },
            Key::Right => {
                perform_player_action(ecs_, PlayerAction::Move(Direction::Right))
            },
            Key::Space => {
                perform_player_action(ecs_, PlayerAction::Attack)
            }
            _ => {}
        };
    }
}

fn dungeon_passive_controls (button: &Button, ecs_: &mut ECS) {

}