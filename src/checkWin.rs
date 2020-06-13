pub fn win(arr: [i32;9]) -> (bool,i32){

    //check horizontal rows
    if arr[0] == arr[1] && arr[1] == arr[2] && arr[0] != 0{return (true,arr[0]);}
    if arr[3] == arr[4] && arr[4] == arr[5] && arr[3] != 0{return (true,arr[3]);}
    if arr[6] == arr[7] && arr[7] == arr[8] && arr[6] != 0{return (true,arr[6]);}

    //check vertical rows
    if arr[0] == arr[3] && arr[3] == arr[6] && arr[0] != 0{return (true,arr[0]);}
    if arr[1] == arr[4] && arr[4] == arr[7] && arr[1] != 0{return (true,arr[1]);}
    if arr[2] == arr[5] && arr[5] == arr[8] && arr[2] != 0{return (true,arr[2]);}

    //check diagnols
    if arr[0] == arr[4] && arr[4] == arr[8] && arr[0] != 0{return (true,arr[0]);}
    if arr[2] == arr[4] && arr[4] == arr[6] && arr[2] != 0{return (true,arr[2]);}

    //check if full
    //if a 0 is found then game is not done
    for i in 0..9 {
        if arr[i] == 0{return (false,0);}
    }

    //end game in a tie
    return (true,0);
}