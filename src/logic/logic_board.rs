use rand::prelude::*;
use std::collections::HashSet;

use yew::services::console::ConsoleService;

use array2d::Array2D;

use crate::cell::cell::*;

pub struct LogicBoard {
  width: usize,
  height: usize,
  mines: usize,
  board: Array2D<CellInfo>
}

impl LogicBoard {
  pub fn new_random(width: usize, height: usize, mines: usize) -> LogicBoard {
    let mut flat_board: Vec<CellInfo> = Vec::new();

    // Fill some mines
    for _n in 0..mines {
      flat_board.push(
        CellInfo { 
          value: CellValue::Mine, 
          state: CellState::Idle 
        }
      )
    }
  
    // Fill in the rest
    for _n in mines..(width*height) {
      flat_board.push(
        CellInfo { 
          value: CellValue::Proximity(0), 
          state: CellState::Idle 
        }
      )
    }
  
    //Shuffle 'em around
    let mut rng = rand::thread_rng();
    let n = (width as i32)*(height as i32) - 1;
    for i in 0..n {
      let j: usize = rng.gen_range(0, width*height - 1);
      flat_board.swap(i as usize, j as usize);
    }
  
    //Convert flat board into 2D board
    let board = Array2D::from_row_major(&flat_board, height, width);
    
    LogicBoard {
      width,
      height,
      mines,
      board
    }
  }

  pub fn init(self: &mut Self) {
    for x in 0..self.width {
      for y in 0..self.height {
        self.write_proximity(x, y);
      }
    }
  }

  fn get_cell(self: &Self, x: usize, y: usize) -> Option<&CellInfo> {
    self.board.get(y, x)
  }

  fn get_cell_mut(self: &mut Self, x: usize, y: usize) -> Option<&mut CellInfo> {
    self.board.get_mut(y, x)
  }

  fn checked_cell(self: &Self, x: usize, y: usize) -> Option<(usize, usize)> {
    self.get_cell(x, y).map(|_| (x, y))
  }

  fn get_proximity_cells(self: &Self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut points = HashSet::<(usize, usize)>::new();
    for i in 0..=1 {
      for j in 0..=1 {
        let vec_ref = &mut points;
        let x_add = x.checked_add(i);
        let x_sub = x.checked_sub(i);
        let y_add = y.checked_add(j);
        let y_sub = y.checked_sub(j);
        
        x_add
          .and_then(|x| y_add.map(|y| (x, y)))
          .and_then(|(x, y)| self.checked_cell(x, y))
          .map(|(x, y)| vec_ref.insert((x, y)));

        x_add
          .and_then(|x| y_sub.map(|y| (x, y)))
          .and_then(|(x, y)| self.checked_cell(x, y))
          .map(|(x, y)| vec_ref.insert((x, y)));

        x_sub
          .and_then(|x| y_add.map(|y| (x, y)))
          .and_then(|(x, y)| self.checked_cell(x, y))
          .map(|(x, y)| vec_ref.insert((x, y)));

        x_sub
          .and_then(|x| y_sub.map(|y| (x, y)))
          .and_then(|(x, y)| self.checked_cell(x, y))
          .map(|(x, y)| vec_ref.insert((x, y)));
      }
    }
    points.iter().copied().filter(|(x_i, y_i)| !(*x_i == x && *y_i == y)).collect()
  }

  fn clear_around(self: &mut Self, x: usize, y: usize) {
    let nearby_cells = self.get_proximity_cells(x, y);
    for (x, y) in nearby_cells {
      self.clear_cell(x, y);
    }
  }

  fn write_proximity(self: &mut Self, x: usize, y: usize) {
    let nearby_cells = self.get_proximity_cells(x, y);
    // ConsoleService::info(format!("[Board] Finding proximity cells for: {:?}", (x, y)).as_str());
    // ConsoleService::info(format!("[Board] Cells found: {:?}", nearby_cells).as_str());
    let mut proximity = 0u32;
    for (x_i, y_i) in nearby_cells {
      let cell = self
        .get_cell(x_i, y_i)
        .and_then(|cell| {
          match cell.value {
            CellValue::Mine => Some(true),
            _ => None
          }
        })
        .map(|_| proximity = proximity+1);
    }

    let mut cell = self.get_cell_mut(x, y).map(|cell| {
      match cell.value {
        CellValue::Proximity(_n) => {
          cell.value = CellValue::Proximity(proximity)
        },
        _ => {
          return
        }
      }
    });
  }

  pub fn clear_cell(self: &mut Self, x: usize, y: usize) {
    let cell_option = self.get_cell_mut(x, y);
    if cell_option.is_none() {
      return
    }

    let mut cell = cell_option.unwrap();

    match cell.state {
      CellState::Idle => cell.state = CellState::Cleared,
      _ => { return }
    }

    match cell.value {
      CellValue::Proximity(0) => self.clear_around(x, y),
      _ => ()
    }
  }

  pub fn flag_cell(self: &mut Self, x: usize, y: usize) -> i8 {
    ConsoleService::info(format!("[Board] Flagging cell: {:?}", (x, y)).as_str());

    let cell = self.get_cell_mut(x, y);
    
    cell.map_or(0, |cell| {
      match cell.state {
        CellState::Flagged => {
          cell.state = CellState::Idle;
          -1
        },
        CellState::Idle => {
          cell.state =CellState::Flagged;
          1
        },
        _ => 0
      }
    })
  }

  pub fn get_board_rows(self: &Self) -> Vec<Vec<CellInfo>> {
    self.board.as_rows()
  }
}