/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < 0{
        return -1
      }
      let mut sum = 0;
      for i in 1..(n+1) {
        sum += i;
      }
    return sum
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut total = 0;

  for i in ls{

    if *i >= s {
      
      if *i > e {
        break
      }
      total += 1;
    }
  }
  return total
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    if set == target {return true}
  
  for t in target {
    let mut found = false;
    for i in set {
      if t == i {
        found = true;
        break
      }
    }
    if !found {return false}
  }
  return true
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls == [] {return None}
    let mut total = 0 as f64;
    for i in ls {
      total += i
    }
    return Some(total / ls.len() as f64)
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    if ls == []{return 0}
  let mut i = (ls.len() - 1) as i32;
  let mut power = 0;
  let mut dec = 0;
  while i >= 0{
    if ls[i as usize] == 1 {
      dec += 2_i32.pow(power as u32);
    }
    i -= 1;
    power+=1
  }
  return dec
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
  let mut factorized = Vec::new();
  primes.push(2);
  let mut i = 3;
  let mut num = n;
  while i <= n{
    primes.push(i);
    for j in 3..i{
      if  i % j == 0{
        primes.pop();
        break
      }
    }
    i += 2
  }

  while num != 1 {
    let temp = primes.pop().unwrap();
    while num % temp == 0{
      num = num / temp;
      factorized.push(temp);
    }
  }
  factorized.sort_unstable();
  return factorized
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    if lst == [] {return Vec::new()}
    let mut v = lst.to_vec();
    v.rotate_left(1);
    v
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    if target == "" { return true }
    let length = target.chars().count();
    let length2 = s.chars().count();
    let mut i = 0;
    while i < length2{
        if target.chars().nth(0) == s.chars().nth(i) {
        for j in i+1..length + i{
            if target.chars().nth(j) != s.chars().nth(i + j) {
            return false
            }
        }
        return true;
        }
        i += 1
    }

    return false;
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s == "" {return None}
    let mut max = 0;
    let mut current = s.chars().nth(0).unwrap();
    let mut sequence = 0;
    let mut i = 0;
    let mut start_index = 0 ;
    let mut end_index = 0 ;
    while i < s.chars().count() {
      if current == s.chars().nth(i).unwrap() {
        sequence += 1;
      }
  
      else {
        sequence = 1;
      }
  
      if sequence > max {
        max = sequence;
        end_index = i + 1;
        start_index = end_index - max
      }
      
      current = s.chars().nth(i).unwrap();
      i += 1;
    }
    return Some (&s[start_index..end_index])
}
