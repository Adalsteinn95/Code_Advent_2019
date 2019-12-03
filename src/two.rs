pub fn two(commands: &mut [i32]) -> i32 {
    let mut index = 0;

    loop {
        match commands[index] {
            1 => {
                commands[commands[index + 3] as usize] =
                    commands[commands[index + 1] as usize] + commands[commands[index + 2] as usize]
            }
            2 => {
                commands[commands[index + 3] as usize] =
                    commands[commands[index + 1] as usize] * commands[commands[index + 2] as usize]
            }
            99 => return commands[0],
            _ => panic!("Unkown commands: {}", commands[index]),
        }
        index += 4;
    }
}

pub fn twopart2(commands: &[i32]) -> i32 {
    loop {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut commands_cp = Vec::from(commands);

                commands_cp[1] = noun;
                commands_cp[2] = verb;

                if two(&mut commands_cp) == 19_690_720 {
                    return 100 * noun + verb;
                }
            }
        }
    }
}
