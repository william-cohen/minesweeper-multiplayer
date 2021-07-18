use yew::prelude::*;
use yew::ComponentLink;

mod board;
use board::Board;

enum Msg {
  AddOne,
}

struct GameModel {
  _link: ComponentLink<Self>,
  height: usize,
  width: usize,
  mines: usize
}

impl Component for GameModel {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      _link: link,
      height: 16,
      width: 16,
      mines: 40
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    // Should only return "true" if new properties are different to
    // previously received properties.
    // This component has no properties so we will always return "false".
    false
  }

  fn view(&self) -> Html {
    html! {
      <div class=classes!("game")>
        <Board height={self.height} width={self.width} mines={self.mines} />
      </div>
    }
  }
}

fn main() {
  yew::start_app::<GameModel>();
}