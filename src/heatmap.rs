use chrono::{Datelike, NaiveDate, Utc, Weekday};
use colored::{ColoredString, Colorize};
use std::collections::HashMap;

// Struct to generate and render contribution heatmaps
pub struct Heatmap {
    contributions: HashMap<NaiveDate, u32>,
    date_range: (NaiveDate, NaiveDate),
}

impl Heatmap {
    pub fn new(contributions: HashMap<NaiveDate, u32>) -> Self {
        // Find the earliest and latest dates
        let mut earliest = Utc::now().naive_utc().date();
        let mut latest = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        
        for date in contributions.keys() {
            if *date < earliest {
                earliest = *date;
            }
            if *date > latest {
                latest = *date;
            }
        }
        
        // If no contributions, use current date
        if contributions.is_empty() {
            let today = Utc::now().naive_utc().date();
            earliest = today;
            latest = today;
        }
        
        // Adjust earliest date to start from Sunday for better alignment
        while earliest.weekday() != Weekday::Sun {
            earliest = earliest.pred_opt().unwrap_or(earliest);
        }
        
        Self {
            contributions,
            date_range: (earliest, latest),
        }
    }
    
    // Calculate total number of contributions
    pub fn total_contributions(&self) -> u32 {
        self.contributions.values().sum()
    }
    
    // Count days with at least one contribution
    pub fn active_days(&self) -> u32 {
        self.contributions.values().filter(|&&count| count > 0).count() as u32
    }
    
    // Find maximum contributions in a single day
    pub fn max_contributions_in_day(&self) -> u32 {
        *self.contributions.values().max().unwrap_or(&0)
    }
    
    // Print a horizontal border with optional message
    fn print_border(&self, width: usize, msg: &str) {
        for _ in 0..width {
            print!("=");
        }
        print!("{}", msg);
        println!();
    }

    // Render the heatmap to the terminal
    pub fn render(&self, dark_mode: bool, use_symbols: bool, use_numbers: bool) {
        println!(); // Add some spacing
        
        // Create grid and determine its width
        let grid = self.create_grid();
        let grid_width = grid.len();
        
        // Print month headers and calculate width for borders
        let header_width = self.print_month_headers(grid_width);
        
        // Print top border with date range
        let (start_date, end_date) = self.date_range;
        let date_range_msg = format!("  {}-{}", 
                                     start_date.format("%Y-%m-%d"), 
                                     end_date.format("%Y-%m-%d"));
        self.print_border(header_width, &date_range_msg);
        
        // Print weekday labels and heatmap grid
        self.print_grid(&grid, dark_mode, use_symbols, use_numbers);
        
        // Print bottom border
        self.print_border(header_width, "");
        println!(); // Add some spacing
        
        // Print color/symbol key
        self.print_key(dark_mode, use_symbols, use_numbers);
    }
    
    // Create grid structure
    fn create_grid(&self) -> Vec<Vec<NaiveDate>> {
        let (start_date, end_date) = self.date_range;
        let mut current_date = start_date;
        
        // Create a grid of weeks x days
        let mut grid = Vec::new();
        let mut current_week = Vec::new();
        
        while current_date <= end_date {
            current_week.push(current_date);
            if current_date.weekday() == Weekday::Sat {
                grid.push(current_week);
                current_week = Vec::new();
            }
            current_date = current_date.succ_opt().unwrap_or(current_date);
        }
        
        // Add the last week if not complete
        if !current_week.is_empty() {
            while current_week.len() < 7 {
                let last_date = *current_week.last().unwrap();
                current_week.push(last_date.succ_opt().unwrap_or(last_date));
            }
            grid.push(current_week);
        }
        
        grid
    }
    
    // Print the month headers above the heatmap
    fn print_month_headers(&self, grid_width: usize) -> usize {
        let (start_date, end_date) = self.date_range;
        let mut current_date = start_date;
        let mut current_month = current_date.month();
        let mut month_positions = Vec::new();
        let mut position = 0;
        
        // Calculate month positions
        while current_date <= end_date {
            if current_date.month() != current_month {
                let month_name = Self::month_name(current_month);
                month_positions.push((position, month_name));
                current_month = current_date.month();
            }
            if current_date.weekday() == Weekday::Sun {
                position += 1;
            }
            current_date = current_date.succ_opt().unwrap_or(current_date);
        }
        
        // Add the last month
        month_positions.push((position, Self::month_name(current_month)));
        
        // Print month names
        print!("    "); // Space for weekday labels (reduced by 1)
        let mut last_pos = 0;
        for (pos, name) in month_positions {
            let spaces = pos - last_pos;
            if spaces > 0 {
                print!("{}{}", " ".repeat(spaces * 2 - spaces), name); // Adjusted spacing
            }
            last_pos = pos + name.len() / 2;
        }
        println!();
        
        // Calculate width based on actual grid size
        // Make sure it's at least as wide as needed for the grid + some padding
        let min_width = 4 + (grid_width * 2); // 4 for labels + 2 chars per week
        let current_width = 4 + (position * 2 - position) + 10;
        
        std::cmp::max(min_width, current_width)
    }
    
    // Print the weekday labels and contribution grid
    fn print_grid(&self, grid: &[Vec<NaiveDate>], dark_mode: bool, use_symbols: bool, use_numbers: bool) {
        // Print the grid transposed (days as rows)
        for day_idx in 0..7 {
            // Print weekday label
            if day_idx == 1 {
                print!("Mon ");
            } else if day_idx == 3 {
                print!("Wed ");
            } else if day_idx == 5 {
                print!("Fri ");
            } else {
                print!("    ");
            }
            
            // Print cells for each week
            for week in grid {
                if day_idx < week.len() {
                    let date = week[day_idx];
                    let count = self.contributions.get(&date).unwrap_or(&0);
                    
                    if use_numbers {
                        print!("{:2}", count); // Removed space
                    } else {
                        let cell = self.format_cell(*count, dark_mode, use_symbols);
                        print!("{}", cell); // Removed space
                    }
                }
            }
            println!();
        }
    }
    
    // Print legend/key for the heatmap
    fn print_key(&self, dark_mode: bool, use_symbols: bool, use_numbers: bool) {
        if use_numbers {
            return; // No key needed for numbers
        }
        
        print!("  Less ");
        
        // Show the full gradient range
        let counts = [0, 4, 8, 12, 16, 20]; // Representing each intensity level
        for count in counts {
            let cell = self.format_cell(count, dark_mode, use_symbols);
            print!("{}", cell);
        }
        
        println!(" More");
    }
    
    // Format a cell based on contribution count and preferences
    fn format_cell(&self, count: u32, dark_mode: bool, use_symbols: bool) -> ColoredString {
        let intensity = if count == 0 {
            0
        } else if count < 5 {
            1
        } else if count < 10 {
            2
        } else if count < 15 {
            3
        } else if count < 20 {
            4
        } else {
            5
        };
        
        let text = if use_symbols {
            match intensity {
                0 => "  ",
                1 => "..",
                2 => "--",
                3 => "~~",
                4 => "**",
                _ => "##",
            }
        } else {
            "  " // Two spaces for colored blocks
        };
        
        if use_symbols {
            match intensity {
                0 => text.normal(),
                1 => text.bright_black(),
                2 => text.blue(),
                3 => text.green(),
                4 => text.yellow(),
                _ => text.red(),
            }
        } else if dark_mode {
            // Red gradient (dark mode)
            match intensity {
                0 => text.normal(),
                1 => text.on_truecolor(59, 0, 0),
                2 => text.on_truecolor(102, 0, 0),
                3 => text.on_truecolor(157, 0, 0),
                4 => text.on_truecolor(204, 0, 0),
                _ => text.on_truecolor(255, 0, 0),
            }
        } else {
            // Green gradient (light mode)
            match intensity {
                0 => text.normal(),
                1 => text.on_truecolor(220, 247, 220),
                2 => text.on_truecolor(153, 237, 153),
                3 => text.on_truecolor(85, 219, 85),
                4 => text.on_truecolor(44, 160, 44),
                _ => text.on_truecolor(0, 109, 0),
            }
        }
    }
    
    // Helper to get month name from month number
    fn month_name(month: u32) -> &'static str {
        match month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "",
        }
    }
}
