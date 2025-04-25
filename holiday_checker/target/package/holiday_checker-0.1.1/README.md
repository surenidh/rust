# 🗓️ holiday_checker

A Rust library for querying and analyzing Sri Lankan public holidays for the year 2025. This crate allows you to load holiday data from a JSON file and check dates, filter by type/month/category, and search by holiday name.

---

## 📆 Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
holiday_checker = "0.1.0"
```

---

## 🛠️ How to Use

### ✅ Load data and check for a holiday

```rust
use holiday_checker::holiday::{load_holidays_from_file, HolidayData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: HolidayData = load_holidays_from_file("holidays_2025.json")?;

    if data.is_holiday("2025-04-14") {
        println!("2025-04-14 is a holiday!");
    } else {
        println!("Not a holiday.");
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

Save it as `holidays_2025.json` in your project root or specify your own path.

---

## 🔍 Available Methods

```rust
let holidays = load_holidays_from_file("holidays_2025.json")?;

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

