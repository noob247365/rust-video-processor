// SPDX-FileCopyrightText: 2026 noob247365
// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod helpers;
mod meta;
mod models;

use constants::*;
use eframe::egui;
use helpers::*;
use models::*;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug)]
pub struct BlueprintEditor {
    nodes: HashMap<Uuid, Node>,
    wires: HashMap<SpecificPin, SpecificPin>, // [input] -> output

    pan: egui::Vec2,
    zoom: f32,

    wire_start: Option<SpecificPin>,

    next_node_number: i32, // TODO: Remove when non-random nodes are implemented
}

#[derive(Debug)]
struct UiChanges {
    nodes_to_delete: HashSet<Uuid>,
    nodes_to_reposition: HashMap<Uuid, egui::Pos2>,
    pin_to_start_wire_creation: Option<SpecificPin>,
    pin_to_finish_wire_creation: Option<SpecificPin>,
    pins_to_disconnect: HashSet<SpecificPin>,
}

impl Default for BlueprintEditor {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            wires: HashMap::new(),
            pan: egui::vec2(0.0, 0.0),
            zoom: 1.0,
            wire_start: None,

            next_node_number: 0,
        }
    }
}

impl BlueprintEditor {
    fn compute_pin_positions(&self) -> HashMap<SpecificPin, egui::Pos2> {
        let mut pin_positions = HashMap::new();
        for (_, node) in self.nodes.iter() {
            for (i, pin) in node.inputs.iter().enumerate() {
                pin_positions.insert(
                    SpecificPin {
                        node_id: node.id,
                        pin_id: pin.id,
                        is_input: true,
                    },
                    get_pin_screen_pos(self.pan, self.zoom, node, i, true),
                );
            }
            for (i, pin) in node.outputs.iter().enumerate() {
                pin_positions.insert(
                    SpecificPin {
                        node_id: node.id,
                        pin_id: pin.id,
                        is_input: false,
                    },
                    get_pin_screen_pos(self.pan, self.zoom, node, i, false),
                );
            }
        }
        pin_positions
    }

    fn handle_context_menu(&mut self, response: &egui::Response) {
        response.context_menu(|ui| {
            if ui.button("Add Random Node").clicked() {
                let canvas_pos = screen_to_canvas(
                    self.pan,
                    self.zoom,
                    ui.ctx()
                        .pointer_interact_pos()
                        .unwrap_or_else(|| ui.max_rect().center()),
                );

                let mut node = random_node(canvas_pos);
                self.next_node_number += 1;
                node.label = format!("Node {}", self.next_node_number);
                self.nodes.insert(node.id, node);

                ui.close();
            }
        });
    }

    fn handle_pan_zoom(&mut self, ui: &egui::Ui, response: &egui::Response, alt_held: bool) {
        // Pan: alt LMB, MMB
        let is_panning =
            response.dragged_by(egui::PointerButton::Middle) || (response.dragged() && alt_held);
        if is_panning {
            self.pan += response.drag_delta();
        } else if response.hovered() {
            self.pan += ui.input(|i| i.smooth_scroll_delta) * SCROLL_PAN_FACTOR;
        }

        // Zoom: ctrl scroll, ctrl +/-, trackpad pinch
        let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
        if zoom_delta != 1.0 {
            let new_zoom = (self.zoom * zoom_delta).clamp(MIN_ZOOM_FACTOR, MAX_ZOOM_FACTOR);

            let zoom_center = ui
                .ctx()
                .pointer_hover_pos()
                .unwrap_or_else(|| ui.max_rect().center());
            self.pan =
                zoom_center.to_vec2() - (zoom_center.to_vec2() - self.pan) * (new_zoom / self.zoom);
            self.zoom = new_zoom;
        }
    }

    fn handle_ui_changes(&mut self, ui: &egui::Ui, changes: UiChanges) {
        let mut all_pins_to_disconnect = changes.pins_to_disconnect.clone();

        for node_id in changes.nodes_to_delete.iter() {
            let Some(node) = self.nodes.get(node_id) else {
                continue;
            };
            for pin in node.inputs.iter() {
                all_pins_to_disconnect.insert(SpecificPin {
                    node_id: node.id,
                    pin_id: pin.id,
                    is_input: true,
                });
            }
            for pin in node.outputs.iter() {
                all_pins_to_disconnect.insert(SpecificPin {
                    node_id: node.id,
                    pin_id: pin.id,
                    is_input: false,
                });
            }
        }

        if !all_pins_to_disconnect.is_empty() {
            if let Some(wire_start) = self.wire_start
                && all_pins_to_disconnect.contains(&wire_start)
            {
                self.wire_start = None;
            }
            self.wires.retain(|k, v| {
                !all_pins_to_disconnect.contains(k) && !all_pins_to_disconnect.contains(v)
            });
        }

        for node_id in changes.nodes_to_delete.iter() {
            self.nodes.remove(node_id);
        }

        for (node_id, new_position) in changes.nodes_to_reposition.iter() {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.position = *new_position;
            }
        }

        if let Some(wire_start) = changes.pin_to_start_wire_creation
            && self.wire_start.is_none()
        {
            self.wire_start = Some(wire_start);
        } else if let Some(wire_end) = changes.pin_to_finish_wire_creation
            && let Some(wire_start) = self.wire_start
        {
            self.wires.insert(wire_end, wire_start);
            self.wire_start = None;
        } else if self.wire_start.is_some() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            // TODO: Click on background to stop
            self.wire_start = None;
        }
    }

    fn draw_ui_nodes(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        alt_held: bool,
        pin_positions: &HashMap<SpecificPin, egui::Pos2>,
    ) -> UiChanges {
        let mut changes = UiChanges {
            nodes_to_delete: HashSet::new(),
            nodes_to_reposition: HashMap::new(),
            pin_to_start_wire_creation: None,
            pin_to_finish_wire_creation: None,
            pins_to_disconnect: HashSet::new(),
        };

        for (_, node) in self.nodes.iter() {
            // Position
            let canvas_rect = egui::Rect::from_min_size(
                node.position,
                egui::vec2(
                    NODE_WIDTH,
                    NODE_HEADER_HEIGHT
                        + (node.inputs.len().max(node.outputs.len()) as f32 * PIN_ROW_HEIGHT),
                ),
            );
            let screen_rect = egui::Rect::from_min_max(
                canvas_to_screen(self.pan, self.zoom, canvas_rect.min),
                canvas_to_screen(self.pan, self.zoom, canvas_rect.max),
            );
            let padding = NODE_CORNER_RADIUS * self.zoom;
            let corner_radius = padding as u8;
            let header_rect = egui::Rect::from_min_size(
                screen_rect.min,
                egui::vec2(screen_rect.width(), NODE_HEADER_HEIGHT * self.zoom),
            );

            // Interact
            let header_response = ui.allocate_rect(
                header_rect,
                if alt_held {
                    egui::Sense::hover()
                } else {
                    egui::Sense::click_and_drag()
                },
            );
            if header_response.dragged() {
                changes.nodes_to_reposition.insert(
                    node.id,
                    node.position + header_response.drag_delta() / self.zoom,
                );
            }
            if header_response.secondary_clicked() {
                changes.nodes_to_delete.insert(node.id);
            }

            // Render
            painter.rect_filled(screen_rect, corner_radius, NODE_BACKGROUND_COLOR);
            painter.rect_filled(
                header_rect,
                egui::CornerRadius {
                    nw: corner_radius,
                    ne: corner_radius,
                    sw: 0,
                    se: 0,
                },
                NODE_HEADER_BACKGROUND_COLOR,
            );
            painter.rect_stroke(
                screen_rect,
                corner_radius,
                egui::Stroke::new(1.0, NODE_BORDER_COLOR),
                egui::StrokeKind::Middle,
            );
            painter.text(
                header_rect.left_center() + egui::vec2(padding, 0.0),
                egui::Align2::LEFT_CENTER,
                &node.label,
                egui::FontId::proportional(NODE_HEADER_FONT_SIZE * self.zoom),
                NODE_HEADER_TEXT_COLOR,
            );

            // Input pins
            for pin in node.inputs.iter() {
                let specific_pin = SpecificPin {
                    node_id: node.id,
                    pin_id: pin.id,
                    is_input: true,
                };
                let Some(pin_pos) = pin_positions.get(&specific_pin) else {
                    continue;
                };

                // Position
                let pin_rect = egui::Rect::from_center_size(
                    *pin_pos,
                    egui::vec2(PIN_RADIUS * 3.0 * self.zoom, PIN_RADIUS * 3.0 * self.zoom), // TODO: Why 3.0?
                );

                // Interact
                let pin_response = ui.allocate_rect(
                    pin_rect,
                    if alt_held {
                        egui::Sense::hover()
                    } else {
                        egui::Sense::click()
                    },
                );
                if pin_response.secondary_clicked() {
                    changes.pins_to_disconnect.insert(specific_pin);
                }

                // Render (and click, if valid wire finish)
                let pin_color = pin.kind.base.color();
                if let Some(wire_start) = self.wire_start
                    && pin_response.hovered()
                    && let Some(wire_start_node) = self.nodes.get(&wire_start.node_id)
                    && let Some(wire_start_pin) =
                        wire_start_node.get_pin_by_id(wire_start.pin_id, wire_start.is_input)
                {
                    painter.circle_filled(
                        *pin_pos,
                        PIN_RADIUS * self.zoom,
                        WIRE_PIN_HOVER_FILL_COLOR,
                    );

                    // TODO: Better cycle prevention
                    let stroke_color;
                    if changes.pin_to_finish_wire_creation.is_none()
                        && pin.kind == wire_start_pin.kind
                        && node.id != wire_start_node.id
                    {
                        stroke_color = WIRE_PIN_HOVER_VALID_STROKE_COLOR;
                        if pin_response.clicked() {
                            // TODO: Dragging release support
                            changes.pin_to_finish_wire_creation = Some(specific_pin);
                        }
                    } else {
                        stroke_color = WIRE_PIN_HOVER_INVALID_STROKE_COLOR;
                    }
                    painter.circle_stroke(
                        *pin_pos,
                        PIN_RADIUS * self.zoom,
                        egui::Stroke::new(PIN_STROKE_WIDTH * self.zoom, stroke_color),
                    );
                } else if self.wires.contains_key(&specific_pin) {
                    painter.circle_filled(*pin_pos, PIN_RADIUS * self.zoom, pin_color);
                } else {
                    painter.circle_stroke(
                        *pin_pos,
                        PIN_RADIUS * self.zoom,
                        egui::Stroke::new(PIN_STROKE_WIDTH * self.zoom, pin_color),
                    );
                }
                painter.text(
                    *pin_pos + egui::vec2(2.0 * padding, 0.0),
                    egui::Align2::LEFT_CENTER,
                    &pin.label,
                    egui::FontId::proportional(PIN_LABEL_FONT_SIZE * self.zoom),
                    PIN_LABEL_COLOR,
                );
            }

            // Output pins
            for pin in node.outputs.iter() {
                let specific_pin = SpecificPin {
                    node_id: node.id,
                    pin_id: pin.id,
                    is_input: false,
                };
                let Some(pin_pos) = pin_positions.get(&specific_pin) else {
                    continue;
                };

                // Position
                let pin_rect = egui::Rect::from_center_size(
                    *pin_pos,
                    egui::vec2(PIN_RADIUS * 3.0 * self.zoom, PIN_RADIUS * 3.0 * self.zoom), // TODO: Why 3.0?
                );

                // Interact
                let pin_response = ui.allocate_rect(
                    pin_rect,
                    if alt_held {
                        egui::Sense::hover()
                    } else {
                        egui::Sense::click_and_drag()
                    },
                );
                if pin_response.secondary_clicked() {
                    changes.pins_to_disconnect.insert(specific_pin);
                }
                if self.wire_start.is_none()
                    && changes.pin_to_start_wire_creation.is_none()
                    && pin_response.clicked()
                {
                    // TODO: Dragging and release of dragging
                    changes.pin_to_start_wire_creation = Some(specific_pin);
                }

                // Render
                let pin_color = pin.kind.base.color();
                if self.wires.values().any(|&sp| sp == specific_pin) {
                    painter.circle_filled(*pin_pos, PIN_RADIUS * self.zoom, pin_color);
                } else {
                    painter.circle_stroke(
                        *pin_pos,
                        PIN_RADIUS * self.zoom,
                        egui::Stroke::new(PIN_STROKE_WIDTH * self.zoom, pin_color),
                    );
                }
                painter.text(
                    *pin_pos - egui::vec2(2.0 * padding, 0.0),
                    egui::Align2::RIGHT_CENTER,
                    &pin.label,
                    egui::FontId::proportional(PIN_LABEL_FONT_SIZE * self.zoom),
                    PIN_LABEL_COLOR,
                );
            }
        }

        changes
    }

    fn draw_wire(
        &self,
        painter: &egui::Painter,
        from: SpecificPin,
        from_pos: egui::Pos2,
        to_pos: egui::Pos2,
    ) {
        let control_offset = egui::vec2(
            (from_pos.distance(to_pos) * CONTROL_GAP_DISTANCE_SCALING)
                .clamp(CONTROL_GAP_MIN, CONTROL_GAP_MAX),
            0.0,
        );
        let stroke_color = self
            .nodes
            .get(&from.node_id)
            .and_then(|n| n.get_pin_by_id(from.pin_id, from.is_input))
            .map(|p| p.kind.base.color())
            .unwrap_or(egui::Color32::WHITE);
        painter.add(egui::epaint::CubicBezierShape {
            points: [
                from_pos,
                from_pos + control_offset,
                to_pos - control_offset,
                to_pos,
            ],
            closed: false,
            fill: egui::Color32::TRANSPARENT,
            stroke: egui::Stroke::new(WIRE_WIDTH * self.zoom, stroke_color).into(),
        });
    }

    fn draw_wire_creation(
        &self,
        ui: &egui::Ui,
        painter: &egui::Painter,
        pin_positions: &HashMap<SpecificPin, egui::Pos2>,
    ) {
        if let Some(wire_start_pin) = self.wire_start
            && let Some(wire_start_pos) = pin_positions.get(&wire_start_pin)
            && let Some(mouse_pos) = ui.ctx().pointer_hover_pos()
        {
            self.draw_wire(&painter, wire_start_pin, *wire_start_pos, mouse_pos);
        }
    }

    fn draw_wires(
        &self,
        painter: &egui::Painter,
        pin_positions: &HashMap<SpecificPin, egui::Pos2>,
    ) {
        for (&to, &from) in self.wires.iter() {
            let Some(&from_pos) = pin_positions.get(&from) else {
                continue;
            };
            let Some(&to_pos) = pin_positions.get(&to) else {
                continue;
            };
            self.draw_wire(painter, from, from_pos, to_pos);
        }
    }
}

impl eframe::App for BlueprintEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            let alt_held = ui.input(|i| i.modifiers.alt);
            self.handle_pan_zoom(ui, &response, alt_held);
            self.handle_context_menu(&response);

            let pin_positions = self.compute_pin_positions();
            self.draw_wires(&painter, &pin_positions);
            let changes = self.draw_ui_nodes(ui, &painter, alt_held, &pin_positions);
            self.draw_wire_creation(ui, &painter, &pin_positions);
            self.handle_ui_changes(ui, changes);
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Video Processor",
        options,
        Box::new(|_cc| Ok(Box::<BlueprintEditor>::default())),
    )
}
