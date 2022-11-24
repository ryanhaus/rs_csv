use crate::csv_parse;
use nwd::NwgUi;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    // main window
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Rust CSV Parser", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [BasicApp::win_close])]
    window: nwg::Window,

    // grid layout that all the objects in the window fall under
    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    // label that instructs the user what to do
    #[nwg_control(text: "Please select a CSV file")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    label: nwg::Label,

    // button that creates file picker when pressed
    #[nwg_control(text: "Choose a file")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0, row_span: 2)]
    #[nwg_events(OnButtonClick: [BasicApp::open_file_dialog])]
    file_pick_button: nwg::Button,
}

impl BasicApp {
    fn open_file_dialog(&self) {
        // build a file picker dialog
        let mut dialog: nwg::FileDialog = Default::default();
        nwg::FileDialog::builder()
            .title("Select a CSV file")
            .action(nwg::FileDialogAction::Open)
            .multiselect(false)
            .build(&mut dialog)
            .expect("Could not create file dialog");

        // run the file picker dialog (returns true if a file is selected)
        if dialog.run(Some(self.window.handle)) {
            // retrieve the name of the selected file
            let file_name = dialog.get_selected_item().unwrap();

            // check that the file is CSV
            let file_type = Path::new(&file_name)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap();

            if file_type != "csv" {
                nwg::modal_error_message(
                    &self.window,
                    "Invalid file type",
                    "Input file must be a .csv file",
                );

                return;
            }

            // attempt to read the file to a string and parse it as a CSV file
            let data = fs::read_to_string(file_name).expect("Could not read file");
            let parsed_data = csv_parse::parse_csv::<u32>(data);

            println!("{:?}", &parsed_data);
        }
    }

    fn win_close(&self) {
        // stop window thread
        nwg::stop_thread_dispatch();
    }
}
