use yew::prelude::*;
use yew::ComponentLink;
use yew::Properties;
use yew::services::console::ConsoleService;

#[derive(Clone, Copy, Debug)]
pub enum CellState {
  Idle,
  Flagged,
  Cleared
}

pub struct CellInfo {
  pub value: CellValue,
  pub state: CellState
}

#[derive(Debug)]
pub enum CellAction {
  Clear,
  Flag
}

#[derive(Clone, Copy, Debug)]
pub enum CellValue {
  Proximity(u32),
  Mine
}

pub enum Msg {
  Click,
  RightClick
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
  pub value: CellValue,
  pub state: CellState,
  pub action_callback: Callback<CellAction>
}

pub struct Cell {
  link: ComponentLink<Self>,
  props: Props
}

impl Cell {
  fn symbol(self: &Cell) -> String {
    ConsoleService::info(format!("Getting symbol for: {:?}", self.props).as_str());
    match self.props.state {
      CellState::Flagged => "🚩".to_string(),
      CellState::Idle => " ".to_string(),
      CellState::Cleared => match self.props.value {
        CellValue::Proximity(0) => "butt".to_string(),
        CellValue::Proximity(m) => m.to_string(),
        CellValue::Mine => "💣".to_string()
      }
    }
  }
}

impl Component for Cell {
  type Properties = Props;
  type Message = Msg;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Cell { 
      props, 
      link
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Click => {
        self.props.action_callback.emit(CellAction::Clear);
        true
      },
      Msg::RightClick => {
        self.props.action_callback.emit(CellAction::Flag);
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn view(&self) -> Html {
    let onclick = self.link.callback(|_: MouseEvent| Msg::Click);
    let onrightclick = self.link.callback(|e: MouseEvent| {
      e.prevent_default();
      Msg::RightClick
    });

    let symbol = self.symbol();

    ConsoleService::info(format!("Render: {:?}", symbol).as_str());


    html! {
      <button onclick=onclick oncontextmenu=onrightclick>{ symbol }</button>
    }
  }
}