pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn align_to_16_bit(raw: usize) -> usize {
    return (raw + 15) & (!15);
}

pub fn fix_offset(raw:String)->String{
    let ref this = raw;
    let mut result = String::new();
    let mut last_end = 0;
    for (start, part) in this.match_indices("\t") {
        result.push_str(unsafe { this.get_unchecked(last_end..start) });
        result.push_str("    ");
        last_end = start + part.len();
    }
    result.push_str(unsafe { this.get_unchecked(last_end..this.len()) });
    return result
}
