#!javascript
const assert = require('assert');

var tools = {
    Target: class {
        constructor(x0, x1, y0, y1) {
            this.x0 = x0;
            this.x1 = x1;
            this.y0 = y0;
            this.y1 = y1;
        }
        
        static fromString (s) {
            let re = /target area: x=([-\d]+)\.\.([-\d]+), y=([-\d]+)\.\.([-\d]+)/;
            let m = s.match(re); // target area: x=155..182, y=-117..-67
            m = m.slice(1,5).map(n => parseInt(n));
            return new this(...m);
        }

        hit (x, y) {
            return (x >= this.x0) && (x <= this.x1) && (y >= this.y0) && (y <= this.y1);
        }
        
        simulate (dx, dy) {
            let track = [[0, 0]];
            let x = 0;
            let y = 0;
            while (!this.hit(x,y) && x < this.x1 && y > this.y0) {
                x += dx;
                y += dy;
                dx -= Math.sign(dx);
                dy -= 1;
                track.push([x, y]);
            }
            return [this.hit(x,y), track];
        }
        
        numHits () {
            var total = 0;
            for (let dx = 0; dx < 1000; dx += 1) {
                for (let dy = -200; dy < 1000; dy += 1) {
                    let [hit, track] = this.simulate(dx, dy);
                    if (hit) {
                        total ++;
                    }
                }
            }
            return total;
        }
        
        highest () {
            var highest = -Infinity;
            for (let dx = 0; dx < 1000; dx += 1) {
                for (let dy = 0; dy < 1000; dy += 1) {
                    let [hit, track] = this.simulate(dx, dy);
                    if (hit) {
                        highest = track.reduce((a, p) => Math.max(p[1], a), highest);
                    }
                }
            }
            return highest;
        }
    },
};    
module.exports = tools;

function main() {
    let lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('017.txt')
    });
    
    var target;
    lineReader.on('line', (line) => {
        target = tools.Target.fromString(line);
    });
    
    lineReader.on('close', () => {
        console.log(` eval: ${target.highest()}`);
        console.log(` eval: ${target.numHits()}`);
    });
};

if (require.main === module) {
  main();
}
