const N: usize = 8;

fn try(mut board: &mut [[bool; N]; N], row: usize, mut count: &mut i64) {
   if row == N {
       //Full board, end of recursion, increase solution count and print it out
       *count += 1;
       for r in board.iter() {
           //Print the board with x for queen and . for an empty field
           println!("{}", r.iter().map(|&x| if x {"x"} else {"."}.to_string()).collect::<Vec<String>>().join(" "))
       }
       println!("");
       return
   }
   //Calculating algorithm
   for i in 0..N { //column run
       let mut ok: bool = true;
       for j in 0..row { //row run
           //Check if there's a conflict with an other queen
           if board[j][i]
               || i+j >= row && board[j][i+j-row]
               || i+row < N+j && board[j][i+row-j]
           { ok = false }
       }
       if ok {
           //No conflict with an other queen - set position and start recursion with next row
           board[row][i] = true;
           try(&mut board, row+1, &mut count);
           board[row][i] = false;
       }
   }
}

fn main() {
   let mut board: [[bool; N]; N] = [[false; N]; N]; //board is [row][col] with a boolean flag for a queen
   let mut count: i64 = 0;
   try (&mut board, 0, &mut count);
   //Reference of count given to algorithm, so it can printed out at the end
   println!("Found {} solutions", count)
}
