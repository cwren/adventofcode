#!javascript
const { PriorityQueue } = require('@datastructures-js/priority-queue');

var tools = {
    Cave: class {
        constructor () {
            this.r = [];
            this.from = [];
            this.c = [];
            this.w = 0;
            this.h = 0;
            this.w0 = 0;
            this.h0 = 0;
            this.queue = [];
            this.plex = 1;
        }

        setMultiplier (m) {
            this.plex = m;
            this.queue = [];
            this.w = m * this.w0;
            this.h = m * this.h0;
            for (let j = 0; j < this.h; j++) {
                this.from.push(new Array(this.w));
                this.c.push(new Array(this.w).fill(Number.MAX_VALUE));
            }
        }

        parse (s) {
            let row = s.split('').map(c => parseInt(c));;
            this.r.push(row);
            this.h0 = this.r.length;
            this.w0 = row.length;
        }

        at (a, b) {
            return a.every((v, i) => v === b[i]);
        }

        enqueue (p) {
            let found = false;
            for (let q of this.queue) {
                found |= this.at(p, q);
            }
            if (!found) {
                this.queue.push(p);
            }
        }

        setFrom (a, b) {
            this.from[a[1]][a[0]] = b;
        }
        
        getFrom (a) {
            return this.from[a[1]][a[0]];
        }
        
        setCost (p, c) {
            this.c[p[1]][p[0]] = c;
        }
        
        getCost (p) {
            return this.c[p[1]][p[0]];
        }
        
        getRisk (p) {
            let i = p[0];
            let j = p[1];
            let i0 = i % this.w0;
            let j0 = j % this.h0;
            let r0 = this.r[j0][i0];
            let n = Math.floor(i / this.w0) + Math.floor(j / this.h0);
            return ((r0 - 1 + n) % 9) + 1;
        }
        
        h (p) {
            return (this.w - p[0]) + (this.h - p[1]);
        }

        neighbors (p) {
            return {
                w: this.w,
                h: this.h,
                *[Symbol.iterator]() {
                    let i = p[0];
                    let j = p[1];
                    for (let di = -1; di < 2; di++) {
                        for (let dj = - 1; dj < 2; dj++) {
                            if ((di != 0 || dj != 0) &&
                                (di == 0 || dj == 0) &&
                                ((i + di) >= 0) &&
                                ((j + dj) >= 0) &&
                                ((i + di) < this.w) &&
                                ((j + dj) < this.h)) {
                                yield [i + di, j + dj];
                            }
                        }
                    }
                }
            }
        }
        
        backprop () {
            let start = [0, 0];
            let p = [this.w - 1, this.h - 1];
            let r = 0;
            while (!this.at(p,start)) {
                //console.log(`${p[0]} ${p[1]}`);
                r += this.getRisk(p);
                p = this.getFrom(p);
            }
            return r;
        }
        
        get risk () {
            let goal = [this.w - 1, this.h - 1];

            this.queue = [];
            this.setCost([0, 0], 0);
            this.enqueue([0, 0]);
            let done = false;
            while(!done) {
                this.queue.sort((a, b) => this.getCost(a) - this.getCost(b));
                let p = this.queue.shift();
                done = this.at(p, goal);

                for (let n of this.neighbors(p)) {
                    let cost = this.getCost(p) + this.getRisk(n);
                    if (cost < this.getCost(n)) {
                        this.setCost(n, cost);
                        this.setFrom(n, p);
                        this.enqueue(n);
                    }
                }
            }
            return this.backprop();
        }
    },
};    
module.exports = tools;

function main() {
    let lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('015.txt')
    });
    
    var cave = new tools.Cave();
    lineReader.on('line', (line) => {
        cave.parse(line);
    });
    
    lineReader.on('close', () => {
        // cave.setMultiplier(1);
        // console.log(`1: ${cave.risk}`);
        cave.setMultiplier(5);
        console.log(`5: ${cave.risk}`);
    });
};

if (require.main === module) {
  main();
}
