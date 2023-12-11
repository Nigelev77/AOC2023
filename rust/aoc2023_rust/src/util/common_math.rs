
pub fn get_first_differences_i64(input: &Vec<i64>) -> Vec<i64> {
    let vals = input.iter();
    let next_vals = input.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

pub fn get_first_differences_u64(input: &Vec<u64>) -> Vec<u64> {
    let vals = input.iter();
    let next_vals = input.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}