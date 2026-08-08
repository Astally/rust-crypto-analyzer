use thousands::Separable;

pub fn format_large_number(large_number: f64) -> String {
    let number: String;

    if large_number >= 1_000_000_000_000.0 {
        number = format!("{:.2} T", large_number / 1_000_000_000_000.0);
    } else if large_number >= 1_000_000_000.0 {
        number = format!("{:.2} B", large_number / 1_000_000_000.0);
    } else if large_number >= 1_000_000.0 {
        number = format!("{:.2} M", large_number / 1_000_000.0);
    } else {
        number = large_number.separate_with_commas();
    }

    number
}

pub fn format_percentage(percent_number: f64) -> String {
    let number: String;
    if percent_number > 0.0 {
        number = format!("+{:.2}%", percent_number);
    } else {
        number = format!("{:.2}%", percent_number);
    }

    number
}

pub fn format_price(price: f64) -> String {
    let number: String;

    if price >= 1.0 {
        number = format!("${}", format!("{:.2}", price).separate_with_commas());
    } else if price >= 0.1 {
        number = format!("${}", format!("{:.3}", price));
    } else if price >= 0.01 {
        number = format!("${}", format!("{:.4}", price));
    } else if price >= 0.001 {
        number = format!("${}", format!("{:.5}", price));
    } else {
        number = format!("${}", price);
    }

    number
}

pub fn format_timestamp(timestamp: f64) -> String {
    chrono::DateTime::from_timestamp_millis(timestamp as i64)
        .map(|datetime| datetime.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "-".to_string())
}
