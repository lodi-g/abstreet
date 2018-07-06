use ezgui::canvas::Canvas;
use ezgui::input::UserInput;
use ezgui::text_box::TextBox;
use map_model::{Map, RoadID, IntersectionID, BuildingID, ParcelID, geometry};
use piston::input::Key;
use piston::window::Size;
use plugins::selection::SelectionState;
use std::usize;

pub enum WarpState {
    Empty,
    EnteringSearch(TextBox),
}

impl WarpState {
    pub fn event(
        self,
        input: &mut UserInput,
        map: &Map,
        canvas: &mut Canvas,
        window_size: &Size,
        selection_state: &mut SelectionState,
    ) -> WarpState {
        match self {
            WarpState::Empty => {
                if input.unimportant_key_pressed(
                    Key::J,
                    "Press J to start searching for something to warp to",
                ) {
                    WarpState::EnteringSearch(TextBox::new())
                } else {
                    self
                }
            }
            WarpState::EnteringSearch(mut tb) => {
                if tb.event(input.use_event_directly()) {
                    input.consume_event();
                    warp(tb.line, map, canvas, window_size, selection_state);
                    WarpState::Empty
                } else {
                    input.consume_event();
                    WarpState::EnteringSearch(tb)
                }
            }
        }
    }

    pub fn get_osd_lines(&self) -> Vec<String> {
        // TODO draw the cursor
        if let WarpState::EnteringSearch(text_box) = self {
            return vec![text_box.line.clone()];
        }
        Vec::new()
    }
}

fn warp(
    line: String,
    map: &Map,
    canvas: &mut Canvas,
    window_size: &Size,
    selection_state: &mut SelectionState,
) {
    let pt = match usize::from_str_radix(&line[1 .. line.len()], 10) {
        Ok(idx) => {
            match line.chars().next().unwrap() {
                'r' => {
                    let id = RoadID(idx);
                    *selection_state = SelectionState::SelectedRoad(id, None);
                    map.get_r(id).first_pt()
                }
                'i' => {
                    let id = IntersectionID(idx);
                    *selection_state = SelectionState::SelectedIntersection(id);
                    map.get_i(id).point
                }
                'b' => {
                    let id = BuildingID(idx);
                    *selection_state = SelectionState::SelectedBuilding(id);
                    geometry::center(&map.get_b(id).points)
                }
                'p' => {
                    let id = ParcelID(idx);
                    geometry::center(&map.get_p(id).points)
                }
                _ => {
                    println!("{} isn't a valid ID; Should be [ribp][0-9]+", line);
                    return;
                }
            }
        }
        Err(_) => {
            println!("{} isn't a valid ID; Should be [ribp][0-9]+", line);
            return;
        }
    };
    canvas.center_on_map_pt(pt.x(), pt.y(), window_size);
}
