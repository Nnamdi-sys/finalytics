use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;
use headless_chrome::types::Bounds;
use teloxide::prelude::*;
use teloxide::types::InputFile;

pub async fn html_to_png(
    html: &str,
    bot: Arc<Bot>,
    msg: Arc<Message>
) -> Result<(), std::io::Error> {
    let browser = match Browser::new(
        LaunchOptions::default_builder()
            .port(Some(9222))
            .sandbox(false)
            .build()
            .expect("Could not find Chrome binary."),
    ) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to create browser: {}", e);
            return Ok(());
        }
    };
    let tab = match browser.new_tab() {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to create tab: {}", e);
            return Ok(());
        }
    };
    let data_url = format!("data:text/html,{}", html);
    tab.navigate_to(&data_url).expect("Failed to navigate to data url");
    tab.wait_for_element("svg.main-svg").expect("Failed to load svg");

    tab.set_bounds(Bounds::Normal {
        left: None,
        top: None,
        width: Some(1300.0),
        height: Some(1100.0),
    }).expect("Failed to set bounds");

    let png_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png,
                                          None, None, true)
        .expect("Failed to capture screenshot");

    // Save the screenshot as an image file
    let image_path = "temp_image.png";
    let mut file = File::create(image_path).expect("Failed to create image file");
    file.write_all(&png_data).expect("Failed to write image file");

    match bot.send_photo(msg.chat.id, InputFile::file(image_path)).await {
        Ok(_) => {},
        Err(e) => println!("Failed to send telegram photo: {}", e),
    }

    Ok(())
}
