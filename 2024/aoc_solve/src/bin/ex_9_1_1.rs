use std::fs;

fn build_checksum(blocks: &String) -> Vec<usize> {
    let mut end_id_block = blocks.len() / 2 + blocks.len() % 2;
    let mut checksum: Vec<usize> = vec![0; end_id_block];
    let mut start_index_checksum: usize = 0;
    let mut id_block: usize = 0;
    let mut remain_spaces_num: usize = 0;
    let mut start_reallocate_file_block_idx = blocks.len() - ((blocks.len() - 1) % 2) - 1;

    let blocks_number: Vec<usize> = blocks
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for idx in (0..blocks_number.len()).step_by(2) {
        if let Some(&file_block_num) = blocks_number.get(idx) {
            if file_block_num > 0 {
                let sum_index_checksum =
                    file_block_num * (2 * start_index_checksum + file_block_num - 1) / 2;
                checksum[id_block] += sum_index_checksum;
                start_index_checksum += file_block_num
            }
        };

        if let Some(&space_block_number) = blocks_number.get(idx + 1) {
            remain_spaces_num += space_block_number;
            id_block = end_id_block - 1;

            if let Some(&reallocate_block_num) = blocks_number.get(start_reallocate_file_block_idx)
            {
                remain_spaces_num += space_block_number - reallocate_block_num;
            }
        };
    }

    return checksum;
}

fn main() {
    let path = "src/bin/input/day9.txt";
    let blocks = fs::read_to_string(path).expect("Error when read file");
}
