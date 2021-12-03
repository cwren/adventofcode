#!javascript

var lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('001.txt')
});

var first = true;
var last = 0
var n = 0

lineReader.on('line', (line) => {
    d = parseInt(line);
    if(!first && d > last) {
        n += 1;
    }
    first = false;
    last = d;
    console.log(`${line}: ${d}`);
});
lineReader.on('close', () => {
    console.log(`${n}`);
});
