use gemini_engine::{
    containers::{CanCollide, CollisionContainer, PixelContainer},
    core::Vec2D,
};
mod blocks;
pub use blocks::{block_manipulation as tetris_core, Block, BlockType};
use rand::Rng;

pub struct BlockManager {
    bag: Vec<BlockType>,
    pub block: Block,
    pub ghost_block: Block,
    pub held_piece: Option<BlockType>,
    pub has_held: bool,
    pub placing_cooldown: u32,
    // Constants
    piece_preview_count: usize,
    block_place_cooldown: u32,
}

impl BlockManager {
    pub fn new(block_place_cooldown: u32, piece_preview_count: usize) -> Self {
        let mut tmp = Self {
            bag: BlockType::bag()[0..rand::rng().random_range(1..8)].to_vec(),
            block: Block::DEFAULT,
            ghost_block: Block::DEFAULT,
            held_piece: None,
            has_held: false,
            placing_cooldown: block_place_cooldown,
            block_place_cooldown,
            piece_preview_count,
        };
        tmp.generate_new_block();
        tmp
    }

    pub fn reset_placing_cooldown(&mut self) {
        self.placing_cooldown = self.block_place_cooldown;
    }

    /// Call when a block is placed
    ///
    /// Returns true if the block is placed at the very top of the board, indicating a lost game
    pub fn reset(&mut self) -> bool {
        self.reset_placing_cooldown();
        self.has_held = false;
        self.block.pos.y < 1
    }

    pub fn generate_new_block(&mut self) {
        let next_piece = self.bag.pop().unwrap_or_else(|| unreachable!());
        if self.bag.len() <= self.piece_preview_count {
            let mut new_bag = BlockType::bag().to_vec();
            new_bag.extend(&self.bag);
            self.bag.clear();
            self.bag.extend(new_bag);
        }

        self.block = Block::new(next_piece);
    }

    /// Attempt to move the block. Resets the placing cooldown and returns true if successful
    pub fn try_move_block(&mut self, collision: &CollisionContainer, offset: Vec2D) -> bool {
        let did_move = tetris_core::try_move_block(collision, &mut self.block, offset);
        if did_move {
            self.reset_placing_cooldown();
        }
        did_move
    }

    /// Attempt to rotate the block. Resets the placing cooldown and returns true if successful
    pub fn try_rotate_block(&mut self, collision: &CollisionContainer, clockwise: bool) {
        let did_rotate = tetris_core::try_rotate_block(collision, &mut self.block, clockwise);
        if did_rotate {
            self.reset_placing_cooldown();
        }
    }

    /// Hold the current block
    pub fn hold(&mut self) {
        if !self.has_held {
            let current_held_piece = self.held_piece;
            self.held_piece = Some(self.block.shape);
            match current_held_piece {
                Some(piece) => self.block = Block::new(piece),
                None => {
                    self.generate_new_block();
                }
            }
            self.has_held = true;
        }
    }

    pub fn generate_ghost_block(&mut self, collision: &CollisionContainer) {
        let mut ghost_block = self.block.clone();
        ghost_block.is_ghost = true;

        while tetris_core::try_move_block(collision, &mut ghost_block, Vec2D::new(0, 1)) {}

        self.ghost_block = ghost_block;
    }

    /// Generate an appropriate alert
    pub fn check_for_t_spin(
        &self,
        collision: &impl CanCollide,
        cleared_lines: i64,
    ) -> Option<(i64, String)> {
        tetris_core::handle_t_spin(collision, &self.block, cleared_lines)
    }

    pub fn next_piece_display(&self) -> PixelContainer {
        let mut container = PixelContainer::new();
        for i in 0..self.piece_preview_count {
            let mut next_block_display = Block::new(self.bag[self.bag.len() - i - 1]);
            next_block_display.pos = Vec2D::new(15, 12 + i as i64 * 3);
            container.draw(&next_block_display);
        }

        container
    }

    pub const fn held_piece_display(&self) -> Option<Block> {
        if let Some(piece) = self.held_piece {
            let mut held_block_display = Block::new(piece);
            held_block_display.pos = Vec2D::new(15, 4);
            Some(held_block_display)
        } else {
            None
        }
    }
}
