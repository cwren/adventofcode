const tools = require('./008.js');
TEST_INPUT = 'be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe';
EXPECTED_PATTERNS = ['be', 'cfbegad', 'cbdgef', 'fgaecd', 'cgeb', 'fdcge', 'agebfd', 'fecdb', 'fabcd', 'edb'];
EXPECTED_OUTPUTS = ['fdgacbe', 'cefdb', 'cefbgd', 'gcbe'];

test('test parser', () => {
    [p, o] = tools.parseLine(TEST_INPUT);
    expect(p).toEqual(EXPECTED_PATTERNS);
    expect(o).toEqual(EXPECTED_OUTPUTS);
});

test('test ones', () => {
    expect(tools.ones(EXPECTED_OUTPUTS).length).toBe(0);
    expect(tools.ones(EXPECTED_PATTERNS).length).toBe(1);
});

test('test fours', () => {
    expect(tools.fours(EXPECTED_OUTPUTS).length).toBe(1);
    expect(tools.fours(EXPECTED_PATTERNS).length).toBe(1);
});

test('test sevens', () => {
    expect(tools.sevens(EXPECTED_OUTPUTS).length).toBe(0);
    expect(tools.sevens(EXPECTED_PATTERNS).length).toBe(1);
});

test('test eights', () => {
    expect(tools.eights(EXPECTED_OUTPUTS).length).toBe(1);
    expect(tools.eights(EXPECTED_PATTERNS).length).toBe(1);
});

test('test norm', () => {
    expect(tools.norm('')).toBe('');
    expect(tools.norm('a')).toBe('a');
    expect(tools.norm('abc')).toBe('abc');
    expect(tools.norm('cba')).toBe('abc');
});

test('test subtract', () => {
    expect(tools.subtract('', '')).toBe('');
    expect(tools.subtract('a', 'b')).toBe('a');
    expect(tools.subtract('a', 'a')).toBe('');
    expect(tools.subtract('abc', 'ac')).toBe('b');
});
    
INPUT = [
    'be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe',
    'edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc',
    'fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg',
    'fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb',
    'aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea',
    'fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb',
    'dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe',
    'bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef',
    'egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb',
    'gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce',
];
OUTPUT = [
    8394,
    9781,
    1197,
    9361,
    4873,
    8418,
    4548,
    1625,
    8717,
    4315,
];

test('test decoder', () => {
    INPUT.forEach((v, i) => {
        expect(tools.decode(v)).toBe(OUTPUT[i]);
    });
});
