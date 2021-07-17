use yew::prelude::*;
use yew::ComponentLink;
use yew::Properties;
use yew::services::console::ConsoleService;

use yew::virtual_dom::VNode::*;
use yew::virtual_dom::VList;

mod cell;
use cell::{Cell, CellInfo, CellState, CellAction};
mod logic_board;
use logic_board::LogicBoard;

#[derive(Properties, Copy, Clone)]
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
  board: LogicBoard
}

impl Board {
  pub fn render_cell_row(&self, y: usize, row: &Vec<CellInfo>) -> Html {
    let mut cells_html = VList::new();
    let mut x: usize = 0;
    for cell in row {
      let cell_html = html! {
        <Cell value={cell.value} state={cell.state} action_callback={self.link.callback(move |action: CellAction| Msg::CellMsg(x, y, action))} />
      };
      cells_html.add_child(cell_html);
      x = x + 1;
    }
    VList(cells_html)
  }

  pub fn render_cell_table(&self) -> Html {
    let mut rows_html = VList::new();
    let mut y: usize = 0;
    for row in self.board.get_board_rows() {
      let row_html = html! {
        <div>
        {
          self.render_cell_row(y, &row)
        }
        </div>
      };
      rows_html.add_child(row_html);
      y = y + 1;
    }
    VList(rows_html)
  }
}

impl Component for Board {
  type Properties = Props;
  type Message = Msg;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let mut board = LogicBoard::new_random(props.height, props.width, props.mines);
    board.init();
    Self {
      link,
      props,
      board
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    ConsoleService::info(format!("[Board] Received message: {:?}", msg).as_str());
    match msg {
      Self::Message::CellMsg(x, y, action) => match action {
        CellAction::Clear => {
          // self.board[y][x].state = CellState::Cleared
          self.board.clear_cell(x, y);
        },
        CellAction::Flag => {
          self.board.flag_cell(x, y);
        }
      }
    }
    true
    // false
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
          self.render_cell_table()
        }
      </div>
    }
  }
}