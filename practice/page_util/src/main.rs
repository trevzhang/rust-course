use std::collections::HashSet;

struct PageResultDTO {
    records: HashSet<i32>,
    current: Option<i32>,
    size: Option<i32>,
    total: i32,
}

fn is_last_page(page_result: &PageResultDTO) -> bool {
    if page_result.records.is_empty() {
        return true;
    }
    match (page_result.current, page_result.size) {
        (Some(current), Some(size)) => current * size >= page_result.total,
        _ => false,
    }
}

fn main() {
    let mut rec = HashSet::new();
    rec.insert(3);
    let page_result = &PageResultDTO { records: rec, current: Some(2), size: Some(10), total:20 };
    let b: bool = is_last_page(page_result);
    assert!(b);
    println!("Passed!");
}