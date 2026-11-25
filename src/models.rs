// SPDX-FileCopyrightText: 2026 noob247365
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::Display;

use crate::enum_with_members;
use eframe::egui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

enum_with_members! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PinKindBase {
        ExecutionFlow,
        Track,
        VideoTrack,
        AudioTrack,
        SubtitleTrack,
        Chapter,
        Attachment,
        Number,
        String,
        Rectangle,
    }
}

impl PinKindBase {
    pub fn color(&self) -> egui::Color32 {
        match self {
            PinKindBase::ExecutionFlow => egui::Color32::WHITE,
            PinKindBase::Track => egui::Color32::from_rgb(255, 100, 255), // LIGHT_PURPLE
            PinKindBase::VideoTrack => egui::Color32::LIGHT_BLUE,
            PinKindBase::AudioTrack => egui::Color32::LIGHT_GREEN,
            PinKindBase::SubtitleTrack => egui::Color32::LIGHT_YELLOW,
            PinKindBase::Chapter => egui::Color32::from_rgb(255, 200, 100), // LIGHT_ORANGE
            PinKindBase::Attachment => egui::Color32::LIGHT_RED,
            PinKindBase::Number => egui::Color32::GREEN,
            PinKindBase::String => egui::Color32::BLUE,
            PinKindBase::Rectangle => egui::Color32::RED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinKind {
    pub base: PinKindBase,
    //pub is_list: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pin {
    pub id: Uuid,
    pub label: String,
    pub kind: PinKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    pub position: egui::Pos2,
}

impl Node {
    pub fn get_pin_by_id(&self, pin_id: Uuid, is_input: bool) -> Option<&Pin> {
        let pin_list = if is_input {
            &self.inputs
        } else {
            &self.outputs
        };
        for pin in pin_list.iter() {
            if pin.id == pin_id {
                return Some(pin);
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificPin {
    pub node_id: Uuid,
    pub pin_id: Uuid,
    pub is_input: bool,
}

impl Display for SpecificPin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(node_id: {}, pin_id: {}, is_input: {})",
            self.node_id, self.pin_id, self.is_input
        )
    }
}
