use macroquad::{prelude::*, ui::{hash, root_ui, widgets, Skin}};

const PADDLE_W: f32 = 20.0;
const PADDLE_H: f32 = 100.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SIZE: f32 = 20.0;

#[macroquad::main("Wasm Pong")]
async fn main() {
    let player_x = 10.0;
    let mut player_y = screen_height() / 2.0 - PADDLE_H / 2.0;

    let bot_x = screen_width() - 10.0 - PADDLE_W;
    let mut bot_y = screen_height() / 2.0 - PADDLE_H / 2.0;

    let mut ball_x = screen_width() / 2.0 - BALL_SIZE / 2.0;
    let mut ball_y = screen_height() / 2.0 - BALL_SIZE / 2.0;

    let mut ball_speed_x = 150;
    let mut ball_speed_y = rand::gen_range(-50, 50);

    let mut score = 0;
    let mut bot_score = 0;
    let mut bot_mode = false;
    let mut button_text: &str;

    loop {
        // Set the background color
        clear_background(BLACK);

        // Get the time since the last frame for physics
        let delta = get_frame_time();

        // Draw the player paddle
        draw_rectangle(player_x, player_y, PADDLE_W, PADDLE_H, GREEN);

        // Draw the bot paddle
        draw_rectangle(bot_x, bot_y, PADDLE_W, PADDLE_H, GREEN);

        // Draw the ball
        draw_rectangle(ball_x, ball_y, BALL_SIZE, BALL_SIZE, GREEN);

        // Draw the famous line
        draw_line(
            screen_width() / 2.0,
            0.0,
            screen_width() / 2.0,
            screen_height(),
            2.0,
            GREEN,
        );

        // Check for input
        for touch in touches() {
            let pos = touch.position;
            if bot_mode {
                player_y = pos.y - PADDLE_H / 2.0;
                // Check for cheating
                if player_y < 0. {
                    player_y = 0.;
                }
                if player_y > screen_height() - PADDLE_H {
                    player_y = screen_height() - PADDLE_H;
                }
            } else {
                if pos.x < screen_width() / 2.0 {
                    player_y = pos.y - PADDLE_H / 2.0;
                    // Check for cheating
                    if player_y < 0. {
                        player_y = 0.;
                    }
                    if player_y > screen_height() - PADDLE_H {
                        player_y = screen_height() - PADDLE_H;
                    }
                } else {
                    bot_y = pos.y - PADDLE_H / 2.0;
                    // Check for cheating
                    if bot_y < 0. {
                        bot_y = 0.;
                    }
                    if bot_y > screen_height() - PADDLE_H {
                        bot_y = screen_height() - PADDLE_H;
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if bot_mode {
                player_y = mouse_position().1 - PADDLE_H / 2.0;
                // Check for cheating
                if player_y < 0. {
                    player_y = 0.;
                }
                if player_y > screen_height() - PADDLE_H {
                    player_y = screen_height() - PADDLE_H;
                }
            } else {
                if mouse_position().0 < screen_width() / 2.0 {
                    player_y = mouse_position().1 - PADDLE_H / 2.0;
                    // Check for cheating
                    if player_y < 0. {
                        player_y = 0.;
                    }
                    if player_y > screen_height() - PADDLE_H {
                        player_y = screen_height() - PADDLE_H;
                    }
                } else {
                    bot_y = mouse_position().1 - PADDLE_H / 2.0;
                    // Check for cheating
                    if bot_y < 0. {
                        bot_y = 0.;
                    }
                    if bot_y > screen_height() - PADDLE_H {
                        bot_y = screen_height() - PADDLE_H;
                    }
                }
            }
        }

        if is_key_down(KeyCode::Up) {
            player_y -= PADDLE_SPEED * delta;
            // Check for cheating
            if player_y < 0. {
                player_y = 0.;
            }
        } else if is_key_down(KeyCode::Down) {
            player_y += PADDLE_SPEED * delta;
            if player_y > screen_height() - PADDLE_H {
                player_y = screen_height() - PADDLE_H;
            }
        }

        if !bot_mode {
            if is_key_down(KeyCode::W) {
                bot_y -= PADDLE_SPEED * delta;
                // Check for cheating
                if bot_y < 0. {
                    bot_y = 0.;
                }
            } else if is_key_down(KeyCode::S) {
                bot_y += PADDLE_SPEED * delta;
                if bot_y > screen_height() - PADDLE_H {
                    bot_y = screen_height() - PADDLE_H;
                }
            }
        }

        // Update Ball position
        ball_x += ball_speed_x as f32 * delta;
        ball_y += ball_speed_y as f32 * delta;

        // Update bot paddle position
        if bot_mode {
            bot_y = ball_y + BALL_SIZE / 2.0 - PADDLE_H / 2.0;
            // Check if the bot is cheating
            if bot_y < 0. {
                bot_y = 0.;
            }

            if bot_y > screen_height() - PADDLE_H {
                bot_y = screen_height() - PADDLE_H;
            }
        }

        
        // Check for ball collision with walls (uwu)
        if ball_x > screen_width() - BALL_SIZE {
            score += 1;
            ball_x = screen_width() / 2.0 - BALL_SIZE / 2.0;
            ball_y = screen_height() / 2.0 - BALL_SIZE / 2.0;
            ball_speed_x = 150;
            ball_speed_y = rand::gen_range(-150, 150);
        } else if ball_x < 0.0 {
            bot_score += 1;
            ball_x = screen_width() / 2.0 - BALL_SIZE / 2.0;
            ball_y = screen_height() / 2.0 - BALL_SIZE / 2.0;
            ball_speed_x = 150;
            ball_speed_y = rand::gen_range(-150, 150);
        }

        if ball_y > screen_height() - BALL_SIZE {
            ball_y = screen_height() - BALL_SIZE;
            ball_speed_y = -ball_speed_y;
        } else if ball_y < 0.0 {
            ball_y = 0.0;
            ball_speed_y = -ball_speed_y;
        }

        // Check for bang bang with player
        if check_paddle_col_player(ball_x, ball_y, player_x, player_y) {
            ball_speed_x = {
                if ball_speed_x > 0 {
                    ball_speed_x + 50
                } else {
                    -ball_speed_x + 50
                }
            };
        }

        // Check for bang bang with bot
        if check_paddle_col_bot(ball_x, ball_y, bot_x, bot_y) {
            ball_speed_x = {
                if ball_speed_x > 0 {
                    -ball_speed_x - 50
                } else {
                    ball_speed_x + 50
                }
            };
        }

        // Customize button style with green border
        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("wasm-pong_button_background.png"),
                     None
                    ).unwrap())
            .text_color(Color::from_rgba(0, 255, 0, 255))
            .text_color_hovered(Color::from_rgba(0, 255, 0, 255))
            .text_color_clicked(Color::from_rgba(0, 255, 0, 255))
            .font_size(20)
            .build();

        let button_skin = Skin {
            button_style: button_style,
            ..root_ui().default_skin()
        };

        if bot_mode {
            button_text = "2-Player Mode";
        } else {
            button_text = "1-Player Mode";
        }

        // Mode toggle button
        root_ui().push_skin(&button_skin);
        if root_ui().button(vec2(50.0, 10.0), button_text) {
            bot_mode = !bot_mode;
            score = 0;
            bot_score = 0;
        };
        root_ui().pop_skin();

        // Draw the score
        let text_player = score.to_string();
        let text_bot = bot_score.to_string();
        let text_dimensions_bot = measure_text(&text_bot, None, 40, 1.0);
        let text_x_player = screen_width() / 4.0;
        let text_x_bot = screen_width() - text_dimensions_bot.width - 10.0 - screen_width() / 4.0;
        draw_text(&text_player, text_x_player, 50.0, 40.0, GREEN);
        draw_text(&text_bot, text_x_bot, 50.0, 40.0, GREEN);

        next_frame().await;
    }
}

fn check_paddle_col_player(ball_x: f32, ball_y: f32, player_x: f32, player_y: f32) -> bool {
    player_x + PADDLE_W > ball_x && player_y < ball_y && player_y + PADDLE_H > ball_y
}

fn check_paddle_col_bot(ball_x: f32, ball_y: f32, bot_x: f32, bot_y: f32) -> bool {
    bot_x - PADDLE_W < ball_x && bot_y < ball_y && bot_y + PADDLE_H > ball_y
}


