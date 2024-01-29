use std::result;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let Letters = vec![
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
        vec!["j", "k", "l"],
        vec!["m", "n", "o"],
        vec!["p", "q", "r", "s"],
        vec!["t", "u", "v"],
        vec!["w", "x", "y", "z"],
    ];

    let digits_char = digits.chars();
    let mut PossiblesLetters: Vec<Vec<&str>> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    for x in digits_char {
        PossiblesLetters.push(Letters[x.to_digit(10).unwrap() as usize - 2].to_vec());
    }

    println!("{:?}", PossiblesLetters);

    for x in PossiblesLetters {
        //println!("x={:?}", x);
        for y in x {
            println!("y={:?}", y);
        }
    }

    //result.dedup();
 
    return vec!["".to_string()];

}

fn main() {
    let result = letter_combinations("23".to_string());
    println!("result = {:?}", result);
}
