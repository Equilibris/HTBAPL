extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;

pub fn normalize_apl_code(str: String) -> String {
    str.nfc().collect()
}
