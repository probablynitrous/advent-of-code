let fs = require('fs'),
  lines = Array.from(fs.readFileSync('input.txt', 'utf8').split('\n'));

console.log(
  'Result',
  [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2],
  ]
    .map((increment) =>
      (function countHits(increment, x = 0, y = 0, count = 0) {
        lines[y].length <= x && (lines = lines.map((line) => line + line));
        lines[y].charAt(x) === '#' && count++;
        return lines.length <= y + 1 + increment[1]
          ? count
          : countHits(increment, x + increment[0], y + increment[1], count);
      })(increment)
    )
    .reduce((a, b) => a * b, 1)
);
