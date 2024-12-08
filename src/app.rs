use crate::doves::{RingOfBells, RingType};

pub struct App {
    rings: Vec<RingOfBells>,

    search_term: String,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Read Dove's
        let mut rings = crate::doves::read_doves_csv("dove.csv");
        rings.retain(|t| t.ring_type == RingType::FullCircle); // Remove carrilons

        Self {
            rings,
            search_term: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Heading
            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.text_edit_singleline(&mut self.search_term);
                ui.add_space(20.0);
                ui.label("(Date placeholder)");
            });
            ui.separator();
            ui.add_space(5.0);

            // Table
            let mut table_rows = get_table_rows(&self.rings);
            table_rows.retain(|row| does_row_match_search(row, &self.search_term));

            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            egui::ScrollArea::vertical().show_rows(
                ui,
                row_height,
                table_rows.len(),
                |ui, row_range| {
                    for row_idx in row_range {
                        ui.horizontal(|ui| {
                            for entry in &table_rows[row_idx] {
                                ui.label(entry);
                            }
                        });
                    }
                },
            );
        });
    }
}

fn get_table_rows(rings: &[RingOfBells]) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for ring in rings {
        let place_entry = match &ring.place2 {
            None => format!("{}", ring.place),
            Some(place2) => format!("{}, {}", ring.place, place2),
        };
        // if let Some(alt_name) = &ring.alt_name {
        //     if &ring.place2 != &ring.alt_name {
        //         print!("/{alt_name}");
        //     }
        // }
        rows.push(vec![
            place_entry,
            format!("({})", ring.dedication),
            match &ring.county {
                None => format!("{}", ring.country),
                Some(county) => format!("{county}, {}", ring.country),
            },
            format!("({} bells, {})", ring.num_bells, ring.weight),
        ]);
    }
    rows
}

fn does_row_match_search(row: &[String], search_phrase: &str) -> bool {
    let search_terms: Vec<String> = search_phrase
        .split_whitespace()
        .map(str::to_lowercase)
        .collect();
    for term in &search_terms {
        let is_found = row.iter().any(|entry| entry.to_lowercase().contains(term));
        if !is_found {
            return false; // If any term isn't present, return false
        }
    }
    true // If all terms are found, then return true
}
