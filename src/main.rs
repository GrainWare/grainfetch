use crossterm::style::{Color, Stylize};
use std::error::Error;

mod grains;

/// Prints a table header.
fn print_table_header() {
    println!(
        "{}",
        format!(
            "{:<10} | {:>15} | {:>15} | {:>12} | {}",
            "Name", "Price (USD)", "Daily Change", "Change (%)", "Status"
        )
        .bold()
        .blue()
    );
    println!("{}", "-".repeat(70));
}

/// Computes the percentage change and returns both the percentage and an associated status string.
/// Assumes yesterday's price is (current_price - daily_change).
fn compute_change_info(daily_change: f64, current_price: f64) -> (f64, String) {
    let yesterday_price = current_price - daily_change;
    let percentage = if yesterday_price.abs() < 1e-6 {
        0.0
    } else {
        (daily_change / yesterday_price) * 100.0
    };

    // Use a match statement with 15 preset statuses.
    let status = match percentage {
        x if x >= 50.0 => "FUCK YEAH HARVEST SEASON IS UPON US AND ITS FUCKING BOUNTIFUL",
        x if x >= 40.0 => "HARVEST SEASON IS UPON US!!!!!",
        x if x >= 30.0 => "harvest season is upon us!!",
        x if x >= 20.0 => "grain!!!",
        x if x >= 10.0 => "grain!!",
        x if x >= 5.0  => "grain!",
        x if x >= 1.0  => "grain",
        x if x > -1.0  => "grain",
        x if x > -5.0  => "oh no",
        x if x > -10.0 => "OH SHIT",
        x if x > -20.0 => "WHAT HAVE WE DONE",
        x if x > -30.0 => "WHAT HAPPENED TO HARVEST SEASON",
        x if x > -40.0 => "RAHHHHHHHHHHHHHHHHHHHH",
        x if x > -50.0 => "AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH",
        _              => "GRAIN IS OVER. GOODBYE.",
    }
    .to_string();

    (percentage, status)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Print table header.
    print_table_header();

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
        let (percentage, status) = compute_change_info(grain.daily_change, grain.current_price);
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
