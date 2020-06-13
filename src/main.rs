use std::io::{self, Read};
mod chart;
mod checkWin;
mod intro;

fn main() {
    let mut playerO = String::new();
    let mut playerX = String::new();
    let mut inputs: [i32;9] = [0,0,0,0,0,0,0,0,0];
    let mut currMove: i32;
    let mut currMoveSTR = String::new();
    let mut gameWin:(bool,i32) = (false,0);

    //display intro to the game
    intro::showIntro();
    //create the players
    //ask for player 1 name
    println!("Please enter player 1's name: ");
    //set player 1 name and verify it works
    match io::stdin().read_line(&mut playerO){
        Ok(_) => { println!("You entered: {}", playerO.trim()); },
        Err(e) => println!("ERROR encountered! {}", e)
    }
    //ask for player 2 name
    println!("Please enter player 2's name: ");
    //set player 2 name and verify it works
    match io::stdin().read_line(&mut playerX){
        Ok(_) => { println!("You entered: {}", playerX.trim()); },
        Err(e) => println!("ERROR encountered! {}", e)
    }

    //begin game loop
    while gameWin.0 == false {
        //clear screen
        print!("{}[2J", 27 as char);

        //display the tic-tac-toe chart
        chart::showChart(inputs);

        //ask for player 1 move
        println!("{}, submit which number you wish to place your O.", playerO.trim());
        //get player 1 move
        std::io::stdin().read_line(&mut currMoveSTR).expect("Failed to read from stdin");
        println!("test");
        //convert from string to int and assign value
        currMove = currMoveSTR.trim().parse::<i32>().unwrap();
        currMove = currMove - 1;
        //make change to inputs
        if currMove >= 0 && currMove < 9 && inputs[currMove as usize] == 0{ inputs[currMove as usize] = 1; }
        else { println!("INVALID MOVE!"); continue; }

        //display t-t-t chart
        chart::showChart(inputs);
        currMoveSTR.clear();
        currMove = 0;

        //check for win or tie
        gameWin = checkWin::win(inputs);
        if gameWin.0 == true{ break; }


        //ask for player 2 move
        println!("{}, submit which number you wish to place your X.", playerX.trim());
        //get player 2 move
        std::io::stdin().read_line(&mut currMoveSTR).expect("Failed to read from stdin");
        //convert from string to int and assign value
        currMove = currMoveSTR.trim().parse::<i32>().unwrap();
        currMove = currMove - 1;
        //make change to inputs
        if currMove >= 0 && currMove < 9 && inputs[currMove as usize] == 0{ inputs[currMove as usize] = -1; }
        else { println!("INVALID MOVE!"); continue; }

        currMoveSTR.clear();
        currMove = 0;

        //check for win or tie
        gameWin = checkWin::win(inputs);
        if gameWin.0 == true{ break; }
    }
    //declare winner
    if gameWin.1 == 1 {println!("{} IS THE WINNER!!!", playerO.trim());}
    else if gameWin.1 == 0 {println!("ITS A TIE! NO ONE WINS!");}
    else {println!("{} IS THE WINNER!!!", playerX.trim());}

}
