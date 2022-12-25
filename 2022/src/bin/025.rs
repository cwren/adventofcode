use std::fs;

fn encode(n: isize) -> String {
  // 5, 25, 125, 625, 3_125, 15_625
  let mut rem = n;
    let mut exp: usize = 0;
    while 5_isize.pow(exp as u32) < rem {
        exp += 1;
    }
    exp += 1;
    let mut values = std::iter::repeat(0).take(exp).collect::<Vec<isize>>();
    let mut carry = std::iter::repeat(false).take(exp).collect::<Vec<bool>>();

    for i in (0..exp).rev() {
      let base = 5_isize.pow(i as u32);
      if rem >= base {
        let mut digit = rem / base;
  	rem -= digit * base;
  	if digit > 2 {       // 3 * 5 = 15, -2 * 5 + 25 = 15
   	  digit -= 5;        // 4 * 5 = 20, -1 * 5 + 25 = 20
  	  carry[i] = true;
  	}
  	values[i] = digit;
      }
    }
    
    // resolve all carries in a ripple
    for i in 0..exp {
      for j in (i..exp).rev() {
        if carry[j] {
          carry[j] = false;
  	  values[j + 1] += 1;
  	  if values[j + 1] == 3 {
  	    values[j + 1] = -2;
  	    carry[j + 1] = true;
          }
        }
      }
    }
    // don't emit leading zeros
    while values[values.len() - 1] == 0 {
      values.remove(values.len() - 1);
    }
    // convert to symbols
    values.iter().rev()
    .map(|v| match v {
        2 => "2",
        1 => "1",
        0 => "0",
        -1 => "-",
        -2 => "=",
        _ => panic!("over/underflow on {v}")
    })
    .collect::<String>()
}

fn decode(s: &str) -> isize {
    let mut result = 0;
    for (exponent, c) in s.chars().rev().enumerate() {
        let value = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unexpected digit {c}"),
        };
        result += 5_isize.pow(exponent as u32) * value;
    }
    result
}

fn main() {
    let input: &str = &fs::read_to_string("input/025.txt").expect("file read error");
    println!("please enter: {}", encode(input.lines().map(decode).sum()));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

    #[test]
    fn test_decode() {
        assert_eq!(decode("1=-0-2"), 1747);
        assert_eq!(decode("12111"), 906);
        assert_eq!(decode("2=0="), 198);
        assert_eq!(decode("21"), 11);
        assert_eq!(decode("2=01"), 201);
        assert_eq!(decode("111"), 31);
        assert_eq!(decode("20012"), 1257);
        assert_eq!(decode("112"), 32);
        assert_eq!(decode("1=-1="), 353);
        assert_eq!(decode("1-12"), 107);
        assert_eq!(decode("12"), 7);
        assert_eq!(decode("1="), 3);
        assert_eq!(decode("122"), 37);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(1747), "1=-0-2");
        assert_eq!(encode(906), "12111");
        assert_eq!(encode(198), "2=0=");
        assert_eq!(encode(11), "21");
        assert_eq!(encode(201), "2=01");
        assert_eq!(encode(31), "111");
        assert_eq!(encode(1257), "20012");
        assert_eq!(encode(32), "112");
        assert_eq!(encode(353), "1=-1=");
        assert_eq!(encode(107), "1-12");
        assert_eq!(encode(7), "12");
        assert_eq!(encode(3), "1=");
        assert_eq!(encode(37), "122");
    }
    
    #[test]
    fn test_sum() {
        assert_eq!(encode(SAMPLE.lines().map(decode).sum()), "2=-1=0");
    }
    
    #[test]
    fn test_wordsize() {
        // just curious... passes on an M1
        assert_eq!(isize::MAX, i64::MAX as isize);
        assert_ne!(isize::MAX, i32::MAX as isize);
        assert_ne!(isize::MAX, i128::MAX as isize);
    }

}
