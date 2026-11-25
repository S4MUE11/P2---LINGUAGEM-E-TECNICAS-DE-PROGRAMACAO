
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David",
        "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let start = students.iter()
        .position(|&name| name == student)
        .unwrap() * 2;

    diagram.lines()
        .flat_map(|row| {
            row.chars()
                .skip(start)
                .take(2)
                .map(|plant| match plant {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => unreachable!(),
                })
        })
        .collect()
}
