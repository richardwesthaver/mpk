use ot_utils::Slicer;
use std::path::Path;
use std::fs;

fn main() {
  let folder_path = "path/to/sample/folder".to_string();
  let check_file: &Path = &folder_path.as_ref();
  let slicer = Slicer::new();
  // Validate directory
  if check_file.is_dir() {

    // Get list of files
    let paths = fs::read_dir(&folder_path).unwrap();

    // Set output folder
    slicer.output_folder = folder_path.clone();

    // Set final .ot and .wav filename
    slicer.output_filename = check_file.file_name().unwrap().to_str().unwrap().to_string();


    for path in paths {
      // Get file info (path, name, and extension)
      let file_path = path.unwrap().path();
      let file_name = &file_path.file_name();
      let file_ext = match &file_path.extension(){
        &Some(x) => x.to_str().unwrap(),
        &None => " "
      };

      if file_ext == "wav" {
        slicer.add_file(file_name.unwrap());
      } 
    }
  }
  
  slicer.generate_ot_file(true);
}
