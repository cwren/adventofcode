const tools = require('./010.js');
TEST_INPUT = [
    '[({(<(())[]>[[{[]{<()<>>',
    '[(()[<>])]({[<{<<[]>>(',
    '{([(<{}[<>[]}>{[]{[(<()>',
    '(((({<>}<{<{<>}{[]{[]{}',
    '[[<[([]))<([[{}[[()]]]',
    '[{[{({}]{}}([{[{{{}}([]',
    '{<[[]]>}<{[{[{[]{()[[[]',
    '[<(<(<(<{}))><([]([]()',
    '<{([([[(<>()){}]>(<<{{',
    '<{([{{}}[<[[[<>{}]]]>[]]',
];
CORRUPTIONS = new Map([
    ['[({(<(())[]>[[{[]{<()<>>', undefined],
    ['[(()[<>])]({[<{<<[]>>(',   undefined],
    ['{([(<{}[<>[]}>{[]{[(<()>', '}'],
    ['(((({<>}<{<{<>}{[]{[]{}',  undefined],
    ['[[<[([]))<([[{}[[()]]]' ,  ')'],
    ['[{[{({}]{}}([{[{{{}}([]',  ']'],
    ['{<[[]]>}<{[{[{[]{()[[[]',  undefined],
    ['[<(<(<(<{}))><([]([]()',   ')'],
    ['<{([([[(<>()){}]>(<<{{',   '>'],
    ['<{([{{}}[<[[[<>{}]]]>[]]', undefined],
]);
EXPECTED_SCORE = 26397;

test('test parser', () => {
    for (line of CORRUPTIONS.keys()) {
        expect(tools.parse(line)).toBe(CORRUPTIONS.get(line));
    }
});

test('test syntax score', () => {
    expect(tools.compute_syntax(Array.from(CORRUPTIONS.values()))).toBe(EXPECTED_SCORE);
});

COMPLETIONS = new Map([
    ['[({(<(())[]>[[{[]{<()<>>', '}}]])})]'],
    ['[(()[<>])]({[<{<<[]>>(',   ')}>]})'],
    ['{([(<{}[<>[]}>{[]{[(<()>', undefined],
    ['(((({<>}<{<{<>}{[]{[]{}',  '}}>}>))))'],
    ['[[<[([]))<([[{}[[()]]]' ,  undefined],
    ['[{[{({}]{}}([{[{{{}}([]',  undefined],
    ['{<[[]]>}<{[{[{[]{()[[[]',  ']]}}]}]}>'],
    ['[<(<(<(<{}))><([]([]()',   undefined],
    ['<{([([[(<>()){}]>(<<{{',   undefined],
    ['<{([{{}}[<[[[<>{}]]]>[]]', '])}>'],
]);
COMPLETION_SCORES = new Map([
    ['}}]])})]',  288957],
    [')}>]})',    5566],
    ['}}>}>))))', 1480781],
    [']]}}]}]}>', 995444],
    ['])}>',      294],
]);

test('test completion score', () => {
    for (suffix of COMPLETION_SCORES.keys()) {
        expect(tools.compute_completion(suffix)).toBe(COMPLETION_SCORES.get(suffix));
    }
});

test('test median', () => {
    expect(tools.median(Array.from(COMPLETION_SCORES.values()))).toBe(288957);
    }
);

test('test complete', () => {
    for (line of COMPLETIONS.keys()) {
        expect(tools.complete(line)).toBe(COMPLETIONS.get(line));
    }
});


