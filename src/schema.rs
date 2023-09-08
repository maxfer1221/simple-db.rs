#[derive(Clone, Debug)]
pub struct Schema {
    pub id: u32,
    pub username: [u8; 32],
    pub email: [u8; 255],
}