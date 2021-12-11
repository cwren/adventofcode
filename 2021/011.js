#!javascript

var tools = {
    Octopus: class {
        constructor (i, j) {
            this.energy = 0;
            this.flashed = false;
            this.i = i;
            this.j = j;
        };
    },
        
    Cave: class {
        constructor (size = 10) {
            this.size = size;
            this.data = new Array();
            this.numFlashes = 0;
            this.t = 0;
            this.synchrony = -1;
            this.resetOctopii();
            this.input = '';
        }

        resetOctopii () {
            this.numFlashes = 0;
            this.t = 0;
            this.synchrony = -1;
            for (var i = 0; i < this.size; i++) {
                this.data.push(new Array(this.size))
                for (var j = 0; j < this.size; j++) {
                    this.data[i][j] = new tools.Octopus(i, j);
                }
            }
        }
        
        toString () {
            return this.data.map(row => row.map(o => `${o.energy}`).join('')).join('');
        }
        
        fromString (s) {
            this.resetOctopii();
            var energies = s.split('').map(n => parseInt(n));
            this.data.forEach(row => {
                row.forEach(o => {
                    o.energy = energies.shift();
                })
            });
        }
        
        parse (s) {
            s = s.trim();
            this.size = s.length;
            this.input += s;
            if (this.input.length == (this.size * this.size)) {
                this.fromString(this.input);
                this.input = '';
            }
        }

        flash (octopus) {
            let flashers = [];
            let i = octopus.i;
            let j = octopus.j;
            let s = this.size;
            for (let di = -1; di < 2; di ++) {
                for (let dj = -1; dj < 2; dj ++) {
                    if (di != 0 || dj != 0) {
                        let k = (i + di);
                        let l = (j + dj);
                        if (k > -1 && k < s && l > -1 && l < s) {
                            let neighbor = this.data[k][l];
                            neighbor.energy += 1;
                            if (neighbor.energy > 9 && !neighbor.flashed) {
                                neighbor.flashed = true;
                                flashers.push(neighbor);
                            }
                        }
                    }
                }
            }
            return flashers;
        }
        
        tick () {
            this.t ++;
            this.data.forEach(row => row.forEach(o => o.energy += 1));

            let flashers = [];
            this.data.forEach(row => row.forEach(o => {
                if (o.energy > 9) {
                    o.flashed = true;
                    flashers.push(o);
                }
            }));
            while (flashers.length > 0) {
                let flasher = flashers.shift();
                flashers.push(...this.flash(flasher));
            }

            let numFlashesNow = 0;
            this.data.forEach(row => row.forEach(o => {
                if (o.flashed) {
                    numFlashesNow ++;
                    o.flashed = false;
                    o.energy = 0;
                }
            }));
            this.numFlashes += numFlashesNow;

            if (numFlashesNow == (this.size * this.size) && this.synchrony == -1) {
                this.synchrony = this.t;
            }
        }
    },
};    
module.exports = tools;

function main() {
    let lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('011.txt')
    });
    
    let cave = new tools.Cave();
    lineReader.on('line', (line) => {
        cave.parse(line);
    });
    
    lineReader.on('close', () => {
        for (let t = 0; t < 100; t++) {
            cave.tick();
        }
        console.log(`tick ${cave.t}: ${cave.numFlashes}, ${cave.toString()}`);
        while (cave.synchrony == -1) {
            cave.tick();
        }
        console.log(`synchrony at ${cave.t}`);
    });
};

if (require.main === module) {
  main();
}
