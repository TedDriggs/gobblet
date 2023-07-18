use ansi_escapes::{CursorTo, EraseScreen};

/// Erase the screen and move the cursor to the top left.
pub fn reset_screen() {
    print!("{}{}", EraseScreen, CursorTo::TopLeft);
}
