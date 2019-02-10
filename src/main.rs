use serde_json::*;
use webhdfs::api::*;

fn main() {
    println!("Hello, world!");
    let p = Responce::FileStatuses(FileStatuses {
        file_status: vec![FileStatus {
            access_time: 12345,
            block_size: 12,
            group: "a".to_owned(),
            length: 123,
            modification_time: 123455,
            owner: "root".to_owned(),
            path_suffix: "test".to_owned(),
            permission: "0644".to_owned(),
            replication: 1,
            fstype: FsType::Directory,
        }],
    });

    println!("{}", serde_json::to_string_pretty(&p).unwrap());

    let o = "
{
    \"FileStatus\":
      {
        \"accessTime\"      : 0,
        \"blockSize\"       : 0,
        \"group\"           : \"supergroup\",
        \"length\"          : 0,
        \"modificationTime\": 1320895981256,
        \"owner\"           : \"szetszwo\",
        \"pathSuffix\"      : \"bar\",
        \"permission\"      : \"711\",
        \"replication\"     : 0,
        \"type\"            : \"DIRECTORY\"
      }
}
    ";

    let s: Responce = serde_json::from_str(o).unwrap();
}
