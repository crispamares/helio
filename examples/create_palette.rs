///
/// This example is used to create the palettes in helio
/// All colors are borrowed from d3 https://github.com/d3/d3-scale-chromatic
/// 
use std::error::Error;
use helio::core::{Color};

fn palette(name: &str, colors: &[&str]) {
    let palette: Vec<Color> = colors.iter().map(|hex| hex.parse().unwrap()).collect();
    println!("/// {} ({}) {:?}", name, palette.len(), colors);
    println!("pub const {}: [Color; {}] = {:?};\n", name, palette.len(), palette);
}

fn main() -> Result<(), Box<dyn Error>> {
    palette("PALETTE_CATEGORY10", &["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf"]);
    palette("PALETTE_ACCENT", &[ "#7fc97f", "#beaed4", "#fdc086", "#ffff99", "#386cb0", "#f0027f", "#bf5b17", "#666666" ]);
    palette("PALETTE_DARK2", &[ "#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e", "#e6ab02", "#a6761d", "#666666" ]);
    palette("PALETTE_PAIRED", &["#a6cee3", "#1f78b4", "#b2df8a", "#33a02c", "#fb9a99", "#e31a1c", "#fdbf6f", "#ff7f00", "#cab2d6", "#6a3d9a", "#ffff99", "#b15928"]);
    palette("PALETTE_PASTEL1", &[ "#fbb4ae", "#b3cde3", "#ccebc5", "#decbe4", "#fed9a6", "#ffffcc", "#e5d8bd", "#fddaec", "#f2f2f2" ]);
    palette("PALETTE_PASTEL2", &[ "#b3e2cd", "#fdcdac", "#cbd5e8", "#f4cae4", "#e6f5c9", "#fff2ae", "#f1e2cc", "#cccccc" ]);
    palette("PALETTE_SET1", &[ "#e41a1c", "#377eb8", "#4daf4a", "#984ea3", "#ff7f00", "#ffff33", "#a65628", "#f781bf", "#999999" ]);
    palette("PALETTE_SET2", &[ "#66c2a5", "#fc8d62", "#8da0cb", "#e78ac3", "#a6d854", "#ffd92f", "#e5c494", "#b3b3b3" ]);
    palette("PALETTE_SET3", &["#8dd3c7","#ffffb3","#bebada","#fb8072","#80b1d3","#fdb462","#b3de69","#fccde5","#d9d9d9","#bc80bd","#ccebc5","#ffed6f"]);
    Ok(())
}