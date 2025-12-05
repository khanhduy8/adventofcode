use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct FileBlock {
    id: usize,
    num_block: usize,
    pos: usize,
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
                    pos: 0,
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

    let mut checksum: usize = 0;
    let mut ei = disk_map.len() - 1;

    while ei >= 1 {
        let reallocate_block = &mut disk_map[ei].clone();

        for si in 0..ei {
            let space_block = &mut disk_map[si].clone();
            let reallocate_file_block = &mut reallocate_block.file_blocks[0].clone();
            if space_block.num_space == 0 {
                continue;
            }

            // println!(
            //     "Before: ({},{}): {:?} {:?}",
            //     si, ei, space_block, reallocate_block
            // );

            if space_block.num_space >= reallocate_file_block.num_block {
                reallocate_file_block.pos = space_block.file_blocks.last().unwrap().pos
                    + space_block.file_blocks.last().unwrap().num_block;
                space_block.file_blocks.push(reallocate_file_block.clone());
                space_block.num_space -= reallocate_file_block.num_block;
                reallocate_block.file_blocks.remove(0);
                reallocate_block.num_space += reallocate_file_block.num_block;
                reallocate_file_block.num_block = 0;

                disk_map[si] = space_block.clone();
                disk_map[ei] = reallocate_block.clone();

                // println!(
                //     "After move: ({},{}): {:?} {:?}",
                //     si, ei, space_block, reallocate_block
                // );

                break;
            }

            // println!(
            //     "After: ({},{}): {:?} {:?}\ndiskmap: {:?}",
            //     si, ei, space_block, reallocate_block, disk_map
            // );
        }

        ei -= 1;
    }

    let mut pos = 0;

    for disk_block in disk_map {
        let num_space = disk_block.num_space;
        let num_disk_block_pos: usize = disk_block
            .file_blocks
            .iter()
            .map(|&file_block| file_block.num_block)
            .sum::<usize>()
            + num_space;
        let mut disk_block_pos = 0;

        while disk_block_pos < num_disk_block_pos {
            if let Some(&file_block) = disk_block
                .file_blocks
                .iter()
                .filter(|&&file_block| file_block.pos == disk_block_pos)
                .last()
            {
                if file_block.pos == disk_block_pos {
                    for _ in 0..file_block.num_block {
                        checksum += pos * file_block.id;
                        print!("{}", file_block.id);
                        pos += 1;
                        disk_block_pos += 1;
                    }
                }
            } else {
                disk_block_pos += 1;
                pos += 1;
                print!(".");
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
