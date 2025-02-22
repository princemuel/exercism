extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut graphemes = input.graphemes(true).collect::<Vec<_>>();
    graphemes.reverse();

    graphemes.into_iter().collect()
}
