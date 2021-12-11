#!javascript

var BRACES = new Map([
    ['<', '>'],
    ['(', ')'],
    ['{', '}'],
    ['[', ']'],
]);
var OPEN = ['<', '(', '{', '['];
var CLOSE = ['>', ')', '}', ']'];

var tools = {
    parse: function (line) {
        return tools.do_parse(line.split(''))['error'];
    },

    complete: function (line) {
        let ret = tools.do_parse(line.split(''));
        if (ret['error']) {
            return undefined;
        }
        return ret['left'].join('');
    },

    do_parse: function (symbols) {
        // empty, done
        if (symbols.length == 0) {
            return {'close': undefined, 'error': undefined, 'left': []};
        }
        // check head
        let s = symbols.shift();
        if (CLOSE.includes(s))
        {
            // should never see an unmatched close
            return {'close': s, 'error': undefined, 'left': symbols};
        }
        // is it incomplete?
        if (symbols.length == 0) {
            return {'close': undefined, 'error': undefined, 'left': [BRACES.get(s)]};
        }

        // recurse on the middle of the phrase
        ret = tools.do_parse(symbols)

        // child found an error, pass it up
        if (ret['error']) {
            return ret;
        }

        // child found an error, pass it up
        if (!ret['close']) {
            ret['left'].push(BRACES.get(s));
            return {'close': undefined, 'error': undefined, 'left': ret['left']};
        }

        // mismatched close for my open
        if (BRACES.get(s) != ret['close']) {
            return {'close': undefined, 'error': ret['close'], 'left': ret['left']};
        }

        // tail recurse on anything that might be left
        return tools.do_parse(ret['left']);
    },
    
    compute_syntax: function (errors) {
        return errors.map(e => {
            switch(e) {
            case ')':
                return 3;
            case ']':
                return 57;
            case '}':
                return 1197;
            case '>':
                return 25137;
            default:
                return 0;
            }
        }).reduce((a, v) => a + v, 0);
    },
    
    compute_completion: function (completion) {
        let score = 0;
        for (c of completion) {
            score *= 5;
            switch(c) {
            case ')':
                score += 1;
                break;
            case ']':
                score += 2;
                break;
            case '}':
                score += 3;
                break;
            case '>':
                score += 4;
                break;
            }
        }
        return score;
    },
    
    median: function (scores) {
        scores.sort((a,b) => b - a);
        let middle = Math.floor(scores.length / 2);
        return scores[middle]
    },
    
};    
module.exports = tools;

function main() {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('010.txt')
    });

    var lines = [];
    lineReader.on('line', (line) => {
        lines.push(line);
    });

    lineReader.on('close', () => {
        let results = [];
        lines.forEach(l => results.push(tools.parse(l)));
        let score = tools.compute_syntax(results);
        console.log(`a: ${score}`);

        let completions = [];
        lines.forEach(l => completions.push(tools.complete(l)));
        completions = completions.filter(e => !!e);
        let scores = completions.map(c => tools.compute_completion(c));
        console.log(`b: ${tools.median(scores)}`);
    });
};

if (require.main === module) {
    main();
}
