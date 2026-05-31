//! Common utility methods for Platform Editor implementations.

/// Returns the number of digits of a `usize`.
pub fn digit_count(int: usize) -> usize {
    if int == 0 {
        return 1;
    }
    let mut number = int;
    let mut digits = 0;
    while number > 0 {
        number /= 10;
        digits += 1;
    }
    digits
}

pub struct ScrollInfo<'a> {
    pub held: bool,
    pub last_pos: &'a mut Option<f32>,
    pub pos: f32,
    pub velocity: &'a mut f32,
    pub scroll: &'a mut f32,
    pub delta_seconds: f32,

    pub starting_level_select_scroll: f32,
    pub spacing: f32,
    pub levels: usize,
    pub levels_per_row: usize,
    pub extra_end_scroll: f32
}

pub fn scroll(info: ScrollInfo<'_>) {
    if info.held {
        if let Some(prev_pos) = info.last_pos {
            let delta = info.pos - *prev_pos;
            *info.velocity = delta;
        }
        *info.last_pos = Some(info.pos);
    } else {
        let max_scroll = info.starting_level_select_scroll - info.spacing * (info.levels.div_ceil(info.levels_per_row) - 2).max(0) as f32 - info.extra_end_scroll;
        // Push the scroll towards the level buttons if it is dragged out of bounds.
        let (drag_value, out_by): (f32, f32) =
            if *info.scroll > info.starting_level_select_scroll {
                *info.velocity -= info.delta_seconds * 100.0;
                (
                    0.01,
                    info.starting_level_select_scroll - *info.scroll,
                )
            } else if *info.scroll < max_scroll {
                *info.velocity += info.delta_seconds * 100.0;
                (0.01, max_scroll - *info.scroll)
            } else {
                (0.01, 0.0)
            };
        *info.velocity *= drag_value.powf(info.delta_seconds);
        // Max out the velocity if needed.
        *info.velocity = info.velocity.clamp(-10.0 + out_by / 10.0, 10.0 + out_by / 10.0);
        *info.last_pos = None;
    }
    *info.scroll += *info.velocity * info.delta_seconds * 40.0;
}