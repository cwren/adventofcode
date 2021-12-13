const tools = require('./013.js');
TEST_INPUT = [
    '6,10',
    '0,14',
    '9,10',
    '0,3',
    '10,4',
    '4,11',
    '6,0',
    '6,12',
    '4,1',
    '0,13',
    '10,12',
    '3,4',
    '3,0',
    '8,4',
    '1,10',
    '2,14',
    '8,10',
    '9,0',
    '',
    'fold along y=7',
    'fold along x=5',
];
EXPECTED_PAGE = [
    '...#..#..#.',
    '....#......',
    '...........',
    '#..........',
    '...#....#.#',
    '...........',
    '...........',
    '...........',
    '...........',
    '...........',
    '.#....#.##.',
    '....#......',
    '......#...#',
    '#..........',
    '#.#........',
];
EXPECTED_INSTRUCTIONS = [
    ['y', 7],
    ['x', 5],
];
EXPECTED_PAGE_2 =[
    '#.##..#..#.',
    '#...#......',
    '......#...#',
    '#...#......',
    '.#.#..#.###',
    '...........',
    '...........',
];
EXPECTED_DOTS_2 = 17;
EXPECTED_PAGE_3 =[
    '#####',
    '#...#',
    '#...#',
    '#...#',
    '#####',
    '.....',
    '.....',
];

test('test parser', () => {
    var manual = new tools.Manual();
    for (line of TEST_INPUT) {
        manual.parse(line);
    }
    expect(manual.format()).toEqual(EXPECTED_PAGE);
    expect(manual.folds).toEqual(EXPECTED_INSTRUCTIONS);
    expect(manual.numDots).toBe(manual.points.length);  // only true before any folds
});

test('test fold 1', () => {
    var manual = new tools.Manual();
    for (line of TEST_INPUT) {
        manual.parse(line);
    }
    manual.fold(0);
    expect(manual.format()).toEqual(EXPECTED_PAGE_2);
    expect(manual.numDots).toBe(EXPECTED_DOTS_2);
});

test('test fold 2', () => {
    var manual = new tools.Manual();
    for (line of TEST_INPUT) {
        manual.parse(line);
    }
    manual.fold(0);
    manual.fold(1);
    expect(manual.format()).toEqual(EXPECTED_PAGE_3);
});

