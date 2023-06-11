use cursive::views::LinearLayout;
use cursive::Printer;
use cursive::Vec2;

struct TableView {
    data: Vec<Vec<String>>,
}

impl TableView {
    fn new(data: Vec<Vec<String>>) -> Self {
        TableView { data }
    }
}

impl cursive::view::View for TableView {
    fn draw(&self, printer: &Printer) {
        let rows = self.data.len();
        let cols = self.data[0].len();
        let cell_width = 20;
        let cell_height = 2;

        // Draw column header border
        printer.print_hline((0, cell_height - 1), printer.size.x, "-");

        // Draw column headers
        for (j, cell) in self.data[0].iter().enumerate() {
            let x = j * cell_width;
            let y = cell_height - 2; // Adjust the y-coordinate to place the headers in a separate row
            printer.print((x, y), &format!("{:^width$}", cell, width = cell_width));
        }

        // Draw table data
        for i in 1..rows {
            for (j, cell) in self.data[i].iter().enumerate() {
                let x = j * cell_width;
                let y = i * cell_height;
                printer.print((x, y), &format!("{:^width$}", cell, width = cell_width));
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let cell_width = 20;
        let cell_height = 2;
        Vec2::new(
            self.data[0].len() * cell_width,
            self.data.len() * cell_height,
        )
    }
}

fn main() {
    let mut siv = cursive::default();

    // Create some sample data
    let data: Vec<Vec<String>> = vec![
        vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
        vec![
            "Alice".to_string(),
            "25".to_string(),
            "New York".to_string(),
        ],
        vec!["Bob".to_string(), "30".to_string(), "London".to_string()],
        vec!["Charlie".to_string(), "35".to_string(), "Tokyo".to_string()],
    ];

    // Create a custom view for the table
    let table_view = TableView::new(data);

    // Create a linear layout to hold the table view
    let layout = LinearLayout::vertical().child(table_view);

    // Handle the 'q' key to exit the application
    siv.add_global_callback('q', |s| s.quit());

    // Set the main layout
    siv.add_layer(layout);

    // Run the application
    siv.run();
}
