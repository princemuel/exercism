extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}


pub fn reverse_bonus(input: &str) -> String {
    input.chars().rev().collect()
}
