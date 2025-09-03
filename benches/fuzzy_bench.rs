use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_fuzzy_rs::{fuzzy, search, SearchOptions};

fn create_test_data() -> Vec<String> {
    vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
        "grape".to_string(),
        "honeydew".to_string(),
        "kiwi".to_string(),
        "lemon".to_string(),
        "mango".to_string(),
        "nectarine".to_string(),
        "orange".to_string(),
        "papaya".to_string(),
        "quince".to_string(),
        "raspberry".to_string(),
        "strawberry".to_string(),
        "tangerine".to_string(),
        "watermelon".to_string(),
        "blueberry".to_string(),
    ]
}

fn create_large_test_data() -> Vec<String> {
    let base_items = create_test_data();
    let mut large_data = Vec::new();

    for i in 0..50 {
        for item in &base_items {
            large_data.push(format!("{}_{}", item, i));
        }
    }

    large_data
}

fn create_unicode_test_data() -> Vec<String> {
    vec![
        "café".to_string(),
        "naïve".to_string(),
        "résumé".to_string(),
        "piñata".to_string(),
        "jalapeño".to_string(),
        "François".to_string(),
        "München".to_string(),
        "São Paulo".to_string(),
        "北京".to_string(),
        "東京".to_string(),
        "златибор".to_string(),
        "الرياض".to_string(),
    ]
}

fn bench_fuzzy_basic(c: &mut Criterion) {
    let str1 = "hello world";
    let str2 = "helo wrold";

    c.bench_function("fuzzy_basic", |b| {
        b.iter(|| {
            fuzzy(
                black_box(str1.to_string()),
                black_box(str2.to_string()),
                black_box(Some(true)),
            )
        })
    });
}

fn bench_fuzzy_no_normalize(c: &mut Criterion) {
    let str1 = "hello world";
    let str2 = "helo wrold";

    c.bench_function("fuzzy_no_normalize", |b| {
        b.iter(|| {
            fuzzy(
                black_box(str1.to_string()),
                black_box(str2.to_string()),
                black_box(Some(false)),
            )
        })
    });
}

fn bench_fuzzy_unicode(c: &mut Criterion) {
    let str1 = "café naïve résumé";
    let str2 = "cafe naive resume";

    c.bench_function("fuzzy_unicode", |b| {
        b.iter(|| {
            fuzzy(
                black_box(str1.to_string()),
                black_box(str2.to_string()),
                black_box(Some(true)),
            )
        })
    });
}

fn bench_search_small(c: &mut Criterion) {
    let items = create_test_data();
    let query = "app";

    c.bench_function("search_small", |b| {
        b.iter(|| {
            search(
                black_box(query.to_string()),
                black_box(items.clone()),
                black_box(None),
            )
        })
    });
}

fn bench_search_large(c: &mut Criterion) {
    let items = create_large_test_data();
    let query = "apple";

    c.bench_function("search_large", |b| {
        b.iter(|| {
            search(
                black_box(query.to_string()),
                black_box(items.clone()),
                black_box(None),
            )
        })
    });
}

fn bench_search_with_limit(c: &mut Criterion) {
    let items = create_large_test_data();
    let query = "apple";
    let options = SearchOptions {
        limit: Some(5),
        threshold: Some(0.5),
        normalize: Some(true),
        ignore_case: Some(true),
    };

    c.bench_function("search_with_limit", |b| {
        b.iter(|| {
            search(
                black_box(query.to_string()),
                black_box(items.clone()),
                black_box(Some(options.clone())),
            )
        })
    });
}

fn bench_search_high_threshold(c: &mut Criterion) {
    let items = create_test_data();
    let query = "appl";
    let options = SearchOptions {
        limit: None,
        threshold: Some(0.8),
        normalize: Some(true),
        ignore_case: Some(true),
    };

    c.bench_function("search_high_threshold", |b| {
        b.iter(|| {
            search(
                black_box(query.to_string()),
                black_box(items.clone()),
                black_box(Some(options.clone())),
            )
        })
    });
}

fn bench_search_no_normalize(c: &mut Criterion) {
    let items = create_test_data();
    let query = "Apple";
    let options = SearchOptions {
        limit: None,
        threshold: None,
        normalize: Some(false),
        ignore_case: Some(true),
    };

    c.bench_function("search_no_normalize", |b| {
        b.iter(|| {
            search(
                black_box(query.to_string()),
                black_box(items.clone()),
                black_box(Some(options.clone())),
            )
        })
    });
}

fn bench_search_unicode(c: &mut Criterion) {
    let items = create_unicode_test_data();
    let query = "cafe";

    c.bench_function("search_unicode", |b| {
        b.iter(|| {
            search(
                black_box(query.to_string()),
                black_box(items.clone()),
                black_box(None),
            )
        })
    });
}

fn bench_fuzzy_worst_case(c: &mut Criterion) {
    let str1 = "abcdefghijklmnopqrstuvwxyz";
    let str2 = "ZYXWVUTSRQPONMLKJIHGFEDCBA";

    c.bench_function("fuzzy_worst_case", |b| {
        b.iter(|| {
            fuzzy(
                black_box(str1.to_string()),
                black_box(str2.to_string()),
                black_box(Some(true)),
            )
        })
    });
}

fn bench_fuzzy_best_case(c: &mut Criterion) {
    let str1 = "identical string";
    let str2 = "identical string";

    c.bench_function("fuzzy_best_case", |b| {
        b.iter(|| {
            fuzzy(
                black_box(str1.to_string()),
                black_box(str2.to_string()),
                black_box(Some(true)),
            )
        })
    });
}

fn bench_fuzzy_long_strings(c: &mut Criterion) {
    let str1 = "This is a very long string that contains many words and characters to test the performance of the fuzzy matching algorithm with longer inputs";
    let str2 = "This is a very long string that contains some words and characters to test the performance of the fuzzy matching algorithm with longer input";

    c.bench_function("fuzzy_long_strings", |b| {
        b.iter(|| {
            fuzzy(
                black_box(str1.to_string()),
                black_box(str2.to_string()),
                black_box(Some(true)),
            )
        })
    });
}

criterion_group!(
    fuzzy_benches,
    bench_fuzzy_basic,
    bench_fuzzy_no_normalize,
    bench_fuzzy_unicode,
    bench_fuzzy_worst_case,
    bench_fuzzy_best_case,
    bench_fuzzy_long_strings
);

criterion_group!(
    search_benches,
    bench_search_small,
    bench_search_large,
    bench_search_with_limit,
    bench_search_high_threshold,
    bench_search_no_normalize,
    bench_search_unicode
);

criterion_main!(fuzzy_benches, search_benches);
