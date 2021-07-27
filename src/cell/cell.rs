use yew::prelude::*;
use yew::ComponentLink;
use yew::Properties;

use crate::cell::icons::flag::Flag;
use crate::cell::icons::bomb::Bomb;


#[derive(Clone, Copy, Debug)]
pub enum CellState {
  Idle,
  Flagged,
  Cleared
}

#[derive(Clone, Debug)]
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
  fn symbol(self: &Cell) -> Html {
    // ConsoleService::info(format!("Getting symbol for: {:?}", self.props).as_str());
    match self.props.state {
      CellState::Flagged => html! { <Flag /> },
      CellState::Idle => html! { " " },
      CellState::Cleared => match self.props.value {
        CellValue::Proximity(0) => html! { " " },
        CellValue::Proximity(m) => html! { m.to_string() },
        CellValue::Mine => html! { <Bomb /> }
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
    let mut cell_classes = Vec::<String>::new();
    cell_classes.push(String::from("cell"));

    cell_classes.push(
      match self.props.state {
      CellState::Flagged => "flagged".to_string(),
      CellState::Idle => "".to_string(),
      CellState::Cleared => {
        match self.props.value {
          CellValue::Proximity(_n) => "cleared".to_string(),
          CellValue::Mine => "mine".to_string()
        }
      }
    });

    // ConsoleService::info(format!("Render: {:?}", symbol).as_str());


    html! {
      <button class=classes!(cell_classes) onclick=onclick oncontextmenu=onrightclick>{ symbol }</button>
    }
  }
}