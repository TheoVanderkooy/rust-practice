
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
}
