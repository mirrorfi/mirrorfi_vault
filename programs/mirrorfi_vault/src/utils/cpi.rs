

// Get the Discriminator of a Program Function
pub fn get_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    discriminator
}
