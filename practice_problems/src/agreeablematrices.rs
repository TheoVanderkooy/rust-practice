
/// # greeablematrix
///
/// given two vectors representing the sum of each row/column in a matrix, return a matrix that has the appropriate sum for each row/column if it is possible.
///
/// ## Parameters
/// R: the sum of each row
/// C: the sum of each column
#[allow(non_snake_case)]
pub fn agreeablematrix( R: &Vec<i32>, C: &Vec<i32> ) -> Option<Vec<Vec<i32>>> {
  assert_eq!(R.len(), C.len());

  let n = R.len();
  let s: i32 = R.iter().sum();
  if s != C.iter().sum() {
    return None;
  }

  if n == 0 {
    return Some(vec![vec![]]); // empty matrix
  } else if n == 1 {
    return Some(vec![vec![R[0]]]);
  }

  let mut M = Vec::with_capacity(n);
  for _ in 0..n {
    let mut temp = Vec::with_capacity(n);
    for _ in 0..n {
      temp.push(0);
    }
    M.push(temp);
  }

  M[0][0] = if R[0] < C[0] { R[0] } else { C[0] };
  M[0][1] = R[0] - M[0][0];
  M[1][0] = C[0] - M[0][0];

  for i in 1..n-1 {
    M[i][i+1] = R[i] - M[i][i-1];
    M[i+1][i] = C[i] - M[i-1][i];
  }

  M[n-1][n-1] = C[n-1] - M[n-2][n-1];

  return Some(M);
}

/// like agreeablematrix but input is positive and output is all non-negative
#[allow(non_snake_case)]
pub fn agreeablematrix2( mut R: Vec<i32>, mut C: Vec<i32> ) -> Option<Vec<Vec<i32>>> {
  assert_eq!(R.len(), C.len());

  let n = R.len();
  let s: i32 = R.iter().sum();
  if s != C.iter().sum() {
    return None;
  }

  if n == 0 {
    return Some(vec![vec![]]); // empty matrix
  } else if n == 1 {
    return Some(vec![vec![R[0]]]);
  }

  let mut M = Vec::with_capacity(n);
  for _ in 0..n {
    let mut temp = Vec::with_capacity(n);
    for _ in 0..n {
      temp.push(0);
    }
    M.push(temp);
  }

  let mut row = 0usize;
  let mut col = 0usize;

  while row < n && col < n {
    let temp = if R[row] < C[col] { R[row] } else { C[col] };
    M[row][col] = temp;
    R[row] -= temp;
    C[col] -= temp;
    // println!("R[{}] = {}, C[{}] = {}", row, R[row], col, C[col]);
    if R[row] == 0 { row += 1; }
    if C[col] == 0 { col += 1; }
    // println!("row: {}, col: {}\n", row, col);
  }

  Some(M)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[allow(non_snake_case)] // for R, C, M
  fn check_agreeablematrix(R: Vec<i32>, C: Vec<i32>) -> bool {
    let n = R.len();
    let M = agreeablematrix(&R, &C);
    if R.iter().sum::<i32>() != C.iter().sum::<i32>() {
      return M.is_none();
    }

    let M = M.unwrap();

    for i in 0..n {
      let mut rs = 0;
      let mut cs = 0;
      for j in 0..n {
        rs += M[i][j];
        cs += M[j][i];
      }
      if rs != R[i] || cs != C[i] {
        return false;
      }
    }
    true
  }

  #[allow(non_snake_case)] // for R, C, M
  fn check_agreeablematrix2(R: Vec<i32>, C: Vec<i32>) -> bool {
    let n = R.len();
    let M = agreeablematrix2(R.clone(), C.clone());
    if R.iter().sum::<i32>() != C.iter().sum::<i32>() {
      assert!( M.is_none() );
      return M.is_none();
    }

    let M = M.unwrap();
    // println!("{:?}", M);

    for i in 0..n {
      let mut rs = 0;
      let mut cs = 0;
      for j in 0..n {
        if M[i][j] < 0 {
          return false;
        }
        rs += M[i][j];
        cs += M[j][i];
      }
      if rs != R[i] || cs != C[i] {
        return false;
      }
    }
    true
  }

  #[test]
  fn test_agreeablematrices() {
    assert!( check_agreeablematrix(vec![1], vec![1]) );
    assert!( check_agreeablematrix(vec![1], vec![2]) );
    assert!( check_agreeablematrix(vec![1,2,3], vec![1,3,2]) );
    assert!( check_agreeablematrix(vec![1,0,-2,10], vec![0, 9, 1, -1]) );
    assert!( check_agreeablematrix(vec![1,1,1,1], vec![1, 1, 1, 1]) );
    assert!( check_agreeablematrix(vec![1,1,1,1], vec![1, 1, 1, 2]) );

    let empty = vec![];
    assert_eq!( agreeablematrix(&empty, &empty), Option::Some(vec![vec![]]) );

  }

  #[test]
  fn test_agreeablematrices2() {
    assert!( check_agreeablematrix2(vec![1], vec![1]) );
    assert!( check_agreeablematrix2(vec![1], vec![2]) );
    assert!( check_agreeablematrix2(vec![1,2,3], vec![1,3,2]) );
    assert!( check_agreeablematrix2(vec![4,2,0,10], vec![0, 9, 7, 0]) );
    assert!( check_agreeablematrix2(vec![1,1,1,1], vec![1, 1, 1, 1]) );
    assert!( check_agreeablematrix2(vec![1,1,1,1], vec![1, 1, 1, 2]) );
    assert_eq!( agreeablematrix2( vec![], vec![]), Option::Some(vec![vec![]]) );

  }
}
