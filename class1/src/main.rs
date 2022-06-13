use regex::Regex;

// Exercise 1: check whether an array is subarray of another array
fn is_sub_array(_a: &mut [u64], _b: &mut [u64]) -> bool {
    let m = _a.len();
    let n = _b.len();

    let mut i = 0;
    let mut j = 0;

    while i < m && j < n {
        if _a[i] == _b[j] {
            i += 1;
            j += 1;

            if j == n {
                return true
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }

    return false
}


// Exercise 2: count words in text
fn count_word_in_text(_s: &str, _w: &str) -> usize {
    let re = Regex::new(format!(r#"(?i){}"#, _w).as_str()).unwrap();
    let result = re.find_iter(_s).count();
    return result;
}

fn main() {
    // Exercise 1 tests
    println!("---- EXERCISE 1 RESULT ----");

    let mut a:[u64;3] = [1, 2, 3];
    let mut b:[u64;2] = [1, 2];
    
    let r1 = is_sub_array(&mut a, &mut b);
    println!("should be true, got {}", r1);

    let mut c:[u64;3] = [1, 2, 3];
    let mut d:[u64;2] = [1, 4];
    
    let r2 = is_sub_array(&mut c, &mut d);
    println!("should be false, got {}", r2);

    println!("---------------------------");

    // Exercise 2 tests
    println!("\n---- EXERCISE 2 RESULT ----");

    let s = "This is foo and FOO foo as well as FoO.";
    let w = "foo";
    let n = count_word_in_text(s, w);
    println!("{} appears {} time(s) in the given string.", w, n);
}
