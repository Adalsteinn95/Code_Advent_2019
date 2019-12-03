pub fn two(commands: &mut [i32]) -> i32 {
    let mut index = 0;

    commands[1] = 12;
    commands[2] = 2;

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
