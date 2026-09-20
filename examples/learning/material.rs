pub const AIR: u8 = 0;
pub const ROCK: u8 = 1;
pub const SOIL: u8 = 2;

/// Called by readers as needed; map an ID to copied RGBA bytes.
/// RGB channels are sRGB encoded: alpha is 0 for clear and 255 for opaque.
/// Unknown IDs return None, rather than silently looking like air.
pub fn rgba(material: u8) -> Option<[u8; 4]> {
    match material {
        AIR => Some([0, 0, 0, 0]),
        ROCK => Some([110, 115, 125, 255]),
        SOIL => Some([60, 160, 70, 255]),
        _ => None,
    }
}
