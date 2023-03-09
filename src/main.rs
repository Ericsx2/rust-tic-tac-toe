use std::io;
use std::process::Command;

fn main() {
    let mut turn = 'X';
    let mut board = [[' '; 3]; 3];

    let winner = loop {
        Command::new("clear").status().unwrap();
        println!("{turn} turns");
        print_board(board);
        let mut row_selected = String::new();
        let mut column_selected = String::new();
        println!("Select row :");
        io::stdin()
            .read_line(&mut row_selected)
            .expect("Please type a valid number!! [0 - 2]");
        println!("Select column: ");
        io::stdin()
            .read_line(&mut column_selected)
            .expect("Please type a valid number!! [0 - 2]");
        let row_selected: usize = row_selected.trim().parse().expect("Not a Number");
        let column_selected: usize = column_selected.trim().parse().expect("Not a Number");

        if !make_move(row_selected, column_selected, &mut board, turn) {
            continue;
        }
        if verify_board(&mut board) {
            break turn;
        }
        switch_turn(&mut turn);
    };

    Command::new("clear").status().unwrap();
    print_board(board);
    println!("{winner} Wins");
}

fn print_board(board: [[char; 3]; 3]) {
    println!("-------------");
    for line in board {
        println!("| {} | {} | {} |", line[0], line[1], line[2]);
        println!("-------------")
    }
}

fn make_move(row: usize, column: usize, board: &mut [[char; 3]; 3], player: char) -> bool {
    if move_is_valid(row, column, board) {
        board[row][column] = player;
        return true;
    }
    return false;
}

fn move_is_valid(row: usize, column: usize, board: &mut [[char; 3]; 3]) -> bool {
    board[row][column] == ' '
}

fn verify_board(board: &mut [[char; 3]; 3]) -> bool {
    for i in 1..3 {
        if board[i][0] == board[i][1]
            && board[i][1] == board[i][2]
            && (board[i][0] != ' ' || board[i][1] != ' ' || board[i][2] != ' ')
        {
            return true;
        }
    }

    for i in 1..3 {
        if board[0][i] == board[1][i]
            && board[1][i] == board[2][i]
            && (board[0][i] != ' ' || board[1][i] != ' ' || board[2][i] != ' ')
        {
            return true;
        }
    }

    if board[0][0] == board[1][1]
        && board[1][1] == board[2][2]
        && (board[0][0] != ' ' || board[1][1] != ' ' || board[2][2] != ' ')
    {
        return true;
    }

    if board[0][2] == board[1][1]
        && board[1][1] == board[2][0]
        && (board[0][2] != ' ' || board[1][1] != ' ' || board[2][0] != ' ')
    {
        return true;
    }

    return false;
}

fn switch_turn(player: &mut char) {
    if *player == 'X' {
        *player = 'O';
    } else {
        *player = 'X';
    }
}
