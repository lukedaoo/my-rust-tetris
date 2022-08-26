const SCORE: [i32; 4] = [100, 300, 500, 800];

pub fn calculate_score(line_cleared: i32) -> u32 {
    if line_cleared == 0 {
        return 0;
    }
    SCORE[line_cleared as usize - 1] as u32
}
