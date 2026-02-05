use std::{fs::File, io::{Read, Write}};

use reqwest::blocking::Client;
fn main() {
    let url="https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.18.8.tar.xz";

    let client=Client::new();
    let response=client.head(url).send().unwrap();
    let h=response.headers();
    let file_size=h.get("content-length").unwrap().to_str().unwrap().parse::<u64>().unwrap();
    println!("File size is {}",file_size);
    println!("content type is {}",h.get("content-type").unwrap().to_str().unwrap());
    let accept_ranges=h.get("accept-ranges");
    let pausable=accept_ranges.is_some();
    #[derive(Clone)]
    struct ChunkInfo {
        start: u64,
        end: u64,
    }
    let mut chunk_size: Vec<ChunkInfo>=Vec::with_capacity(8);
    for i in 0..8 {
        chunk_size.push(ChunkInfo{start: i*file_size/8, end: (i+1)*file_size/8-1});
        // println!("chunk_size is {:?}-{:?}",chunk_size[i as usize].start,chunk_size[i as usize].end);
    }
    chunk_size[7].end = file_size;
    if pausable {
        if file_size>1024000 {
        for i in 0..8 {
            let range_header=format!("bytes={}-{}",chunk_size[i].start,chunk_size[i].end);
            let response=client.get(url).header("range", range_header).send().unwrap();
            let mut file=std::fs::File::create(format!("linux-6.18.8_{}.tar.xz",i)).unwrap();
            file.write_all(&response.bytes().unwrap()).unwrap();
            
            // let h=response.headers();
            // println!("{}",h.get("content-length").unwrap().to_str().unwrap().parse::<u64>().unwrap());
            // std::fs::File::create("linux-6.18.8.tar.xz").unwrap();
        }
    } else {
        println!("The file is not pausable");
    }
    let mut output_file= std::fs::File::create("linux-6.18.9.tar.xz").unwrap();
    for i in 0..8 {
        let mut file=std::fs::File::open(format!("linux-6.18.8_{}.tar.xz",i)).unwrap();
        let mut buffer=Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        output_file.write_all(&buffer).unwrap();
    }
}
}
