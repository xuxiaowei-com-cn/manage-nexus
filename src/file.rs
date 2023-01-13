use std::fs;

/**
 * 遍历文件夹中所有的文件
 */
pub fn all_file_path_fs(_path: &str) {
    let read_dirs = fs::read_dir(_path).unwrap();

    for read_dir in read_dirs {
        let path = read_dir.unwrap().path();

        if path.is_dir() {
            // 文件夹


            // 递归
            all_file_path_fs(path.display().to_string().as_str());
        } else {
            // 文件

            // 打印文件名
            println!("{}", path.display());
        }
    }
}

#[test]
fn all_file_path_fs_test() {
    let path = ".";
    all_file_path_fs(path);
}
