use std::collections::VecDeque;

use globset::Glob;
use pfetch_logo_parser::Logo;

pub fn logo(logo_name: &str) -> Logo {
    let (tux, included_logos) = pfetch_extractor::parse_logos!();
    let /* mut */ logos: VecDeque<_> = included_logos.into();
    // if let Ok(filename) = dotenvy::var("PF_CUSTOM_LOGOS") {
    //     // insert custom logos in front of incuded logos
    //     for custom_logo in parse_custom_logos(&filename).into_iter().flatten() {
    //         logos.insert(0, custom_logo.clone());
    //     }
    // };
    logos
        .into_iter()
        .find(|logo| {
            logo.pattern.split('|').any(|glob| {
                Glob::new(glob.trim())
                    .expect("Invalid logo pattern")
                    .compile_matcher()
                    .is_match(logo_name)
            })
        })
        .unwrap_or(tux)
}
