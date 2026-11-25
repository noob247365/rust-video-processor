// SPDX-FileCopyrightText: 2026 noob247365
// SPDX-License-Identifier: GPL-3.0-or-later

use eframe::egui;

pub const MAX_ZOOM_FACTOR: f32 = 5.0;
pub const MIN_ZOOM_FACTOR: f32 = 1.0 / MAX_ZOOM_FACTOR;
pub const SCROLL_PAN_FACTOR: f32 = 1.0;

pub const WIRE_WIDTH: f32 = 3.0;
pub const CONTROL_GAP_DISTANCE_SCALING: f32 = 0.5;
pub const CONTROL_GAP_MIN: f32 = 25.0;
pub const CONTROL_GAP_MAX: f32 = 150.0;

pub const NODE_WIDTH: f32 = 140.0;
pub const NODE_CORNER_RADIUS: f32 = 5.0;
pub const NODE_BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_gray(40);
pub const NODE_BORDER_COLOR: egui::Color32 = egui::Color32::from_gray(100);
pub const NODE_HEADER_HEIGHT: f32 = 25.0;
pub const NODE_HEADER_FONT_SIZE: f32 = 14.0;
pub const NODE_HEADER_BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_gray(60);
pub const NODE_HEADER_TEXT_COLOR: egui::Color32 = egui::Color32::WHITE;

pub const PIN_ROW_HEIGHT: f32 = 20.0;
pub const PIN_RADIUS: f32 = 5.0;
pub const PIN_STROKE_WIDTH: f32 = 2.0;
pub const PIN_LABEL_FONT_SIZE: f32 = 12.0;
pub const PIN_LABEL_COLOR: egui::Color32 = egui::Color32::LIGHT_GRAY;

pub const WIRE_PIN_HOVER_FILL_COLOR: egui::Color32 = egui::Color32::WHITE;
pub const WIRE_PIN_HOVER_VALID_STROKE_COLOR: egui::Color32 = egui::Color32::LIGHT_GREEN;
pub const WIRE_PIN_HOVER_INVALID_STROKE_COLOR: egui::Color32 = egui::Color32::LIGHT_RED;
