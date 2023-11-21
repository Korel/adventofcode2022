import math
import sys

def parse_input(filename) -> list[list[str], list[int], list[int]]:
    grid = []
    start = [-1, -1]
    end = [-1, -1]
    with open(filename) as f:
        for line in f.readlines():
            if start[1] == -1:
                start[0] += 1
            if end[1] == -1:
                end[0] += 1
            row = []
            for char in line.strip():
                if char == 'S':
                    start[1] = line.index(char)
                    row.append(0)
                    continue
                if char == 'E':
                    end[1] = line.index(char)
                    row.append(ord('z') - ord('a'))
                    continue
                row.append(ord(char) - ord('a'))
            grid.append(row)
    return grid, start, end


class Solver:
    grid: list[list[int]]
    memo: list[list[int]]
    start: list[int]
    end: list[int]

    def __init__(self, grid: list[list[int]], start: list[int], end: list[int]):
        self.grid = grid
        self.start = start
        self.end = end
        self.memo = [[0 for _ in range(len(grid[0]))]
                     for _ in range(len(grid))]

    def move(self, curr_pos):
        if curr_pos == self.end:
            return 0
        [row, col] = curr_pos
        if self.memo[row][col]:
            return self.memo[row][col]

        self.memo[row][col] = math.inf

        up, down, right, left = math.inf, math.inf, math.inf, math.inf
        curr = self.grid[row][col]

        if row - 1 >= 0:
            if self.grid[row - 1][col] <= curr or self.grid[row - 1][col] - curr <= 1:
                up = self.move([row - 1, col]) + 1
        if row + 1 < len(self.grid):
            if self.grid[row + 1][col] <= curr or self.grid[row + 1][col] - curr <= 1:
                down = self.move([row + 1, col]) + 1
        if col - 1 >= 0:
            if self.grid[row][col - 1] <= curr or self.grid[row][col - 1] - curr <= 1:
                left = self.move([row, col - 1]) + 1
        if col + 1 < len(self.grid[0]):
            if self.grid[row][col + 1] <= curr or self.grid[row][col + 1] - curr <= 1:
                right = self.move([row, col + 1]) + 1

        self.memo[row][col] = min(up, down, left, right)
        return self.memo[row][col]
    
    def solve(self):
        self.solution = [[math.inf for _ in range(len(self.grid[0]))]
                     for _ in range(len(self.grid))]
        self.solution[self.end[0]][self.end[1]] = 0
        self.traverse(self.end)
        return self.solution[self.start[0]][self.start[1]]
    
    def traverse(self, curr_pos):
        if curr_pos == self.start:
            return
        [row, col] = curr_pos
        curr = self.grid[row][col]
        if row - 1 >= 0:
            if curr - self.grid[row - 1][col] <= 1:
                if self.solution[row - 1][col] > self.solution[row][col] + 1:
                    self.solution[row - 1][col] = self.solution[row][col] + 1
                    self.traverse([row - 1, col])
                
        if row + 1 < len(self.grid):
            if curr - self.grid[row + 1][col] <= 1:
                if self.solution[row + 1][col] > self.solution[row][col] + 1:
                    self.solution[row + 1][col] = self.solution[row][col] + 1
                    self.traverse([row + 1, col])
        if col - 1 >= 0:
            if curr - self.grid[row][col - 1] <= 1:
                if self.solution[row][col - 1] > self.solution[row][col] + 1:
                    self.solution[row][col - 1] = self.solution[row][col] + 1
                    self.traverse([row, col - 1])
        if col + 1 < len(self.grid[0]):
            if curr - self.grid[row][col + 1] <= 1:
                if self.solution[row][col + 1] > self.solution[row][col] + 1:
                    self.solution[row][col + 1] = self.solution[row][col] + 1
                    self.traverse([row, col + 1])

def first_task():
    sys.setrecursionlimit(1300)
    test_input = parse_input("test.txt")
    test_solver = Solver(test_input[0], test_input[1], test_input[2])
    test_solution = test_solver.solve()
    print(f"test input: {test_solution}")
    assert(test_solution == 31)
    
    [grid, start, end] = parse_input("input.txt")
    solver = Solver(grid, start, end)
    solution = solver.solve()
    print(f"real input: {solution}")
    assert(solution == 370)

def second_task():
    sys.setrecursionlimit(1300)
    [test_grid, test_start, test_end]= parse_input("test.txt")
    test_solver = Solver(test_grid, test_start, test_end)
    test_solution = math.inf
    for i in range(len(test_grid)):
        for j in range(len(test_grid[0])):
            if test_grid[i][j] == 0:
                test_start = [i,j]
                test_solver.start = test_start
                test_solution = min(test_solution, test_solver.solve())
    print(f"test input: {test_solution}")
    assert(test_solution == 29)
    
    
    [grid, _, end] = parse_input("input.txt")
    solution = math.inf
    import copy
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            solver = Solver(copy.deepcopy(grid), [i,j], end)
            if grid[i][j] == 0:
                temp_solution = solver.solve()
                solution = min(solution, temp_solution)
    print(f"real input: {solution}")
    assert(solution == 363)

def main():
    first_task()
    second_task()


if __name__ == "__main__":
    main()