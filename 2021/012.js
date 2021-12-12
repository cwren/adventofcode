#!javascript
const assert = require('assert');

var tools = {
    Caves: class {
        constructor () {
            this.system = new Map();
        }

        static fromMap (m) {
            assert(m instanceof Map);
            let caves = new this();
            caves.system = new Map(m);
            return caves;
        }
        
        keys () {
            return this.system.keys();
        }

        get (key) {
            return this.system.get(key);
        }
        
        addLink (from, to) {
            if (!this.system.has(from)) {
                this.system.set(from, []);
            }
            this.system.get(from).push(to)
        }

        parse (s) {
            let [from, to] = s.split('-');
            this.addLink(from, to);
            this.addLink(to, from);
        }

        isSmall (name) {
            return /^[a-z]*$/.test(name);
        }

        crawlAllPaths (at, haveMultied, prefix, alreadyVisited, paths) {
            if (this.isSmall(at) && alreadyVisited.has(at)) {
                if (at == 'start' || at == 'end' || haveMultied) {
                    return;
                }
                haveMultied = true;
            }
            let path = [...prefix];
            path.push(at);
            let visited = new Set(...[alreadyVisited]);
            visited.add(at);

            if (at == 'end') {
                paths.push(path.join(','));
                return
            }

            for(let next of this.system.get(at)) {
                this.crawlAllPaths(next, haveMultied, path, visited, paths);
            }
        }

        get numPaths () {
            let paths = [];
            this.crawlAllPaths('start', true, [], new Set(), paths);
            return paths.length;
        }
        
        get multiPaths () {
            let paths = [];
            this.crawlAllPaths('start', false, [], new Set(), paths);
            return paths;
        }
    },
};    
module.exports = tools;

function main() {
    let lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('012.txt')
    });
    
    let caves = new tools.Caves();
    lineReader.on('line', (line) => {
        caves.parse(line);
    });
    
    lineReader.on('close', () => {
        console.log(`a: ${caves.numPaths}`);
        console.log(`b: ${caves.multiPaths.length}`);
    });
};

if (require.main === module) {
  main();
}
