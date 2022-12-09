use crate::utils::read_input;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct CompressedImage {
    pixels: Vec<Vec<bool>>,
    background: bool,
    width: usize,
    height: usize,
}

impl CompressedImage {
    fn new(input: &str) -> Self {
        let pixels = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = pixels.len();
        let width = pixels[0].len();

        Self {
            pixels,
            background: false,
            width,
            height,
        }
    }

    fn decompress(&self, algorithm: &CompressionAlgorithm) -> Self {
        let width = self.width + 2;
        let height = self.height + 2;
        let mut pixels = vec![vec![false; width + 2]; height + 2];

        let background = if self.background {
            algorithm.algorithm[511] // 0b111111111
        } else {
            algorithm.algorithm[0] // 0b000000000
        };

        for (new_y, new_row) in pixels.iter_mut().enumerate() {
            for (new_x, new_pixel) in new_row.iter_mut().enumerate() {
                let mut pixel_value = 0;
                for neighbor_y in 0..3 {
                    for neighbor_x in 0..3 {
                        let old_x = (new_x + neighbor_x).wrapping_sub(2);
                        let old_y = (new_y + neighbor_y).wrapping_sub(2);

                        let pixel = if old_x < self.width && old_y < self.height {
                            self.pixels[old_y][old_x]
                        } else {
                            self.background
                        };

                        pixel_value <<= 1;
                        if pixel {
                            pixel_value |= 1;
                        }
                    }
                }

                *new_pixel = algorithm.algorithm[pixel_value];
            }
        }

        Self {
            pixels,
            background,
            width,
            height,
        }
    }

    fn num_lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .filter(|pixel| **pixel)
            .count()
    }
}

impl Display for CompressedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        for row in &self.pixels {
            for pixel in row {
                s.push(if *pixel { '#' } else { '.' });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct CompressionAlgorithm {
    algorithm: [bool; 512],
}

impl CompressionAlgorithm {
    fn new(input: &str) -> Self {
        Self {
            algorithm: input
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (compression_algorithm, image) = input.split_once("\n\n").unwrap();

    let compression_algorithm = CompressionAlgorithm::new(compression_algorithm);
    let mut image = CompressedImage::new(image);

    for _ in 0..2 {
        image = image.decompress(&compression_algorithm);
    }

    image.num_lit_pixels()
}

#[test]
fn test_part1() {
    let input = read_input(2021, 20);
    assert_eq!(part1(&input), 5597);
}

pub fn part2(input: &str) -> usize {
    let (compression_algorithm, image) = input.split_once("\n\n").unwrap();

    let compression_algorithm = CompressionAlgorithm::new(compression_algorithm);
    let mut image = CompressedImage::new(image);

    for _ in 0..50 {
        image = image.decompress(&compression_algorithm);
    }

    image.num_lit_pixels()
}

#[test]
fn test_part2() {
    let input = read_input(2021, 20);
    assert_eq!(part2(&input), 18723);
}
