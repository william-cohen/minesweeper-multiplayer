use yew::prelude::*;
use yew::ComponentLink;
use yew::Properties;
use yew::services::console::ConsoleService;

mod cell;
use cell::{Cell, CellInfo, CellState, CellValue, CellAction};

#[derive(Properties, Clone)]
pub struct Props {
  pub height: usize,
  pub width: usize,
  pub mines: usize
}

#[derive(Debug)]
pub enum Msg {
  CellMsg(usize, usize, CellAction)
}


pub struct Board {
  link: ComponentLink<Self>,
  props: Props,
  board: Vec<Vec<CellInfo>>
}

// fn createRandomCellState

fn createBoard(height: usize, width: usize, mines: usize) -> Vec<Vec<CellInfo>> {
  let mut board: Vec<Vec<CellInfo>> = Vec::new();
  board.push(vec![ 
    CellInfo { value: CellValue::Proximity(0), state: CellState::Idle }, 
    CellInfo { value: CellValue::Mine, state: CellState::Idle }, 
    CellInfo { value: CellValue::Proximity(0), state: CellState::Idle } 
  ]);
  board.push(vec![ 
    CellInfo { value: CellValue::Mine, state: CellState::Idle }, 
    CellInfo { value: CellValue::Proximity(0), state: CellState::Idle }, 
    CellInfo { value: CellValue::Proximity(0), state: CellState::Idle } 
  ]);
  board.push(vec![ 
    CellInfo { value: CellValue::Mine, state: CellState::Idle }, 
    CellInfo { value: CellValue::Mine, state: CellState::Idle }, 
    CellInfo { value: CellValue::Proximity(0), state: CellState::Idle } 
  ]);
  
  board
}

impl Component for Board {
  type Properties = Props;
  type Message = Msg;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      props,
      board: createBoard(0, 0 ,0)
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    ConsoleService::info(format!("Recieved message: {:?}", msg).as_str());
    match msg {
      Self::Message::CellMsg(x, y, action) => match action {
        CellAction::Clear => {
          self.board[y][x].state = CellState::Cleared
        },
        CellAction::Flag => {
          self.board[y][x].state = CellState::Flagged
        }
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <div class="board">
        <div class="game-info">
          <span class="info"></span>
          <br />
          <span class="info"></span>
        </div>
        { 
          self.board.iter().enumerate().map(move |(y, row)| {
            html! {
              <div>
              {
                row.iter().enumerate().map(move |(x, cell)| 
                  html! { 
                    <Cell value={cell.value} state={cell.state} action_callback={self.link.callback(move |action: CellAction| Msg::CellMsg(x, y, action))} />
                  }
                ).collect::<Html>()
              }
              </div>
            }
          }).collect::<Html>()
        }
      </div>
    }
  }
}