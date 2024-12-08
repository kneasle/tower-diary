use doves::RingType;

mod doves;

fn main() {
    let mut rings = doves::read_doves_csv("dove.csv");
    rings.retain(|t| t.ring_type == RingType::FullCircle); // Remove carrilons
    dbg!(rings);
}
