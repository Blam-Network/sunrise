use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_end_of_file, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use blf_lib::types::array::StaticArray;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_rewards_persistance, s_blf_chunk_daily_challenges};

blf_file!(
    struct rewards_file {
        _blf: s_blf_chunk_start_of_file,
        rdpl: s_blf_chunk_rewards_persistance,
        dcha: s_blf_chunk_daily_challenges,
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_rewards_file() -> impl IntoResponse {
    let mut rdpl_data = Vec::<u8>::new();
    rdpl_data.resize(0x20F, 1);

    let mut arena = rewards_file {
        _blf: s_blf_chunk_start_of_file::default(),
        rdpl: s_blf_chunk_rewards_persistance {
            unknown1: 20_000_000,
            unknown2: StaticArray::from_slice(rdpl_data.as_slice()).unwrap(),
            unknown3: 0,
            unknown4: 0
        },
        dcha: s_blf_chunk_daily_challenges::default(),
        _eof: s_blf_chunk_end_of_file::default()
    };

    arena.write()
}