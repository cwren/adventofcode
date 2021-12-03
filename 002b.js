#!javascript

var lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('002.txt')
});

var a = 0;
var x = 0;
var y = 0;

lineReader.on('line', (line) => {
    [o, n] = line.split(" ", 2);
    n = parseInt(n)
    switch (o) {
    case 'forward':
        x += n;
        y += a * n;
        break;
    case 'up':
        a -= n;
        break;
    case 'down':
        a += n;
        break;
    }
});
lineReader.on('close', () => {
    console.log(`${x} * ${y} = ${x*y}`);
});
