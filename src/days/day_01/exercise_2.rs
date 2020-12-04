pub fn solve(expense_report: &Vec<u32>) {
    for first_index in 0..expense_report.len() {
        let first_expense = expense_report[first_index];

        for second_index in first_index..expense_report.len() {
            let second_expense = expense_report[second_index];

            for third_index in second_index..expense_report.len() {
                let third_expense = expense_report[third_index];

                if first_expense + second_expense + third_expense == 2020 {
                    println!(
                        "Solution: {}",
                        first_expense * second_expense * third_expense
                    );
                }
            }
        }
    }
}
