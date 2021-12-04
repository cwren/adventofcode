const tools = require('./004.js');
TEST_INPUT = [
    '59,91,13,82,8,32,74,96,55,51,19',
    '',
    '42 47 77 49 67',
    '64 82 32 94 78',
    '96 62 45 11 43',
    '55 92 81  6 88',
    '12 95 19 24 71'
];
EXPECTED_BALLS = [59, 91, 13, 82, 8, 32, 74, 96, 55, 51, 19];
EXPECTED_BOARD = [
    [42, 47, 77, 49, 67],
    [64, 82, 32, 94, 78],
    [96, 62, 45, 11, 43],
    [55, 92, 81,  6, 88],
    [12, 95, 19, 24, 71]
];
HORIZONTAL_WIN = [96, 62, 45, 11, 43];
HORIZONTAL_SCORE = (42 + 47 + 77 + 49 + 67 +
                    64 + 82 + 32 + 94 + 78 +
                    55 + 92 + 81 +  6 + 88 +
                    12 + 95 + 19 + 24 + 71) * 43;
VERTICAL_WIN = [77, 32, 45, 81, 19];
VERTICAL_SCORE = (42 + 47 + 49 + 67 +
                  64 + 82 + 94 + 78 +
                  96 + 62 + 11 + 43 +
                  55 + 92 +  6 + 88 +
                  12 + 95 + 24 + 71) * 19;

test('test ball reader', () => {
    balls = tools.read_balls(TEST_INPUT[0]);
    expect(balls).toEqual(EXPECTED_BALLS);
});

test('board_reader', () => {
    board = new tools.Board(5);
    expect(board.parse(TEST_INPUT[1])).toBe(false);
    expect(board.parse(TEST_INPUT[2])).toBe(false);
    expect(board.parse(TEST_INPUT[3])).toBe(false);
    expect(board.parse(TEST_INPUT[4])).toBe(false);
    expect(board.parse(TEST_INPUT[5])).toBe(false);
    expect(board.parse(TEST_INPUT[6])).toBe(true);
    expect(board.to_array()).toEqual(EXPECTED_BOARD);
});

test('horizontal is_win true', () => {
    board = new tools.Board(5);
    board.mark = [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [true,  true,  true,  true,  true ],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]
    expect(board.is_win()).toBe(true);
});

test('horizontal is_win false', () => {
    board = new tools.Board(5);
    board.mark = [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [true,  true,  true,  false,  true ],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]
    expect(board.is_win()).toBe(false);
});

test('horizontal win', () => {
    board = new tools.Board(5);
    board.from_array(EXPECTED_BOARD);
    expect(board.play(HORIZONTAL_WIN[0])).toBe(false);
    expect(board.play(HORIZONTAL_WIN[1])).toBe(false);
    expect(board.play(HORIZONTAL_WIN[2])).toBe(false);
    expect(board.play(HORIZONTAL_WIN[3])).toBe(false);
    expect(board.play(HORIZONTAL_WIN[4])).toBe(true);
    expect(board.num_plays).toBe(5);
    expect(board.score).toBe(HORIZONTAL_SCORE);
});


test('vertical is_win true', () => {
    board = new tools.Board(5);
    board.mark = [
        [false, true,  false, false, false],
        [false, true,  false, false, false],
        [false, true,  false, false, false],
        [false, true,  false, false, false],
        [false, true,  false, false, false],
    ]
    expect(board.is_win()).toBe(true);
});

test('vertical is_win false', () => {
    board = new tools.Board(5);
    board.mark = [
        [false, true,  false, false, false],
        [false, true,  false, false, false],
        [false, true,  false, false, false],
        [false, true,  false, false, false],
        [false, false, false, false, false],
    ]
    expect(board.is_win()).toBe(false);
});

test('vertical win', () => {
    board = new tools.Board(5);
    board.from_array(EXPECTED_BOARD);
    expect(board.play(VERTICAL_WIN[0])).toBe(false);
    expect(board.play(VERTICAL_WIN[1])).toBe(false);
    expect(board.play(VERTICAL_WIN[2])).toBe(false);
    expect(board.play(VERTICAL_WIN[3])).toBe(false);
    expect(board.play(VERTICAL_WIN[4])).toBe(true);
    expect(board.num_plays).toBe(5);
    expect(board.score).toBe(VERTICAL_SCORE);
});

test('keep winning but not counting', () => {
    board = new tools.Board(5);
    board.from_array(EXPECTED_BOARD);
    expect(board.play(VERTICAL_WIN[0])).toBe(false);
    expect(board.play(VERTICAL_WIN[1])).toBe(false);
    expect(board.play(VERTICAL_WIN[2])).toBe(false);
    expect(board.play(VERTICAL_WIN[3])).toBe(false);
    expect(board.play(VERTICAL_WIN[4])).toBe(true);
    // win at 5 but keep playing
    EXPECTED_BALLS.forEach(b => board.play(b));
    // should get same scpre and history
    expect(board.num_plays).toBe(5);
    expect(board.score).toBe(VERTICAL_SCORE);
});

test('no win', () => {
    board = new tools.Board(5);
    board.from_array(EXPECTED_BOARD);
    EXPECTED_BALLS.forEach(b => board.play(b));
    expect(board.num_plays).toBe(EXPECTED_BALLS.length);
    expect(board.score).toBe(0);
});
