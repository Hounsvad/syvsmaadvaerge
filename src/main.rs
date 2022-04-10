use std::collections::HashMap;
use strfmt::strfmt;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
struct Dwarf<'a> {
    name: &'a str,
    add_dwarf_reaction: &'a str,
    leave_reaction: &'a str,
    last_reaction: &'a str,
}

fn get_dwarfes<'a>() -> Vec<Dwarf<'a>> {
    let mut dwarves = Vec::new();
    let mut dwarf_doc = Dwarf
    {
        name: "Doc",
        add_dwarf_reaction: "Doc needs to talk to {other_dwarf}. He yells \"Where are you {other_dwarf}\"",
        leave_reaction: "Doc has gotten annoyed with the others, and decides to clear his head with a walk.",
        last_reaction: "Doc wonders where the other dwarves are. Have something occured that he is not aware of? Unsure, he goes looking for the others.",
    };
    dwarves.push(dwarf_doc);
    let mut dwarf_sleepy = Dwarf
    {
        name: "Sleepy",
        add_dwarf_reaction: "Sleepy comes in and says \"Arhgh {other_dwarf}, you are so loud that i can't sleep\"",
        leave_reaction: "Sleepy so sleepy that he can no longer stay awake, colapses on the spot at starts snoring.",
        last_reaction: "Sleepy proclaims his desire to sleep, and then goes to bed to take a nap.",
    };
    dwarves.push(dwarf_sleepy);
    let mut dwarf_grumpy = Dwarf
    {
        name: "Grumpy",
        add_dwarf_reaction: "Grumpy yells at {other_dwarf} for seemingly no reason.",
        leave_reaction: "Puffed up with rage, Grumpy decides to leave the room.",
        last_reaction: "\"If you are all just gonna leave, then ill just go as well\" Grumpy says, and then leaves the room angrily.",
    };
    dwarves.push(dwarf_grumpy);
    let mut dwarf_sneezy = Dwarf
    {
        name: "Sneezy",
        add_dwarf_reaction: "Sneezy accidentally sneezes at {other_dwarf}",
        leave_reaction: "A puff of flower pollen blowes in through the window, and Sneezy starts sneezing so hard that he blows himself out of the room.",
        last_reaction: "With a puffy noze, Sneezy looks around and sees that he is the only one left in the room, he then blows his nose and leaves.",
    };
    dwarves.push(dwarf_sneezy);
    return dwarves;
}

fn addDwarf<'a, 'b>(dwarves: &'a Vec<Dwarf<'a>>, currentDwarves: &Vec<String>) -> Option<&'a Dwarf<'a>> {
    if dwarves.len() > currentDwarves.len(){
        let mut rng = ChaCha20Rng::from_entropy();
        let random_dwarf = rng.gen_range(0..dwarves.len()); //..= means inclusive range
        let mut dwarf;
        dwarf = &dwarves[random_dwarf];
        while currentDwarves.contains(&dwarf.name.to_string()) {
            let rnd = rng.gen_range(0..dwarves.len()); //..= means inclusive range
            dwarf = &dwarves[rnd];
        }
        return Some(dwarf);
    }
    return None;
}


fn main() {
    let dwarves: Vec<Dwarf> = get_dwarfes();
    let mut rng = ChaCha20Rng::from_entropy();
    let mut currentDwarves = Vec::new() as Vec<Dwarf>;
    let mut otherDwarvesThatAreReallyTheSame = dwarves.clone();
    let mut result;
    let mut usedDwarves = Vec::new();
    result = *addDwarf(& otherDwarvesThatAreReallyTheSame, &usedDwarves).unwrap();
    currentDwarves.push(result);
    usedDwarves.push(result.name.to_string());

    result = *addDwarf(& otherDwarvesThatAreReallyTheSame, &usedDwarves).unwrap();
    currentDwarves.push(result);
    usedDwarves.push(result.name.to_string());
    println!("Syv små dværge interaktions simulator: \n");
    let mut names: String = currentDwarves[0].name.to_string() + ", " + currentDwarves[1].name;
    println!("{}", names);
    while currentDwarves.len() > 0 {
        //println!("{0} {1:?}",currentDwarves.len(), usedDwarves);
        let currentCurrentDwarves = currentDwarves.clone();
        let dwarf = &currentCurrentDwarves[currentCurrentDwarves.len()-1];
        

        if currentDwarves.len() == 1 {
            println!("{}",
            dwarf.last_reaction);
            return;
        }
        else{
            if rng.gen_range(0.0..=1.0) < 0.5f32 && usedDwarves.len() < dwarves.len() {
                let mut available = vec![];

                for i in 0..dwarves.len() {
                    if !usedDwarves.contains(&dwarves[i].name.to_string()) {
                        //println!("Dwarf {} is available", dwarves[i].name);
                        available.push(dwarves[i].name.to_string());
                    }else{
                        //println!("Dwarf {} is not available", dwarves[i].name);
                    }
                }
                

                let resultDwarf = *addDwarf(& otherDwarvesThatAreReallyTheSame, &usedDwarves).unwrap();
                //println!("Debug Available {0:?}, Selected {1}", available, resultDwarf.name);
                currentDwarves.push(resultDwarf);
                usedDwarves.push(resultDwarf.name.to_string());

                let mut var = HashMap::new();
                var.insert("other_dwarf".to_string(), resultDwarf.name);
                println!(
                    "{}",
                    strfmt(&dwarf.add_dwarf_reaction.to_string(), &var).unwrap()
                );
            }else{
                println!(
                    "{}",
                    dwarf.leave_reaction);
                currentDwarves.pop();
            }
        }
    }
}
