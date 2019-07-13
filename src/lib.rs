use std::cmp;
use stdweb::traits::IMouseEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

const HANDLE_RADIUS_PX: i32 = 5;
const MIN_CARD_SIZE_PX: i32 = 2 * HANDLE_RADIUS_PX + 1;

/// A Card is a transformable image displayed on the Refboard canvas.
#[derive(PartialEq)]
pub struct Card {
    /// The image displayed by this card. This value is directly used in HTML
    /// `img` tags.
    image: String,

    /// The absolute position of the top-left corner of this card, represented
    /// as an (x, y) tuple.
    position: (i32, i32),

    /// The absolute size of this card, represented as a (width, height) tuple.
    size: (i32, i32),

    /// The rotation of this card in degrees.
    rotation: f64,

    /// The Z-index of this card.
    z: i32,
}

impl Card {
    fn rotation_handle_angle(&self) -> f64 {
        let (width, height) = self.size;
        (height as f64).atan2(width as f64)
    }
}

/// A Model represents the state of the webapp.
pub struct Model {
    /// A vector of all cards on the Refboard canvas.
    cards: Vec<Card>,

    /// The current action bound to mouse movement.
    drag_state: DragState,
}

/// A DragState represents an action controlled by holding down the left mouse
/// button and moving the mouse.
pub enum DragState {
    /// Mouse movement should be ignored.
    None,

    /// The card with the given index should be moved with the cursor.
    MoveCard(usize),

    /// The card with the given index should be scaled from the bottom-right.
    MoveScaleHandle(usize),

    /// The card with the given index should be rotated about its center.
    MoveRotateHandle(usize),
}

/// A Msg (Message) is a signal sent to the Model requesting a controlled state
/// change.
pub enum Msg {
    CreateCard(String, (i32, i32)),
    RemoveCard(u32),
    ResetRotation(usize),
    StartDraggingScaleHandle(usize),
    StartDraggingRotateHandle(usize),
    StartDraggingCard(usize),
    Drag((i32, i32), (i32, i32)),
    StopDragging,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            cards: vec![
                Card {
                    image: "".to_string(),
                    position: (0, 0),
                    size: (300, 300),
                    rotation: 0.0,
                    z: 0,
                },
                Card {
                    image: "".to_string(),
                    position: (400, 0),
                    size: (300, 300),
                    rotation: 0.0,
                    z: 1,
                },
            ],
            drag_state: DragState::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartDraggingCard(idx) => {
                self.drag_state = DragState::MoveCard(idx);
                let selected_card_z = self.cards[idx].z;

                for mut card in &mut self.cards {
                    if card.z >= selected_card_z {
                        card.z -= 1;
                    }
                }

                self.cards[idx].z = (self.cards.len() - 1) as i32;

                true
            }
            Msg::StartDraggingScaleHandle(idx) => {
                self.drag_state = DragState::MoveScaleHandle(idx);
                true
            }
            Msg::StartDraggingRotateHandle(idx) => {
                self.drag_state = DragState::MoveRotateHandle(idx);
                true
            }
            Msg::ResetRotation(idx) => {
                self.cards[idx].rotation = 0.0;
                true
            }
            Msg::Drag(delta, pos) => match self.drag_state {
                DragState::MoveCard(idx) => {
                    let card = &mut self.cards[idx];

                    card.position.0 += delta.0;
                    card.position.1 += delta.1;

                    true
                }
                DragState::MoveScaleHandle(idx) => {
                    let card = &mut self.cards[idx];

                    card.size.0 = cmp::max(MIN_CARD_SIZE_PX, card.size.0 + delta.0);
                    card.size.1 = cmp::max(MIN_CARD_SIZE_PX, card.size.1 + delta.1);

                    true
                }
                DragState::MoveRotateHandle(idx) => {
                    let card = &mut self.cards[idx];
                    let (cursor_x, cursor_y) = pos;
                    let (x, y) = card.position;
                    let (width, height) = card.size;

                    let atan_x: f64 = (cursor_x - (x + (width / 2))).into();
                    let atan_y: f64 = (cursor_y - (y + (height / 2))).into();

                    card.rotation = atan_y.atan2(atan_x) + card.rotation_handle_angle();

                    true
                }
                DragState::None => false,
            },
            Msg::StopDragging => {
                self.drag_state = DragState::None;
                true
            }
            _ => true,
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="refboard",
                    onmousemove=|e| Msg::Drag((e.movement_x(), e.movement_y()), (e.client_x(), e.client_y())),
                    onmouseup=|_| Msg::StopDragging,>
                { for self.cards.iter().map(|c| self.view_card(c)) }
            </div>
        }
    }
}

impl Model {
    fn view_card(&self, card: &Card) -> Html<Model> {
        let card_idx = self.cards.iter().position(|c| c == card);

        match card_idx {
            Some(idx) => html! {
                <div class="unselectable card",
                        style=format!("left: {}px; top: {}px; width: {}px; height: {}px; transform: rotate({}rad); z-index: {};", card.position.0, card.position.1, card.size.0, card.size.1, card.rotation, card.z),>

                    // Transformation handles

                    <div class="scaling-handle",
                        style=format!("right: -{}px; bottom: -{}px;", HANDLE_RADIUS_PX, HANDLE_RADIUS_PX),
                        onmousedown=|_| Msg::StartDraggingScaleHandle(idx),
                        ondragstart=|_| Msg::StartDraggingScaleHandle(idx),></div>

                    <div class="rotation-handle",
                        style=format!("right: -{}px; top: -{}px;", HANDLE_RADIUS_PX, HANDLE_RADIUS_PX),
                        onmousedown=|_| Msg::StartDraggingRotateHandle(idx),
                        ondragstart=|_| Msg::StartDraggingRotateHandle(idx),
                        oncontextmenu=|_| Msg::ResetRotation(idx),></div>

                    // Actual image body

                    <div class="image",
                        onmousedown=|_| Msg::StartDraggingCard(idx),
                        ondragstart=|_| Msg::StartDraggingCard(idx),
                        style=format!("width: {}px; height: {}px", card.size.0, card.size.1),></div>
                </div>
            },
            None => html! {},
        }
    }
}
