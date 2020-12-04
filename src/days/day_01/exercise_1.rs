pub fn solve(expense_report: &Vec<u32>) {
    for first_index in 0..expense_report.len() {
        let expense = expense_report[first_index];

        for second_index in first_index..expense_report.len() {
            let other_expense = expense_report[second_index];

            if expense + other_expense == 2020 {
                println!("Solution: {}", expense * other_expense);
            }
        }
    }
}
