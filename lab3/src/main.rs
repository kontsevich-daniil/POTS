fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3]; // создаём пустую матрицу для результата
    for i in 0..3 { // проходим по строкам исходной матрицы
        for j in 0..3 { // проходим по столбцам
            result[j][i] = matrix[i][j]; // меняем индексы местами
        }
    }
    result
}

fn print_matrix(matrix: &[[i32; 3]; 3], label: &str){
    println!("{}", label);
    
    for row in matrix {
        for &val in row {
            print!("{:>4} ", val);
        }
        println!();
    }
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    print_matrix(&matrix, "Матрица:");

    let transposed = transpose(matrix);
    print_matrix(&transposed, "Транспонированная матрица:");
}