// SPDX-FileCopyrightText: 2026 noob247365
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::constants::*;
use crate::models::*;
use eframe::egui;
use uuid::Uuid;

pub fn canvas_to_screen(pan: egui::Vec2, zoom: f32, canvas_pos: egui::Pos2) -> egui::Pos2 {
    (canvas_pos.to_vec2() * zoom + pan).to_pos2()
}

pub fn screen_to_canvas(pan: egui::Vec2, zoom: f32, screen_pos: egui::Pos2) -> egui::Pos2 {
    ((screen_pos.to_vec2() - pan) / zoom).to_pos2()
}

pub fn get_pin_screen_pos(
    pan: egui::Vec2,
    zoom: f32,
    node: &Node,
    pin_index: usize,
    is_input: bool,
) -> egui::Pos2 {
    canvas_to_screen(
        pan,
        zoom,
        egui::pos2(
            node.position.x + if is_input { 0.0 } else { NODE_WIDTH },
            node.position.y + NODE_HEADER_HEIGHT + (pin_index as f32 + 0.5) * PIN_ROW_HEIGHT,
        ),
    )
}

pub fn random_node(pos: egui::Pos2) -> Node {
    let node_id = Uuid::new_v4();
    let in_count = rand::random_range(1..=4);
    let out_count = rand::random_range(1..=4);

    let mut inputs = Vec::new();
    for i in 1..=in_count {
        inputs.push(Pin {
            id: Uuid::new_v4(),
            label: format!("Input {}", i),
            kind: PinKind {
                base: PinKindBase::MEMBERS[rand::random_range(..PinKindBase::MEMBERS.len())],
                //is_list: rand::random_bool(0.25), // TODO: Add back when pin rendering actually shows
            },
        });
    }

    let mut outputs = Vec::new();
    for i in 1..=out_count {
        outputs.push(Pin {
            id: Uuid::new_v4(),
            label: format!("Output {}", i),
            kind: PinKind {
                base: PinKindBase::MEMBERS[rand::random_range(..PinKindBase::MEMBERS.len())],
                //is_list: rand::random_bool(0.25),
            },
        });
    }

    Node {
        id: node_id,
        label: node_id.to_string(),
        inputs,
        outputs,
        position: pos,
    }
}
