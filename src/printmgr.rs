use crossterm::style::{Stylize};

/// Prints a table header.
pub fn print_table_header() {
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
pub fn compute_change_info(daily_change: f64, current_price: f64) -> (f64, String) {
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