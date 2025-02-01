extern crate proc_macro;

use crate::demo::gameevent_gen::get_sizes;
use crate::demo::gamevent::{GameEventDefinition, GameEventValueType};
use crate::demo::parser::MessageHandler;
use crate::{Demo, ParserState};
use crate::{DemoParser, MessageType};
use fnv::FnvHashMap;
use inflector::Inflector;
use lazy_static::lazy_static;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

struct GameEventAnalyser;

impl MessageHandler for GameEventAnalyser {
    type Output = Vec<GameEventDefinition>;

    fn does_handle(_message_type: MessageType) -> bool {
        false
    }

    fn into_output(self, state: &ParserState) -> Self::Output {
        state.event_definitions.clone()
    }
}

fn should_box_event(name: &str) -> bool {
    lazy_static! {
        static ref SIZES: FnvHashMap<&'static str, usize> = get_sizes();
    }

    SIZES.get(name).cloned().unwrap_or_default() > 120
}

fn get_type_name(ty: GameEventValueType) -> &'static str {
    match ty {
        GameEventValueType::String => "MaybeUtf8String",
        GameEventValueType::Float => "f32",
        GameEventValueType::Boolean => "bool",
        GameEventValueType::Byte => "u8",
        GameEventValueType::Local => "()",
        GameEventValueType::Long => "u32",
        GameEventValueType::Short => "u16",
        GameEventValueType::None => "()",
    }
}

fn get_entry_name(name: &str) -> String {
    lazy_static! {
        static ref REPLACEMENTS: Vec<(&'static str, &'static str)> = vec!(
            ("mapname", "map_name"),
            ("cvarname", "cvar_name"),
            ("cvarvalue", "cvar_value"),
            ("userid", "user_id"),
            ("networkid", "network_id"),
            ("teamid", "team_id"),
            ("teamname", "team_name"),
            ("oldteam", "old_team"),
            ("autoteam", "auto_team"),
            ("entindex", "ent_index"),
            ("weaponid", "weapon_id"),
            ("damagebit", "damage_bit"),
            ("customkill", "custom_kill"),
            ("logclassname", "log_class_name"),
            ("playerpenetratecount", "player_penetrate_count"),
            ("damageamount", "damage_amount"),
            ("showdisguisedcrit", "show_disguised_crit"),
            ("minicrit", "mini_crit"),
            ("allseecrit", "all_see_crit"),
            ("weaponid", "weapon_id"),
            ("bonuseffect", "bonus_effect"),
            ("teamonly", "team_only"),
            ("oldname", "old_name"),
            ("newname", "new_name"),
            ("hintmessage", "hint_message"),
            ("roundslimit", "rounds_limit"),
            ("timelimit", "time_limit"),
            ("fraglimit", "frag_limit"),
            ("numadvanced", "num_advanced"),
            ("numbronze", "num_bronze"),
            ("numsilver", "num_silver"),
            ("numgold", "num_gold"),
            ("oldmode", "old_mode"),
            ("newmode", "new_mode"),
            ("entityid", "entity_id"),
            ("winreason", "win_reason"),
            ("flagcaplimit", "flag_cap_limit"),
            ("cpname", "cp_name"),
            ("capteam", "cap_team"),
            ("captime", "cap_time"),
            ("eventtype", "event_type"),
            ("killstreak", "kill_stream"),
            ("forceupload", "force_upload"),
            ("targetid", "target_id"),
            ("isbuilder", "is_builder"),
            ("objecttype", "object_type"),
            ("namechange", "name_change"),
            ("readystate", "ready_state"),
            ("builderid", "builder_id"),
            ("recedetime", "recede_time"),
            ("ownerid", "owner_id"),
            ("sapperid", "sapper_id"),
            ("itemdef", "item_def"),
            ("bitfield", "bit_field"),
            ("playsound", "play_sound"),
            ("totalhits", "total_hits"),
            ("posx", "pos_x"),
            ("posy", "pos_y"),
            ("posz", "pos_z"),
            ("ineye", "in_eye"),
            ("maxplayers", "max_players"),
            ("levelname", "level_name"),
            ("isstrange", "is_strange"),
            ("isunusual", "is_unusual"),
            ("defindex", "definition_index"),
            ("matchgroup", "match_group")
        );
    }

    if name == "type" {
        return "kind".to_string();
    }

    let mut snake: String = name.to_snake_case();
    for (search, replace) in REPLACEMENTS.iter() {
        snake = snake.replace(search, replace);
    }

    snake
}

fn get_event_name(name: &str) -> String {
    lazy_static! {
        static ref REPLACEMENTS: Vec<(&'static str, &'static str)> = vec!(
            ("ReplayReplaysavailable", "ReplayReplaysAvailable"),
            ("ServerAddban", "ServerAddBan"),
            ("ServerRemoveban", "ServerRemoveBan"),
            ("ClientBeginconnect", "ClientBeginConnect"),
            ("ClientFullconnect", "ClientFullConnect"),
            ("PlayerChangename", "PlayerChangeName"),
            ("PlayerHintmessage", "PlayerHintMessage"),
            ("GameNewmap", "GameNewMap"),
            ("IntroNextcamera", "IntroNextCamera"),
            ("PlayerChangeclass", "PlayerChangeClass"),
            ("Updateimages", "UpdateImages"),
            ("Updatelayout", "UpdateLayout"),
            ("Updatecapping", "UpdateCapping"),
            ("Updateowner", "UpdateOwner"),
            ("Starttouch", "StartTouch"),
            ("Endtouch", "EndTouch"),
            ("FakeCaptureMult", "FakeCaptureMultiplier"),
            ("TeamplayWaitingAbouttoend", "TeamPlayWaitingAboutToEnd"),
            ("TeamplayPointStartcapture", "TeamPlayPointStartCapture"),
            ("FreezecamStarted", "FreezeCamStarted"),
            ("LocalplayerChangeteam", "LocalPlayerChangeTeam"),
            ("LocalplayerChangeclass", "LocalPlayerChangeClass"),
            ("LocalplayerChangedisguise", "LocalPlayerChangeDisguise"),
            ("FlagstatusUpdate", "FlagStatusUpdate"),
            ("TournamentEnablecountdown", "TournamentEnableCountdown"),
            ("PlayerCalledformedic", "PlayerCalledForMedic"),
            ("PlayerAskedforball", "PlayerAskedForBall"),
            ("LocalplayerBecameobserver", "LocalPlayerBecameObserver"),
            ("PlayerHealedmediccall", "PlayerHealedMedicCall"),
            ("ArenaMatchMaxstreak", "ArenaMatchMaxStreak"),
            ("StatsResetround", "StatsResetRound"),
            ("FishNotice_arm", "FishNoticeArm"),
            ("PlayerBonuspoints", "PlayerBonusPoints"),
            ("PlayerUsedPowerupBottle", "PlayerUsedPowerUpBottle"),
            ("ReplayStartrecord", "ReplayStartRecord"),
            ("ReplaySessioninfo", "ReplaySessionInfo"),
            ("ReplayEndrecord", "ReplayEndRecord"),
            ("ReplayServererror", "ReplayServerError"),
            ("Teamplay", "TeamPlay"),
            ("death", "Death"),
            ("panel", "Panel"),
            ("object", "Object"),
            ("update", "Update"),
            ("ready", "Ready"),
            ("Gameui", "GameUI"),
            ("onhit", "OnHit"),
            ("bymedic", "ByMedic"),
            ("Controlpoint", "ControlPoint"),
            ("Pipebomb", "PipeBomb"),
            ("Scorestats", "ScoreStats"),
            ("Creditbonus", "CreditBonus"),
            ("Sentrybuster", "SentryBuster"),
            ("Questlog", "QuestLog"),
            ("Localplayer", "LocalPlayer"),
            ("Minigame", "MiniGame"),
            ("Winlimit", "WinLimit"),
            ("Skillrating", "SkillRating"),
            ("Directhit", "DirectHit"),
            ("Chargedeployed", "ChargeDeployed"),
            ("Winddown", "WindDown"),
            ("Stealsandvich", "StealSandvich"),
            ("Pricesheet", "PriceSheet"),
            ("Teambalanced", "TeamBalanced"),
            ("Highfive", "HighFive"),
            ("Powerup", "PowerUp"),
            ("Hltv", "HLTV"),
            ("Changelevel", "ChangeLevel"),
            ("Rocketpack", "RocketPack"),
            ("Deadringer", "DeadRinger"),
            ("Mainmenu", "MainMenu"),
            ("Mmstats", "MMStats"),
        );
    }

    let mut snake: String = name.to_pascal_case();
    for (search, replace) in REPLACEMENTS.iter() {
        snake = snake.replace(search, replace);
    }

    snake
}

pub fn generate_game_events(demo: Demo) -> TokenStream {
    let (_, mut events) = DemoParser::new_with_analyser(demo.get_stream(), GameEventAnalyser)
        .parse()
        .unwrap();

    events.sort();

    let span = Span::call_site();

    let imports = quote!(
        use super::gamevent::{
            EventValue, GameEventDefinition, GameEventEntry, RawGameEvent, GameEventValue,
        };
        use crate::demo::Stream;
        use crate::{ParseError, Result};
        use bitbuffer::{BitRead, LittleEndian, BitWrite, BitWriteStream};
        use serde::{Deserialize, Serialize};
        use crate::demo::data::MaybeUtf8String;
    );

    let event_definitions = events.iter().map(|event| {
        let fields = event.entries.iter().map(|entry| {
            let name = Ident::new(&get_entry_name(&entry.name), span);
            let ty = Ident::new(get_type_name(entry.kind), span);

            quote!(pub #name: #ty,)
        });

        let field_getters = event.entries.iter().map(|entry| {
            let raw_name = &entry.name;
            let name = get_entry_name(&entry.name);
            let ident = Ident::new(name.as_str(), span);

            quote!(#raw_name => Ok(self.#ident.clone().into()))
        });

        let event_name = get_event_name(event.event_type.as_str());
        let name = Ident::new(&format!("{event_name}Event"), span);

        let entry_readers = event.entries.iter().map(|entry| {
            let raw_name = &entry.name;
            let name_str = get_entry_name(&entry.name);
            let name = Ident::new(&name_str, span);
            let ty = Ident::new(get_type_name(entry.kind), span);

            quote!(
                #name: read_value::<#ty>(stream, definition.get_entry(#raw_name), #name_str)?,
            )
        });

        quote!(
            #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
            #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
            pub struct #name {
                #(#fields)*
            }

            impl #name {
                #[allow(unused_variables)]
                fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
                    Ok(#name {
                        #(#entry_readers)*
                    })
                }

                #[allow(unused_variables)]
                fn get_field(&self, field: &str) -> Result<GameEventValue> {
                    #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
                    match field {
                        #(#field_getters,)*
                        _ => Err(ParseError::MissingGameEventValue {
                            ty: #event_name,
                            field: field.into(),
                        }),
                    }
                }

                #[allow(unused_variables)]
                fn write(&self, stream: &mut BitWriteStream<LittleEndian>, definition: &GameEventDefinition) -> Result<()> {
                    for entry in &definition.entries {
                        let value = self.get_field(&entry.name).unwrap_or_else(|_| entry.kind.default_value());
                        stream.write(&value)?;
                    }
                    Ok(())
                }
            }
        )
    });

    let event_variants = events.iter().map(|event| {
        let name_str = get_event_name(event.event_type.as_str());
        let name = Ident::new(&name_str, span);
        let struct_name = Ident::new(&format!("{}Event", name_str), span);

        if should_box_event(&name_str) {
            quote!(#name(Box<#struct_name>),)
        } else {
            quote!(#name(#struct_name),)
        }
    });

    let event_types = events.iter().map(|event| {
        let type_name = event.event_type.as_str();
        let name_str = get_event_name(type_name);
        let name = Ident::new(&name_str, span);

        quote!(
            #[serde(rename = #type_name)]
            #name,
        )
    });

    let type_from_names = events.iter().map(|event| {
        let name_str = event.event_type.as_str();
        let variant_name = Ident::new(&get_event_name(&name_str), span);

        quote!(#name_str => GameEventType::#variant_name,)
    });

    let type_to_names = events.iter().map(|event| {
        let name_str = event.event_type.as_str();
        let variant_name = Ident::new(&get_event_name(&name_str), span);

        quote!(GameEventType::#variant_name => #name_str,)
    });

    let to_types = events.iter().map(|event| {
        let name = get_event_name(event.event_type.as_str());
        let variant_name = Ident::new(&name, span);

        quote!(GameEvent::#variant_name(_) => GameEventType::#variant_name,)
    });

    let read_events = events.iter().map(|event| {
        let name = get_event_name(event.event_type.as_str());
        let variant_name = Ident::new(&name, span);
        let struct_name = Ident::new(&format!("{}Event", name), span);

        if should_box_event(&name) {
            quote!(
                GameEventType::#variant_name => {
                    GameEvent::#variant_name(Box::new(<#struct_name>::read(stream, definition)?))
                }
            )
        } else {
            quote!(
                GameEventType::#variant_name => {
                    GameEvent::#variant_name(#struct_name::read(stream, definition)?)
                }
            )
        }
    });

    let write_events = events.iter().map(|event| {
        let name = get_event_name(event.event_type.as_str());
        let variant_name = Ident::new(&name, span);

        quote!(
            GameEvent::#variant_name(event) => event.write(stream, definition),
        )
    });

    let sizes = events.iter().map(|event| {
        let name = get_event_name(event.event_type.as_str());
        let struct_name = Ident::new(&format!("{}Event", name), span);

        quote!(
            (#name, std::mem::size_of::<#struct_name>())
        )
    });

    quote!(
        #imports

        fn read_value<'a, T: EventValue + BitRead<'a, LittleEndian> + Default>(
            stream: &mut Stream<'a>,
            entry: Option<&GameEventEntry>,
            name: &'static str,
        ) -> Result<T> {
            let entry = match entry {
                Some(entry) => entry,
                None => {
                    return Ok(T::default());
                }
            };
            if T::value_type() != entry.kind {
                return Ok(T::default());
                //return Err(ParseError::InvalidGameEvent {
                //    expected_type: T::value_type(),
                //    name,
                //    found_type: entry.kind,
                //});
            }
            Ok(T::read(stream)?)
        }

        #(#event_definitions)*


        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[serde(tag = "type")]
        pub enum GameEvent {
            #(#event_variants)*
            Unknown(RawGameEvent),
        }

        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum GameEventType {
            #(#event_types)*
            Unknown(String),
        }

        impl GameEventType {
            pub fn from_type_name(name: &str) -> Self {
                match name {
                    #(#type_from_names)*
                    ty => GameEventType::Unknown(ty.into()),
                }
            }
            pub fn as_str(&self) -> &str {
                match self {
                    #(#type_to_names)*
                    GameEventType::Unknown(ty) => ty,
                }
            }
        }

        impl GameEvent {
            pub fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
                Ok(match definition.event_type {
                    #(#read_events)*
                    GameEventType::Unknown(_) => GameEvent::Unknown(RawGameEvent::read(stream, definition)?),
                })
            }
            pub fn write(&self, stream: &mut BitWriteStream<LittleEndian>, definition: &GameEventDefinition) -> Result<()> {
                match &self {
                    #(#write_events)*
                    GameEvent::Unknown(raw) => Ok(raw.write(stream)?),
                }
            }
            pub fn event_type(&self) -> GameEventType {
                match &self {
                    #(#to_types)*
                    GameEvent::Unknown(raw) => raw.event_type.clone(),
                }
            }
        }

        pub fn get_sizes() -> fnv::FnvHashMap<&'static str, usize> {
            [
                #(#sizes,)*
            ].iter().copied().collect()
        }
    )
}
