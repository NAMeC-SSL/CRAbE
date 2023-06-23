use std::time::{SystemTime,UNIX_EPOCH};
use chrono::Utc;
use serde::de::Unexpected::Option;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::{GameState, World};
use crabe_protocol::protobuf::game_controller_packet::{GameEvent, GameEventProposalGroup, MatchType, Referee};
use crabe_protocol::protobuf::game_controller_packet::game_event::Event;
use crabe_protocol::protobuf::game_controller_packet::referee::TeamInfo;
use crate::data::referee::{RefereeCommand, Stage};
use crate::{FilterConfig, FilterPipeline};
use crate::data::referee::event::EventOrigin::GameController;
// Importez les éléments nécessaires
use crate::post_filter::game_controller::GameControllerPostFilter;
use crate::post_filter::PostFilter;
use crate::pre_filter::common::create_date_time;

#[test]
fn test_game_controller_step() {
    // Créez une instance de GameControllerPostFilter à tester
    let mut game_controller = GameControllerPostFilter::default();

    // Préparez un paquet Referee de test
    let now = SystemTime::now();
    let epoch = UNIX_EPOCH;
    match now.duration_since(epoch){
        Ok(duration) => {
            let now_in_u64 = duration.as_secs();
            let referee = Referee {
                source_identifier: Some("".parse().unwrap()),
                match_type: Some(3),
                packet_timestamp: now_in_u64,
                stage: 0,
                stage_time_left: Some(10000),
                command: crabe_protocol::protobuf::game_controller_packet::referee::Command::Halt.into(),
                command_counter: 0,
                command_timestamp: now_in_u64,
                yellow: TeamInfo {
                    name: "".to_string(),
                    score: 0,
                    red_cards: 0,
                    yellow_card_times: vec![],
                    yellow_cards: 0,
                    timeouts: 0,
                    timeout_time: 0,
                    goalkeeper: 0,
                    foul_counter: None,
                    ball_placement_failures: None,
                    can_place_ball: None,
                    max_allowed_bots: None,
                    bot_substitution_intent: None,
                    ball_placement_failures_reached: None,
                    bot_substitution_allowed: None,
                },
                blue: TeamInfo {
                    name: "".to_string(),
                    score: 0,
                    red_cards: 0,
                    yellow_card_times: vec![],
                    yellow_cards: 0,
                    timeouts: 0,
                    timeout_time: 0,
                    goalkeeper: 0,
                    foul_counter: None,
                    ball_placement_failures: None,
                    can_place_ball: None,
                    max_allowed_bots: None,
                    bot_substitution_intent: None,
                    ball_placement_failures_reached: None,
                    bot_substitution_allowed: None,
                },
                designated_position: None,
                blue_team_on_positive_half: Some(false),
                next_command: None,
                game_events: vec![GameEvent {
                    r#type: None,
                    origin: vec![],
                    created_timestamp: None,
                    event: None,
                }],
                game_event_proposals: vec![GameEventProposalGroup {
                    game_event: vec![],
                    accepted: None,
                }],
                current_action_time_remaining: None,
            };
            let data = InboundData{
                vision_packet: vec![],
                gc_packet: vec![referee],
                feedback: Default::default(),
            };
            let commonConfig = CommonConfig {
                yellow: false,
                real: false,
                gc: false,
            };
            let mut world = World::with_config(&commonConfig);
            let filterConfig = FilterConfig{

            };
            let mut filter_component = FilterPipeline::with_config(filterConfig, &commonConfig);
            filter_component.step(data,&mut world);
            let mut game_controller = GameControllerPostFilter::default();
            game_controller.step(&filter_component.filter_data,&mut world);
            assert_eq!(world.data.state, GameState::Halted);

        }
        Err(_) => {
            dbg!("Error");
        }
    }




    // Appelez la méthode step avec le paquet Referee de test
  //  game_controller.step(&referee_packet, &mut world);

    // Vérifiez que l'état de GameControllerPostFilter a été mis à jour conformément à vos attentes
   // assert_eq!(game_controller.some_property, expected_value);
    // Autres assertions pertinentes pour votre test
}