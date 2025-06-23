//! Useful functions for River implementations

use nanoid::nanoid;

/// Alphanumeric alphabet used by [`generate_id`]
///
/// This is the same alphabet that is used by the [main river implementation](https://github.com/replit/river/blob/main/transport/id.ts)
pub static NANOID_ALPHABET: [char; 60] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'x', 'y', 'z', 'A', 'B', 'C',
    'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    'X', 'Y', 'Z',
];

/// Helper function used to generate identifiers similar to the [main river implementation](https://github.com/replit/river/blob/main/transport/id.ts)
pub fn generate_id() -> String {
    nanoid!(12, &NANOID_ALPHABET)
}
