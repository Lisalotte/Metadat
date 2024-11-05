/*
* Metadat
* A small script to change the metadata of a jpg picture.
* Made because I did not want to change the timestamps of all my holiday pictures by hand.
* Uses little_exif crate for all the metadata things
*/

use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;
use little_exif::u8conversion::*;
use chrono::{NaiveDateTime, Duration};
use std::path::Path;
use std::fs;

fn get_datetime_original(image_path: &str) -> std::io::Result<String> {
    let metadata: Metadata = Metadata::new_from_path(std::path::Path::new(image_path)).unwrap();

    let datetime_original = metadata.get_tag(&ExifTag::DateTimeOriginal(String::new())).next().unwrap();
    
    // Print it as String
	let endian = metadata.get_endian();
    let original_datetime_str = String::from_u8_vec(
		&datetime_original.value_as_u8_vec(&metadata.get_endian()),
		&endian
	);

    println!("Original DateTime: {}", original_datetime_str);

    Ok(original_datetime_str)
}

fn set_datetime_new(image_path: &str, new_image_path: &str, original_datetime_str: &String) {
    let shift_hours = 7; // Hours to add to the timestamp
    let shift_minutes = -17;
    let original_datetime = NaiveDateTime::parse_from_str(original_datetime_str.as_str(), "%Y:%m:%d %H:%M:%S").unwrap();
    let new_datetime = original_datetime + Duration::hours(shift_hours) + Duration::minutes(shift_minutes);
    let formatted_datetime = new_datetime.format("%Y:%m:%d %H:%M:%S").to_string();
    
    println!("Updated DateTime: {}", formatted_datetime);

    let mut metadata_new: Metadata = Metadata::new_from_path(std::path::Path::new(image_path)).unwrap();

    metadata_new.set_tag(
        ExifTag::DateTimeOriginal(formatted_datetime)
    );

    fs::copy(image_path, new_image_path);
    let jpg_path = Path::new(new_image_path);
    metadata_new.write_to_file(jpg_path).unwrap();
}

fn change_time_and_copy_to_new_folder(folder_path: &str, destination_path: &str) -> std::io::Result<()>  {
    let path = Path::new(folder_path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                println!("File: {:?}", path);
                let filename = path.file_name().unwrap();
                let new_file_path = format!("{}\\{}", destination_path, filename.to_str().unwrap());
                println!("{}", new_file_path);
                let datetime_original = get_datetime_original(path.to_str().unwrap()).unwrap();
                set_datetime_new(path.to_str().unwrap(), &new_file_path, &datetime_original);
            }
        }
    }    

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let folder_path = "Folder";
    let destination_path = "Destination";

    change_time_and_copy_to_new_folder(folder_path, destination_path);

    Ok(())
}