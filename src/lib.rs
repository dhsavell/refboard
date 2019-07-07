use yew::events::IMouseEvent;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

type Degrees = f64;

pub enum Scale {
    Proportional(f64),
    Independent(f64, f64),
}

pub struct Card {
    image: String,
    position: (i32, i32),
    scale: Scale,
    rotation: Degrees,
}

pub struct Model {
    console: ConsoleService,
    cards: Vec<Card>,
    drag_idx: Option<usize>,
}

pub enum Msg {
    CreateCard(String, (i32, i32)),
    RemoveCard(u32),
    StartDragging(usize),
    Drag((i32, i32)),
    StopDragging,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            cards: vec![Card {
                image: "".to_string(),
                position: (0, 0),
                rotation: 0.0,
                scale: Scale::Proportional(1.0),
            }],
            drag_idx: Option::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartDragging(idx) => {
                self.drag_idx = Option::Some(idx);
                true
            }
            Msg::Drag(delta) => match self.drag_idx {
                Some(idx) => {
                    self.cards[idx].position.0 += delta.0;
                    self.cards[idx].position.1 += delta.1;
                    true
                }
                None => false,
            },
            Msg::StopDragging => {
                self.drag_idx = Option::None;
                true
            }
            _ => true,
        }
    }
}

fn view_card(card: &Card) -> Html<Model> {
    html! {
        <div class="unselectable card",
                style=format!("left: {}px; top: {}px;", card.position.0, card.position.1),
                onmousedown=|_| Msg::StartDragging(0),>
            <div class="image",></div>
        </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="refboard",
                onmousemove=|e| Msg::Drag((e.movement_x(), e.movement_y())),
                onmouseup=|_| Msg::StopDragging,>
                { for self.cards.iter().map(view_card) }
            </div>
        }
    }
}
