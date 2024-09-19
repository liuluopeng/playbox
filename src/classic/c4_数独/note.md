对于一个有多个解决办法的数独, 请你写一个回溯函数, 求出所有的解. 可以用任意语言.

以下是一个使用Python编写的回溯函数，用于解决数独问题并找到所有的解：
```python
def is_valid(board, row, col, num):
    # Check if the number is already in the row
    if num in board[row]:
        return False

    # Check if the number is already in the column
    if num in [board[i][col] for i in range(9)]:
        return False

    # Check if the number is already in the 3x3 subgrid
    start_row, start_col = 3 * (row // 3), 3 * (col // 3)
    for i in range(start_row, start_row + 3):
        for j in range(start_col, start_col + 3):
            if board[i][j] == num:
                return False

    return True

def solve_sudoku(board):
    def backtrack(row, col):
        if row == 9:
            solutions.append([row[:] for row in board])
            return

        next_row = row + 1 if col == 8 else row
        next_col = 0 if col == 8 else col + 1

        if board[row][col] != 0:
            backtrack(next_row, next_col)
        else:
            for num in range(1, 10):
                if is_valid(board, row, col, num):
                    board[row][col] = num
                    backtrack(next_row, next_col)
                    board[row][col] = 0

    solutions = []
    backtrack(0, 0)
    return solutions

# Example usage
board = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9]
]

solutions = solve_sudoku(board)
for solution in solutions:
    for row in solution:
        print(row)
    print()
```
这个函数会打印出数独问题的所有解。你可以将数独题目输入到 board 变量中，然后运行这个函数来找到所有的解。