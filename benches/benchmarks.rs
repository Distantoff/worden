use langnote::helpers::words::words_builder::WordsBuilder;
use criterion::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::async_executor::FuturesExecutor;
// use crate::


pub fn criterion_benchmark(c: &mut Criterion) {
    let limit = 1000;
    let words = WordsBuilder::new()
        .user_id(11)
        .offset(0)
        .limit(limit);

    // dbg!(&words.clone().get_words().await);

    let desc = format!("Reg user words {limit} words");

    // c.bench_function(desc.as_str(),
    //     |b| b.iter(|| words.clone().get_words()));
    c.bench_function(desc.as_str(), move |b|
        b.to_async(FuturesExecutor).iter(|| async { words.clone().get_words().await }));
}

// pub fn criterion_benchmark(c: &mut Criterion) {
//     let enword_id_list = vec![
//         438, 439, 568, 1,   578, 589, 590, 698,
//         100, 101, 1002,101, 102, 304, 504, 890,
//         123, 2001,2030,40,  504, 789, 3,   170,
//         783, 1203,2340,43,  4234,333, 4253,19090,
//         8934,3213,45,  5324,3003,4224,5454,20010,
//
//         101, 102, 103, 104, 205, 204, 203, 202,
//         651, 661, 662, 663, 664, 671, 672, 673,
//         781, 782, 783, 794, 795, 796, 797, 798,
//     ];
//     let words = UserWords::new()
//         .set_offset(0)
//         .set_limit(100)
//         .set_enword_id_list(black_box(enword_id_list));
//
//     c.bench_function("Anon user words 40 words",
//         |b| b.iter(|| words.clone().get_words()));
// }

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
