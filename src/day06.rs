use super::{AppError, InputReader};
use anyhow::Result;

pub fn day_6_1(filename: &str) -> Result<usize> {
    let mut input_reader = InputReader::new(filename, |s| Ok(s.to_owned()))?;
    let input = input_reader.next().ok_or(AppError::BadInput)?;

    let opt_index = find_marker_hashset(&input.as_bytes(), 4);

    opt_index.ok_or_else(|| AppError::BadInput.into())
}

pub fn day_6_2(filename: &str) -> Result<usize> {
    let mut input_reader = InputReader::new(filename, |s| Ok(s.to_owned()))?;
    let input = input_reader.next().ok_or(AppError::BadInput)?;

    let opt_index = find_marker_hashset(&input.as_bytes(), 14);

    opt_index.ok_or_else(|| AppError::BadInput.into())
}

fn _find_marker(bytestream: &str) -> Option<usize> {
    let skip0 = bytestream.chars();
    let skip1 = bytestream.chars().skip(1);
    let skip2 = bytestream.chars().skip(2);
    let skip3 = bytestream.chars().skip(3);

    let mut zipped = skip0
        .zip(skip1)
        .zip(skip2)
        .zip(skip3)
        .map(|(((a, b), c), d)| (a, b, c, d));

    zipped
        .position(|(a, b, c, d)| (a != b) & (a != c) & (a != d) & (b != c) & (b != d) & (c != d))
        .map(|x| x + 4)
}

fn find_marker_hashset(bytestream: &[u8], marker_size: usize) -> Option<usize> {
    let mut hashset: [usize; 26] = [0; 26];
    let mut num_repeats = 0;

    let mut index = 0;

    for i in 0..marker_size {
        let character = bytestream.get(i)?;
        let char_index = character - ('a' as u8);
        hashset[char_index as usize] += 1;
    }

    for i in hashset {
        if i > 1 {
            num_repeats += 1;
        }
    }

    while num_repeats > 0 {
        let char_to_remove = bytestream.get(index)?;
        let char_to_remove_index = char_to_remove - ('a' as u8);
        if hashset[char_to_remove_index as usize] == 2 {
            num_repeats -= 1;
        }
        hashset[char_to_remove_index as usize] -= 1;

        let char_to_insert = bytestream.get(index + marker_size)?;
        let char_to_insert_index = char_to_insert - ('a' as u8);
        if hashset[char_to_insert_index as usize] == 1 {
            num_repeats += 1;
        }
        hashset[char_to_insert_index as usize] += 1;

        index += 1;
    }

    Some(index + marker_size)
}

#[cfg(test)]
mod test {
    use super::{_find_marker, find_marker_hashset};

    #[test]
    fn test_find_marker() {
        let bytestream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker_pos = 7;
        assert_eq!(_find_marker(bytestream), Some(marker_pos));

        let bytestream = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker_pos = 5;
        assert_eq!(_find_marker(bytestream), Some(marker_pos));

        let bytestream = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker_pos = 6;
        assert_eq!(_find_marker(bytestream), Some(marker_pos));

        let bytestream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker_pos = 10;
        assert_eq!(_find_marker(bytestream), Some(marker_pos));

        let bytestream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker_pos = 11;
        assert_eq!(_find_marker(bytestream), Some(marker_pos));
    }

    #[test]
    fn test_find_marker_hashset() {
        let bytestream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker_pos = 7;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 4),
            Some(marker_pos)
        );
        let marker_pos = 19;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 14),
            Some(marker_pos)
        );

        let bytestream = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker_pos = 5;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 4),
            Some(marker_pos)
        );
        let marker_pos = 23;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 14),
            Some(marker_pos)
        );

        let bytestream = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker_pos = 6;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 4),
            Some(marker_pos)
        );
        let marker_pos = 23;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 14),
            Some(marker_pos)
        );

        let bytestream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker_pos = 10;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 4),
            Some(marker_pos)
        );
        let marker_pos = 29;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 14),
            Some(marker_pos)
        );

        let bytestream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker_pos = 11;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 4),
            Some(marker_pos)
        );
        let marker_pos = 26;
        assert_eq!(
            find_marker_hashset(bytestream.as_bytes(), 14),
            Some(marker_pos)
        );
    }
}
