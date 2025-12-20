use embassy_rp::i2c::Instance;
use embassy_time::Timer;

use crate::display::DisplayController;

static ROWS_WORD: [u8; 5] = [0b01010010, 0b01101111, 0b01110111, 0b01110011, 0b00111010];

pub struct CountingMode<'a, T: Instance> {
    display_controller: DisplayController<'a, T>,
    rows: u8,
}

impl<'a, T: Instance> CountingMode<'a, T> {
    pub async fn enter(mut display_controller: DisplayController<'a, T>) -> Self {
        display_controller.clear_display();
        display_controller.return_home();
        display_controller.set_backlight_color(0, 255, 0);

        Timer::after_millis(3).await;
        for character in ROWS_WORD {
            display_controller.write_character(character);
        }

        display_controller
            .write_character_at_coord(0b00110000, (5, 0));

        Self {
            display_controller,
            rows: 0,
        }
    }

    async fn update_display(&mut self) {
        self.display_controller
            .write_character_at_coord(0b00110000 + self.rows, (5, 0));
    }

    pub async fn increment_count(&mut self) {
        self.rows += 1;
        self.update_display().await;
    }

    pub async fn decrement_count(&mut self) {
        self.rows -= 1;
        self.update_display().await;
    }

    pub async fn reset_count(&mut self) {
        self.rows = 0;
        self.update_display().await;
    }
}
