#!javascript

var tools = {
    Sequence: class {
        constructor () {
            this.seq0 = '';
            this.last = '';;
            this.bases = new Map();
            this.pairs = new Map();
            this.rules = new Map();
        }

        plus (map, k, v) {
            map.set(k, map.get(k) + v);
        }
        
        logMap (map) {
            console.log(JSON.stringify(Array.from(map.entries()), null, 4))
        }
        
        parse (s) {
            if (this.seq0.length == 0) {
                this.seq0 = s;
                return;
            }
            if (s.length == 0) {
                return;
            }
            let [input, output] = s.split(' -> ');
            this.rules.set(input, output);
            this.pairs.set(input, 0);
            let [c, d] = input.split('');
            this.bases.set(c, 0);
            this.bases.set(d, 0);
        }

        init () {
            this.last = this.seq0[this.seq0.length - 1]
            this.fromString(this.seq0);
        }
        
        fromString(s) {
            var seq = s.split('');
            for (let i = 0; i < seq.length; i++) {
                let c = seq[i];
                this.plus(this.bases, c, 1);
                if ((i + 1) < seq.length) {
                    let d = seq[i + 1];
                    let p = [c, d].join('');
                    this.plus(this.pairs, p, 1);
                }
            }
        }

        grow () {
            var nextBases = new Map();
            var nextPairs = new Map();
            for (let [k, v] of this.bases) {
                nextBases.set(k, 0);
            }
            for (let [k, v] of this.pairs) {
                nextPairs.set(k, 0);
            }
            for (let [p, n] of this.pairs) {
                if (n > 0) {
                    let [c, d] = p.split('');
                    let o = this.rules.get(p);
                    this.plus(nextBases, c, n);
                    this.plus(nextBases, o, n);
                    this.plus(nextPairs, [c, o].join(''), n);
                    this.plus(nextPairs, [o, d].join(''), n);
                }
            }
            this.plus(nextBases, this.last, 1);
            this.bases = nextBases;
            this.pairs = nextPairs;
        }

        get length () {
            let total = 0;
            for (let v of this.bases.values()) {
                total += v;
            }
            return total;
        }
        
        get score () {
            var max = 0;
            var min = Number.MAX_VALUE;
            for (let v of this.bases.values()) {
                max = Math.max(max, v);
                min = Math.min(min, v);
            }
            return max - min
        }

        format () {
            return this.page.map(row => row.map(c => c ? '#' : '.').join(''));
        }

        get numDots () {
            return this.page.map(row => row.map(c => c ? 1 : 0)
                                 .reduce((a, v) => a + v, 0))
                .reduce((a, v) => a + v, 0);
        }
    },
};    
module.exports = tools;

function main() {
    let lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('014.txt')
    });
    
    var seq = new tools.Sequence();
    lineReader.on('line', (line) => {
        seq.parse(line);
    });
    
    lineReader.on('close', () => {
        seq.init();
        for (var t = 0; t < 10; t++) {
            seq.grow();
        }
        console.log(`10: ${seq.score}`);
        for (var t = 0; t < 30; t++) {
            seq.grow();
        }
        console.log(`40: ${seq.score}`);
    });
};

if (require.main === module) {
  main();
}
