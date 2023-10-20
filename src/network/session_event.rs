use bevy::prelude::*;
use bevy_ggrs::{ggrs::GGRSEvent as GgrsEvent, *};

use crate::{GameState, RollbackState};

use super::GgrsConfig;

#[derive(Event)]
pub struct SessionEvent {
    main_menu: bool,
}

fn handle_session_event(
    event: GgrsEvent<GgrsConfig>,
    ev_session_event: &mut EventWriter<SessionEvent>,
) {
    match event {
        GgrsEvent::Synchronizing { .. } => {}
        GgrsEvent::Synchronized { .. } => {}
        GgrsEvent::Disconnected { .. } => {
            // Return to main menu
            ev_session_event.send(SessionEvent { main_menu: true });
        }
        GgrsEvent::NetworkInterrupted { .. } => {}
        GgrsEvent::NetworkResumed { .. } => {}
        GgrsEvent::WaitRecommendation { .. } => {}
        GgrsEvent::DesyncDetected { .. } => {}
    }
}

fn debug_session_event(event: &GgrsEvent<GgrsConfig>) {
    match event {
        GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
            warn!("{event:?}")
        }
        GgrsEvent::DesyncDetected { .. } => error!("{event:?}"),
        _ => info!("{event:?}"),
    }
}

pub fn handle_session_events(
    mut session: ResMut<Session<GgrsConfig>>,
    mut ev_session_event: EventWriter<SessionEvent>,
) {
    match session.as_mut() {
        Session::P2P(s) => {
            for event in s.events() {
                debug_session_event(&event);
                handle_session_event(event, &mut ev_session_event);
            }
        }
        _ => panic!("Expecting a P2P Session."),
    }
}

pub fn change_game_state(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut ev_session_event: EventReader<SessionEvent>,
) {
    for ev in ev_session_event.iter() {
        if ev.main_menu {
            next_game_state.set(GameState::MainMenu);
        }
    }
}

pub fn change_rollback_state(
    mut next_rollback_state: ResMut<NextState<RollbackState>>,
    mut ev_session_event: EventReader<SessionEvent>,
) {
    for ev in ev_session_event.iter() {
        if ev.main_menu {
            next_rollback_state.set(RollbackState::Setup);
        }
    }
}
