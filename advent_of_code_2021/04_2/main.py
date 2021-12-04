#!/usr/bin/env python

class Board:
    def __init__(self):
        # map from number to its place in the board: `(line, column)`
        self.num_pos = dict()
        self.used_lines = [0, 0, 0, 0, 0]
        self.used_columns = [0, 0, 0, 0, 0]
        self.total_unmarked = 0
        self.win = False

    # Returns true if "bingo".
    def mark(self, number):
        if number not in self.num_pos:
            return
        (line, col) = self.num_pos[number]
        self.used_lines[line] += 1
        self.used_columns[col] += 1
        self.total_unmarked -= number

        return self.used_lines[line] == 5 or self.used_columns[col] == 5

def read_board():
    board = Board()

    for i in range(0, 5):
        line = [ int(n) for n in input().split(" ") if n != "" ]
        for j in range(0, 5):
            board.num_pos[line[j]] = (i, j)
            board.total_unmarked += line[j]

    return board

def bingo(board, number):
    print(number * board.total_unmarked)

def play():
    for n in numbers:
        for board in boards:
            if not board.win and board.mark(n):
                board.win = True
                bingo(board, n)

boards = []

numbers = list(map(lambda n: int(n), input().split(",")))

while True:
    try:
        input()
    except:
        break
    boards.append(read_board())

play()
