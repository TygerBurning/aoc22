pub fn day01() {
    let readings = include_str!("../inputs/day01.txt");
    let groups = readings.split("\n\n").map(|elf_group|
        elf_group.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>()
    );

    let top_3 = groups.fold([0,0,0], |mut acc, elem| {
        // lol. Kinda cute though.
        if elem > acc[0] {
            if acc[0] > acc[1] {
                if acc[1] > acc[2] {
                    acc[2] = acc[1];
                }
                acc[1] = acc[0]
            }
            acc[0] = elem;
        }
        else if elem > acc[1] {
            if acc[1] > acc[2] {
                acc[2] = acc[1];
            }
            acc[1] = elem;
        }
        else if elem > acc[2] {
            acc[2] = elem;
        }
        acc
    });

    println!("Part A answer is: {:?}", top_3.iter().max().unwrap());
    println!("Part B answer is: {:?}", top_3.iter().sum::<u32>());
}
