fn main() {
    // if else
        let num = 43;

        if num < 43 { // Braces {} compulsion, if not syntax err
            print!("Hyalo\n")
        } // Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match
        else {
            print!("No hyalo\n")
        }
        // Else if else if else also allowed

        // if number { // Throws err, condition must be a bool
            // print!("Something")
        // }

        // This also allowed
        let x = if true {54} else {53}; // Both types must be same
        print!("Value of x = {x}\n");


    // Loops
        // loop{   // infinite
        //     println!("Something")
        // }

        let mut count = 0;
        let value_returned_from_loop = loop {
                count = count + 1;
                if count == 10{
                    break count;
                }
        };

        print!("Value returned from loop = {value_returned_from_loop}\n");
        
        // Labels in loops
            let mut count = 0;
            'outerLoop: loop{   // Loop labels must begin with single quote
                print!("Count: {count}\t");
                let mut inner_count = 10;
                loop{
                    print!("Inner count: {inner_count}\t");
                    if count == 3 {break 'outerLoop};
                    inner_count = inner_count - 1;
                    if inner_count == 8 {break;} // Brak by default exits the inner loop
                }
                count = count + 1;
                println!();
            }
            println!();

        // Conditional loops
            // while
            let an_array = [1,2,3,4,5];
            let mut index = 0;
            while index < 5 {
                println!("Value at {index}: {}", an_array[index]);
                index += 1;
            }
            // Since while may panic if the index value or test condition is incorrect.
            // so use for

            // for, mostly used
            for elem in an_array{
                println!("Element: {}", elem);
            }
            // increased safety, no index out of bounds

            for elem in (10..15).rev(){
                println!("Element: {elem}");
            }
            

}
