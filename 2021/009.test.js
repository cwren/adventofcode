const tools = require('./009.js');
TEST_INPUT = ['2199943210',
              '3987894921',
              '9856789892',
              '8767896789',
              '9899965678',
              ];
EXPECTED_HEIGHTS = [[2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                    [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                    [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                    [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                    [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
                   ];
EXPECTED_LOWS = [0, 1, 5, 5];
EXPECTED_THREAT = 15;
EXPECTED_BASINS = [ 14, 3, 9, 9];

test('test parser', () => {
    height_map = TEST_INPUT.map(l => tools.parseLine(l));
    expect(height_map).toEqual(EXPECTED_HEIGHTS);
});

test('test lows', () => {
    expect(tools.find_lows(EXPECTED_HEIGHTS).sort()).toEqual(EXPECTED_LOWS);
});

test('test threat', () => {
    expect(tools.compute_threat(EXPECTED_LOWS)).toBe(EXPECTED_THREAT);
});

test('test basins', () => {
    expect(tools.find_basins(EXPECTED_HEIGHTS).sort()).toEqual(EXPECTED_BASINS);
});

test('test basin score', () => {
    expect(tools.compute_score(EXPECTED_BASINS)).toBe(1134);
});

