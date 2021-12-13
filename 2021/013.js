#!javascript
const assert = require('assert');

var tools = {
    Manual: class {
        constructor () {
            this.w = 0;
            this.h = 0;
            this.points = [];
            this.folds = [];
            this.pageLoaded = false;
        }

        parse (s) {
            if (!this.pageLoaded) {
                if (s == '') {
                    this.pageLoaded = true;
                    for (let [x, y] of this.points) {
                        if ((x + 1) > this.w) {
                            this.w = x + 1;
                        }
                        if ((y + 1) > this.h) {
                            this.h = y + 1;
                        }
                    }
                    return;
                }
                let [x, y] = s.split(',').map(n => parseInt(n));
                this.points.push([x, y]);
            } else {
                let [d, c] = s.split(' ')[2].split('=');
                this.folds.push([d, parseInt(c)]);
            }
        }

        fold (i) {
            if (i >= this.folds.length) {
                return false;
            }
            let [d, c] = this.folds[i];
            for (let i = 0; i < this.points.length; i++) {
                let [x, y] = this.points[i];
                if (d == 'y' && y > c) {
                    y = 2 * c - y;
                }
                if (d == 'x' && x > c) {
                    x = 2 * c - x;
                }
                this.points[i] = [x, y]
            }
            if (d == 'y') {
                this.h = c;
            }
            if (d == 'x') {
                this.w = c;
            }
            return true;
        }

        get page () {
            let page = [];
            for (let j = 0; j < this.h; j++) {
                page.push(new Array(this.w).fill(false));
            }
            for (let [x, y] of this.points) {
                page[y][x] = true;
            }
            return page;
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
        input: require('fs').createReadStream('013.txt')
    });
    
    let manual = new tools.Manual();
    lineReader.on('line', (line) => {
        manual.parse(line);
    });
    
    lineReader.on('close', () => {
        manual.fold(0);
        console.log(`a: ${manual.numDots}`);;

        for (let i = 1; i < manual.folds.length; i++) {
            manual.fold(i);
        }
        console.log(`b:`);;
        for (let row of manual.format()) {
            console.log(`${row}`);
        }
    });
};

if (require.main === module) {
  main();
}
