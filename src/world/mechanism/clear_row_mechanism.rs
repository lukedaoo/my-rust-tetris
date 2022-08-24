use std::collections::{hash_map::RandomState, HashMap, HashSet};

use crate::world::World;

impl World {
    pub fn clear_rows_and_fall_other_pieces_down(&mut self, row_has_full_pieces: &HashSet<u32>) {
        if row_has_full_pieces.is_empty() {
            return;
        }
        self.clear_rows(&row_has_full_pieces);
        self.fall_down_other_pieces(&row_has_full_pieces);
    }

    fn clear_rows(&mut self, row_has_full_pieces: &HashSet<u32>) {
        let mut current_brick_index_in_stack = 0;

        while current_brick_index_in_stack != self.stack.len() {
            let mut current_coord_index = 0;

            while current_coord_index != self.stack[current_brick_index_in_stack].coords().len() {
                let row = self.stack[current_brick_index_in_stack].coords()[current_coord_index].y;

                if row_has_full_pieces.contains(&row) {
                    self.stack[current_brick_index_in_stack]
                        .coords_mut()
                        .remove(current_coord_index);
                } else {
                    current_coord_index += 1;
                }
            }

            // when there are no coord of current brick. just remove brick from stack
            if self.stack[current_brick_index_in_stack].coords().is_empty() {
                self.stack.remove(current_brick_index_in_stack);
            } else {
                current_brick_index_in_stack += 1;
            }
        }
    }

    fn fall_down_other_pieces(&mut self, row_has_full_pieces: &HashSet<u32>) {
        let mut diff = vec![0; self.dimension.v() as usize];
        for row in row_has_full_pieces.iter() {
            for num in diff.iter_mut().skip(*row as usize) {
                *num += 1;
            }
        }

        for i in 0..self.stack.len() {
            for j in 0..self.stack[i].coords().len() {
                self.stack[i].coords_mut()[j].y -= diff[self.stack[i].coords()[j].y as usize];
            }
        }
    }

    pub fn find_rows_has_full_pieces(&mut self) -> HashSet<u32, RandomState> {
        self.number_of_piece_per_row()
            .iter()
            .filter_map(|e| {
                if *e.1 == self.dimension.h_size as u32 {
                    return Some(*e.0);
                } else {
                    None
                }
            })
            .collect::<HashSet<u32>>()
    }

    fn number_of_piece_per_row(&self) -> HashMap<u32, u32> {
        let mut pieces_per_row: HashMap<u32, u32> = HashMap::new();

        for brick in self.stack.iter() {
            for coord in brick.coords() {
                let e = pieces_per_row.entry(coord.y).or_insert(0);
                *e += 1;
            }
        }

        pieces_per_row
    }
}
