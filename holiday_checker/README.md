# 🗓️ holiday_checker

A Rust library for querying and analyzing Sri Lankan public holidays for the year 2025. This crate allows you to load holiday data from a JSON file and check dates, filter by type/month/category, and search by holiday name.

---

## 📆 Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
holiday_checker = "0.1.3"
```

---

## 🛠️ How to Use

### ✅ Load data and check for a holiday

```rust
use holiday_checker::holiday::{load_embedded_holidays, HolidayData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let holidays: HolidayData = load_embedded_holidays()?;

    // Check if a specific date is a holiday
    let check_date = "2025-04-14";
    if holidays.is_holiday(check_date) {
        println!("{check_date} is a holiday!");
    } else {
        println!("{check_date} is not a holiday.");
    }

    // Get all holidays
    println!("\nAll holidays:");
    for holiday in holidays.get_all_holidays() {
        println!("{:?}", holiday);
    }

    // Filter by month (April = 4)
    println!("\nHolidays in April:");
    for holiday in holidays.get_holidays_by_month(4) {
        println!("{:?}", holiday);
    }

    // Filter by type
    println!("\nReligious holidays:");
    for holiday in holidays.get_holidays_by_type("religious") {
        println!("{:?}", holiday);
    }

    // Filter by category
    println!("\nBank holidays:");
    for holiday in holidays.get_holidays_by_category("bank") {
        println!("{:?}", holiday);
    }

    // Search by name
    println!("\nSearch results for 'new year':");
    for holiday in holidays.search_holiday_by_name("new year") {
        println!("{:?}", holiday);
    }

    // Find holiday by exact date
    println!("\nHoliday on 2025-05-01:");
    match holidays.get_holiday_by_date("2025-05-01") {
        Some(h) => println!("{:?}", h),
        None => println!("No holiday found on 2025-05-01."),
    }

    Ok(())
}
```

---

## 📁 JSON File Format

The crate expects a JSON file like this:

```json
{
  "year": 2025,
  "holidays": [
    {
      "date": "2025-04-14",
      "name": "Sinhala and Tamil New Year Day",
      "type": "cultural",
      "categories": ["public", "bank", "mercantile"]
    }
  ]
}
```

It already saved as `holidays_2025.json` in your project root or specify your own path.

---

## 🔍 Available Methods

```rust
let holidays = load_embedded_holidays()?;

// Get all holidays
holidays.get_all_holidays();

// Filter by month
holidays.get_holidays_by_month(4);

// Filter by type
holidays.get_holidays_by_type("religious");

// Filter by category
holidays.get_holidays_by_category("bank");

// Search by name
holidays.search_holiday_by_name("new year");

// Check if a specific date is a holiday
holidays.is_holiday("2025-04-14");

// Find holiday by exact date
holidays.get_holiday_by_date("2025-05-01");
```

Each method is well-documented and returns results you can use immediately in applications or APIs.

---

## ▶️ How to Run

Once added to a binary crate or test project:

```bash
cargo build
cargo run
```

Make sure the JSON file (e.g. `holidays_2025.json`) is in the same directory where you're running the command.

---

## ✅ When to Use This Crate

- Holiday calendars
- HR attendance systems
- School academic tools
- Event planners
- National observance trackers

---

Pull requests and contributions welcome!

