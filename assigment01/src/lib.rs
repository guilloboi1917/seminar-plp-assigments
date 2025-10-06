// Declare quicksort module
pub mod quicksort;
pub use quicksort::quicksort;
pub use quicksort::run_quicksort_examples;

// Declare external_command module
pub mod external_command;
pub use external_command::show_sysinfo;

// Declare xml_util module
pub mod xml_util;
pub use xml_util::download_file;
pub use xml_util::parse_xml;
