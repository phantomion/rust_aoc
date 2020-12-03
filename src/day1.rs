pub fn day1_part1(input: &str) -> i32 {
    let mut hash_table: [i32; 2020] = [0; 2020];
    let input = input
        .lines()
        .map(|number| {
            let index = number.parse::<usize>().unwrap();
            hash_table[index] = 1;
            return number.parse().unwrap();
        })
        .collect::<Vec<i32>>();

    let result = input
        .iter()
        .find(|&&number| {
            let index = number as usize;
            hash_table[2020 - index] == 1
        })
        .unwrap()
        .to_owned();

    let result = result * (2020 - result);
    return result;
}

pub fn day1_part2(_input: &str) -> i32 {
    let mut _hash_table: [i32; 2020] = [0; 2020];
    let mut _hash_table2: [i32; 2020] = [0; 2020];

    //let input = input
        //.lines()
        //.map(|number| {
            //let index = number.parse::<usize>().unwrap();
            //hash_table[index] = index as i32;
            //number.parse().unwrap()
        //})
        //.collect::<Vec<i32>>();

    //let hash_table = hash_table
        //.iter()
        //.filter(|&&number| number != 0)
        //.map(|&number| {
            //println!("{}", number);
            //let input = input
                //.iter()
                //.filter(|&&lnumber| lnumber + number < 2020)
                //.map(|&lnumber| {
                    //println!("{} {}", number, lnumber);
                    //hash_table2[(lnumber + number) as usize] = 1;
                    //lnumber
                //})
                //.collect::<Vec<i32>>();
            //number
        //})
        //.collect::<Vec<i32>>();

    //let result = input
        //.iter()
        //.map(|&lnumber| {
            //let found = hash_table
                //.iter()
                //.filter(|&&hash| hash != 0 && lnumber + hash < 2020)
                //.find(|&&hash| hash_table2[2020 - (lnumber + hash) as usize] == 1);

            //let found = if found.is_some() {
                //found.unwrap().to_owned()
            //} else {
                //0
            //};
            //println!("{}", found);
            //found
        //})
        //.collect::<Vec<i32>>();

    //let index = result
        //.iter()
        //.find(|&&number| number !=0 )
        //.unwrap()
        //.to_owned();
    //let index = result.iter().nth(index as usize).unwrap().to_owned();

    //let final_result = input
        //.iter()
        //.filter(|&&lnumber| index + lnumber < 2020)
        //.find(|&lnumber| hash_table[2020 - (index + lnumber) as usize] != 0)
        //.unwrap()
        //.to_owned();

    //let final_result = final_result * index * (2020 - index - final_result);
    return 1;
}
