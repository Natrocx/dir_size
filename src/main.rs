use std::path::Path;
use std::fs::read_dir;

#[derive(Default)]
struct SizeInfo {
    size : u64,
    size_on_disk: u64,
    files: u64,
    special_files: u64,
    directories: u64
}


fn main() {

    let root_directory_buf = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!(e) // no use in continuing
    };
    let root_directory = root_directory_buf.as_path(); // good thing you can't inline that...

    let size_info = get_dir_size(root_directory);

    println!("Size of {:0}: {:1}",
             root_directory.to_str().unwrap(), // shouldn't fail given the method of acquiring the path object
             size_info.size);
    println!("Size on disk is not currently available.");
    println!("Files: {:0}", size_info.files);
    println!("Subdirectories: {:0}", size_info.directories);
    println!("Symlinks: {:0}", size_info.special_files);

    println!("Size in MB: {:0}", size_info.size/(1024*1024));
}

fn get_dir_size(path : &Path) -> SizeInfo {
    let mut size_info = SizeInfo::default();

    let read_dir = match read_dir(path) {
        Ok(dir) => dir,
        Err(_) => panic!("Can't open directory.")
    };

    read_dir.into_iter().for_each(|_dir_ent| {
        let dir_ent = match _dir_ent {
            Ok(d) => d,
            Err(_) => panic!("Can't open files")
        };

        let f_type = dir_ent.file_type().unwrap();
        if f_type.is_dir() {
            let temp = get_dir_size(dir_ent.path().as_path());
            size_info.size += temp.size;
            size_info.size_on_disk += temp.size_on_disk;
            size_info.files += temp.files;
            size_info.special_files += temp.special_files;
            size_info.directories += temp.directories + 1;
        }
        else if f_type.is_file() {
            let metadata = dir_ent.metadata();
            size_info.size += metadata.unwrap().len();
            size_info.size_on_disk += 0; // not accessible from standard metadata object
            size_info.files += 1;
        }
        else if f_type.is_symlink() {
            size_info.special_files +=1;
        }
    });

    size_info
}
