def is_valid(board, row, col, num):
    for i in range(9):
        if board[row][i] == num or board[i][col] == num:
            return False
    start_row, start_col = 3 * (row // 3), 3 * (col // 3)
    for i in range(3):
        for j in range(3):
            if board[i + start_row][j + start_col] == num:
                return False
    return True

def solve_sudoku(board):
    empty = find_empty(board)
    if not empty:
        return True
    row, col = empty
    for num in range(1, 10):
        if is_valid(board, row, col, num):
            board[row][col] = num
            if solve_sudoku(board):
                return True
            board[row][col] = 0
    return False

def find_empty(board):
    for i in range(9):
        for j in range(9):
            if board[i][j] == 0:
                return (i, j)
    return None

def save_solution_to_file(solution, solution_number):
    with open("sudoku_solutions.txt", "a") as file:
        file.write(f"Solution {solution_number}:\n")
        for row in solution:
            file.write(" ".join(map(str, row)) + "\n")
        file.write("\n")

def load_last_solution_number():
    try:
        with open("sudoku_solutions.txt", "r") as file:
            lines = file.readlines()
            if lines:
                last_line = lines[-2]
                return int(last_line.split()[1])
    except FileNotFoundError:
        return 0

board = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9]
]

solution_number = load_last_solution_number() + 1
if solve_sudoku(board):
    save_solution_to_file(board, solution_number)
    print("Solution found and saved to sudoku_solutions.txt")
else:
    print("No solution found")

