use doves::RingType;

mod doves;

fn main() {
    let mut rings = doves::read_doves_csv("dove.csv");
    rings.retain(|t| t.ring_type == RingType::FullCircle); // Remove carrilons

    // dbg!(rings);

    for ring in &rings {
        print!("{}", ring.place);
        if let Some(alt_name) = &ring.alt_name {
            print!("/{alt_name}");
        }
        print!(" ({}): ", ring.dedication);
        if let Some(county) = &ring.county {
            print!("{county}, ");
        }
        print!("{}", ring.country);
        print!(" ({} bells, {})", ring.num_bells, ring.weight);
        println!();
    }
}
