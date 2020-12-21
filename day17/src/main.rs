use std::collections::HashSet;

const INPUT: &str = r#"
##....#.
#.#..#..
...#....
...#.#..
###....#
#.#....#
.#....##
.#.###.#
"#;

fn parse() -> HashSet<(i32, i32, i32)> {
    INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(x, l)| {
            l.chars().enumerate().map(move |(y, c)| match c {
                '#' => Some((x as i32, y as i32, 0 as i32)),
                _ => None,
            })
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

fn offset_iter3() -> impl Iterator<Item = (i32, i32, i32)> {
    (-1..=1)
        .flat_map(|x| {
            (-1..=1)
                .map(move |y| (x, y))
                .flat_map(|(x, y)| (-1..=1).map(move |z| (x, y, z)))
        })
        .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
}

fn step3(before: &HashSet<(i32, i32, i32)>, after: &mut HashSet<(i32, i32, i32)>) {
    let min_x = before.iter().min_by_key(|(x, _, _)| *x).unwrap().0;
    let max_x = before.iter().max_by_key(|(x, _, _)| *x).unwrap().0;
    let min_y = before.iter().min_by_key(|(_, y, _)| *y).unwrap().1;
    let max_y = before.iter().max_by_key(|(_, y, _)| *y).unwrap().1;
    let min_z = before.iter().min_by_key(|(_, _, z)| *z).unwrap().2;
    let max_z = before.iter().max_by_key(|(_, _, z)| *z).unwrap().2;
    after.clear();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                if before.contains(&(x, y, z)) {
                    if (2..=3).contains(
                        &offset_iter3()
                            .map(|(xo, yo, zo)| (x + xo, y + yo, z + zo))
                            .filter(|p| before.contains(p))
                            .count(),
                    ) {
                        after.insert((x, y, z));
                    }
                } else if offset_iter3()
                    .map(|(xo, yo, zo)| (x + xo, y + yo, z + zo))
                    .filter(|p| before.contains(p))
                    .count()
                    == 3
                {
                    after.insert((x, y, z));
                }
            }
        }
    }
}

fn offset_iter4() -> impl Iterator<Item = (i32, i32, i32, i32)> {
    (-1..=1)
        .flat_map(|w| {
            (-1..=1)
                .flat_map(|x| {
                    (-1..=1)
                        .map(move |y| (x, y))
                        .flat_map(|(x, y)| (-1..=1).map(move |z| (x, y, z)))
                })
                .map(move |(x, y, z)| (x, y, z, w))
        })
        .filter(|(x, y, z, w)| *x != 0 || *y != 0 || *z != 0 || *w != 0)
}

fn step4(before: &HashSet<(i32, i32, i32, i32)>, after: &mut HashSet<(i32, i32, i32, i32)>) {
    let min_x = before.iter().min_by_key(|(x, _, _, _)| *x).unwrap().0;
    let max_x = before.iter().max_by_key(|(x, _, _, _)| *x).unwrap().0;
    let min_y = before.iter().min_by_key(|(_, y, _, _)| *y).unwrap().1;
    let max_y = before.iter().max_by_key(|(_, y, _, _)| *y).unwrap().1;
    let min_z = before.iter().min_by_key(|(_, _, z, _)| *z).unwrap().2;
    let max_z = before.iter().max_by_key(|(_, _, z, _)| *z).unwrap().2;
    let min_w = before.iter().min_by_key(|(_, _, _, w)| *w).unwrap().3;
    let max_w = before.iter().max_by_key(|(_, _, _, w)| *w).unwrap().3;
    after.clear();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for w in (min_w - 1)..=(max_w + 1) {
                    if before.contains(&(x, y, z, w)) {
                        if (2..=3).contains(
                            &offset_iter4()
                                .map(|(xo, yo, zo, wo)| (x + xo, y + yo, z + zo, w + wo))
                                .filter(|p| before.contains(p))
                                .count(),
                        ) {
                            after.insert((x, y, z, w));
                        }
                    } else if offset_iter4()
                        .map(|(xo, yo, zo, wo)| (x + xo, y + yo, z + zo, w + wo))
                        .filter(|p| before.contains(p))
                        .count()
                        == 3
                    {
                        after.insert((x, y, z, w));
                    }
                }
            }
        }
    }
}

fn part1(mut world: HashSet<(i32, i32, i32)>) -> usize {
    let mut other = HashSet::new();
    for _ in 0..6 {
        step3(&world, &mut other);
        std::mem::swap(&mut world, &mut other);
    }

    world.len()
}

fn part2(mut world: HashSet<(i32, i32, i32, i32)>) -> usize {
    let mut other = HashSet::new();
    for _ in 0..6 {
        step4(&world, &mut other);
        std::mem::swap(&mut world, &mut other);
    }

    world.len()
}

fn main() {
    let world = parse();
    println!("part 1: {}", part1(world.clone()));
    println!(
        "part 2: {}",
        part2(world.into_iter().map(|(x, y, z)| (x, y, z, 0)).collect())
    );
}
