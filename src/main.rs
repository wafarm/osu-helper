use std::{fs::File, io::Write};

use osu_db::{
    listing::{Beatmap, RankedStatus},
    Listing, Mod, ModSet, Mode,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
struct BeatmapInfo {
    artist: String,
    title: String,
    mapper: String,
    difficulty: String,
    id: i32,
    nm_rating: f64,
    ht_rating: f64, // Only HT DT mod affects star rating in mania
    dt_rating: f64,
}

#[derive(Serialize, Deserialize)]
struct OsuInfo {
    beatmaps: Map<String, Value>,
}

fn main() {
    let listing = Listing::from_file("osu!.db").unwrap();

    // Export ranked mania maps only
    let beatmaps: Vec<&Beatmap> = listing
        .beatmaps
        .iter()
        .filter(|it| it.status == RankedStatus::Ranked && it.mode == Mode::Mania)
        .collect();
    let mut osu_info = OsuInfo {
        beatmaps: Map::new(),
    };
    
    for beatmap in beatmaps {
        let artist = beatmap.artist_ascii.as_ref().unwrap().clone();
        let title = beatmap.title_ascii.as_ref().unwrap().clone();
        let mapper = beatmap.creator.as_ref().unwrap().clone();
        let difficulty = beatmap.difficulty_name.as_ref().unwrap().clone();
        let id = beatmap.beatmap_id;
        let nm_rating = beatmap
            .mania_ratings
            .iter()
            .find(|it| it.0 == ModSet::empty())
            .unwrap()
            .1;
        let ht_rating = beatmap
            .mania_ratings
            .iter()
            .find(|it| it.0 == ModSet::empty().with(Mod::HalfTime))
            .unwrap()
            .1;
        let dt_rating = beatmap
            .mania_ratings
            .iter()
            .find(|it| it.0 == ModSet::empty().with(Mod::DoubleTime))
            .unwrap()
            .1;
        let info = BeatmapInfo {
            artist,
            title,
            mapper,
            difficulty,
            id,
            nm_rating,
            ht_rating,
            dt_rating,
        };
        osu_info
            .beatmaps
            .insert(id.to_string(), serde_json::to_value(&info).unwrap());
    }

    let serialized = serde_json::to_string_pretty(&osu_info).unwrap();
    let serialized_min = serde_json::to_string(&osu_info).unwrap();

    File::create("osu_info.json")
        .unwrap()
        .write_all(serialized.as_bytes())
        .unwrap();
    File::create("osu_info.min.json")
        .unwrap()
        .write_all(serialized_min.as_bytes())
        .unwrap();
}
