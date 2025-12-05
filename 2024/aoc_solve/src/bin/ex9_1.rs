use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct FileBlock {
    id: usize,
    num_block: usize,
}
#[derive(Debug, Clone, PartialEq)]
struct DiskBlock {
    file_blocks: Vec<FileBlock>,
    num_space: usize,
}

fn build_checksum(blocks: &String) -> usize {
    let mut disk_map: Vec<DiskBlock> = vec![];
    let mut file_id: usize = 0;

    for (idx, ch) in blocks.chars().enumerate() {
        if let Some(num) = ch.to_digit(10) {
            if idx % 2 == 0 {
                let file_block = FileBlock {
                    id: file_id,
                    num_block: num as usize,
                };
                let block = DiskBlock {
                    file_blocks: vec![file_block],
                    num_space: 0,
                };
                disk_map.push(block);
                file_id += 1;
            } else {
                if let Some(last_block) = disk_map.last_mut() {
                    last_block.num_space += num as usize;
                }
            }
        };
    }

    println!("{:?}", disk_map);

    let mut checksum: usize = 0;
    let mut si = 0;
    let mut ei = disk_map.len() - 1;

    while si < ei {
        let (space_block, reallocate_block) =
            (&mut disk_map[si].clone(), &mut disk_map[ei].clone());

        // println!("{:?} {:?}", space_block, reallocate_block);

        // println!(
        //     "---: {:?} {:?}\ndiskmap:{:#?}",
        //     space_block, reallocate_block, disk_map
        // );
        let mut idx = 0;
        while idx < reallocate_block.file_blocks.len() {
            let reallocate_file_block = reallocate_block.file_blocks[idx];
            if space_block.num_space == 0 {
                si += 1;
                break;
            }

            if space_block.num_space >= reallocate_file_block.num_block {
                space_block.num_space -= reallocate_file_block.num_block;
                space_block.file_blocks.push(reallocate_file_block);
                reallocate_block.file_blocks.remove(idx);

                disk_map[si] = space_block.clone();
                disk_map[ei] = reallocate_block.clone();

                if reallocate_block.file_blocks.is_empty() {
                    ei -= 1;
                    break;
                }
            } else if space_block.num_space < reallocate_file_block.num_block {
                reallocate_block.file_blocks[idx].num_block -= space_block.num_space;
                space_block.file_blocks.push(FileBlock {
                    id: reallocate_file_block.id,
                    num_block: space_block.num_space,
                });
                space_block.num_space = 0;

                disk_map[si] = space_block.clone();
                disk_map[ei] = reallocate_block.clone();
                si += 1;
                break;
            }
            idx += 1;
        }
    }

    // println!("{:#?}", disk_map);
    let mut pos = 0;
    for blocks in disk_map {
        for file_block in blocks.file_blocks {
            for _ in 0..file_block.num_block {
                checksum += pos * file_block.id;
                print!("{}", file_block.id);
                pos += 1;
            }
        }
    }

    println!("");
    println!("{}", checksum);
    return checksum;
}

fn main() {
    let path = "src/bin/input/day9.txt";
    let blocks = fs::read_to_string(path).expect("Error when read file");

    build_checksum(&blocks);
}
