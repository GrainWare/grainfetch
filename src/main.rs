use crossterm::style::{Color, Stylize};
use std::error::Error;

mod grains;
mod printmgr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Print table header.
    printmgr::print_table_header();

    // Fetch grain data using the grains module.
    let grains_data = grains::get_common_grains_data().await?;

    // For each grain, print a formatted row.
    for grain in grains_data {
        // Select background color based on daily change.
        let bg_color = if grain.daily_change > 0.0 {
            Color::DarkGreen
        } else if grain.daily_change < 0.0 {
            Color::DarkRed
        } else {
            Color::Grey
        };

        // Format numeric columns with fixed widths.
        let price_str = format!("{:>15.2}", grain.current_price);
        let daily_str = if grain.daily_change >= 0.0 {
            format!("{:>15}", format!("(+{:.2})", grain.daily_change))
        } else {
            format!("{:>15}", format!("({:.2})", grain.daily_change))
        };
        let (percentage, status) = printmgr::compute_change_info(grain.daily_change, grain.current_price);
        let percent_str = format!("{:>12.2}%", percentage);

        println!(
            "{:<10} | {} | {} | {} | {}",
            grain.name,
            price_str.on(bg_color),
            daily_str.on(bg_color),
            percent_str.on(bg_color),
            status
        );
    }

    println!(
        "\nData retrieved on: {} UTC",
        chrono::Utc::now().format("%Y-%m-%d %H:%M")
    );
    Ok(())
}
