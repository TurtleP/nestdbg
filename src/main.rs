mod config;
mod network;
mod terminal;

use terminal::app::Application;
use terminal::input;
use terminal::message;
use terminal::ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Application::new();

    loop {
        // ui::draw_ui(&mut state.terminal, &mut state)?;
        terminal.draw()?;
        let message = input::handle_input(&mut terminal).await;

        match message.message_type {
            message::MessageType::SendInput => {
                terminal.send_input();
            }
            message::MessageType::ShowPopup => {
                terminal.toggle_popup();
            }
            message::MessageType::None => {}
            message::MessageType::Quit => {
                break;
            }
        }
    }

    Ok(())
}
