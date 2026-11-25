
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David",
        "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let student_index = students.iter()
        .position(|&name| name == student)
        .unwrap();

    let start = student_index * 2;

    let mut result = Vec::new();

    for row in diagram.lines() {
        for plant in row.chars().skip(start).take(2) {
            let name = match plant {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => unreachable!(),
            };

            result.push(name);
        }
    }

    result
}
