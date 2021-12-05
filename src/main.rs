use std::fmt;
use std::time::Instant;

impl fmt::Debug for CollatzNumbers {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Hi Collatz")
  }
}

// impl fmt::Debug for Palindrome {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "Hi Palindrome")
//   }
// }

#[derive(PartialEq)]
pub struct CollatzNumbers {
  loops_in: i128,
  stops: usize,
  values: Vec<i128>,
}

fn collatz(number: i128, vec: &Vec<i128>) -> CollatzNumbers {
  let mut arr = vec.clone();
  let hailstone = if number % 2 == 0 { number / 2 } else { number * 3 + 1 };


  let op_2_bool = arr.iter().find(|&&index| index == hailstone);
  // if arr.iter().any(|&index| index == hailstone) {
  if !op_2_bool.is_none() {
    CollatzNumbers {
      loops_in: hailstone,
      stops: arr.len(),
      values: arr,
    }
  } else {
    arr.push(hailstone);
    collatz(hailstone, &arr)
  }
}

// #[derive(PartialEq)]
// pub struct Palindrome {
//   jumps: usize,
//   values: Vec<u128>,
//   palindrome: u128,
// }

// fn reverse_digits(num: u128) -> u128 {
//   let digits: String = num.to_string()
//     .chars()
//     .map(|d| d.to_digit(10).unwrap())
//     .rev()
//     .map(|digit| digit.to_string())
//     .collect();
  
//   let numeric: u128 = digits.parse().unwrap();
//   numeric
// }

// fn suma_reverse(number: u128, acc: &Vec<u128>) -> Palindrome {
//   let reverse_number = reverse_digits(number);
//   let mut values = acc.clone();

//   if number == reverse_number {
//     Palindrome {
//       jumps: acc.len(),
//       values: values,
//       palindrome: number,
//     }
//   } else {
//     let sigma = number + reverse_number;
//     values.push(sigma);

//     suma_reverse(sigma, &values)
//   }
// }

fn main() {
    let my_vec = vec![];
    // let my_acc = vec![];
    // let number: i128 = 0xFFEDCBAA_FFEDCBAA_FFEDCBAB;
    let number: i128 = 8400511;
    // let number: i128 = 8400509;
    let collatz_ini = Instant::now();
    let _c = collatz(number, &my_vec);
    let collatz_end = Instant::now();
    let d = collatz_end.duration_since(collatz_ini);
    println!("Duration {:?}", d);

    // let palindrome = suma_reverse(177, &my_acc);
    // println!("");
    // println!("PALINDROMES");
    // println!(
    //   "jumps: {}, values: {:?}, palindrome: {}",
    //   palindrome.jumps, palindrome.values, palindrome.palindrome
    // );

    // println!("");
    // println!("COLLATZ");
    // println!(
    //   "loops in: {},  number of stops: {}, values: {:?}",
    //   c.loops_in, c.stops, c.values
    // );
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_collatz() {
    let my_result = CollatzNumbers {
      loops_in: 4,
      stops: 6,
      values: vec![5, 16, 8, 4, 2, 1]
    };
    let empty_vec: Vec<i128> = vec![];

     assert_eq!(collatz(10, &empty_vec), my_result)
  }

  // #[test]
  // fn test_suma_reverse() {
  //   let my_result = Palindrome {
  //     jumps: 15,
  //     values: vec![
  //       948, 1797, 9768, 18447, 92928, 175857, 934428, 1758867, 9447438, 17794887, 96644658, 182289327, 906271608, 1712444217, 8836886388
  //     ],
  //     palindrome: 8836886388,
  //   };

  //   let empty_vec: Vec<u128> = vec![];
  //   assert_eq!(suma_reverse(177, &empty_vec), my_result);
  // }

  // #[test]
  // fn test_reverse_digits() {
  //   let expected = 321;
  //   assert_eq!(reverse_digits(123), expected);
  //   let expected = 2;
  //   assert_eq!(reverse_digits(2), expected);
  // }
}