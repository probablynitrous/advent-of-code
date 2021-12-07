const fs = require('fs');

try {
  const data = fs.readFileSync('test.txt', 'utf8');
  const lines = data.split('\n');
  // Remove emtpty line
  lines.pop();

  const re =
    /^(?<startX>[0-9]+),(?<startY>[0-9]+) -> (?<endX>[0-9]+),(?<endY>[0-9]+)/;
  let vents = lines
    .map((line) => line.match(re).groups)
    .filter((vent) => vent.startX === vent.endX || vent.startY === vent.endY)
    .map((vent) => {
      if (vent.endX < vent.startX) {
        let temp;
        //Swap
        temp = vent.endX;
        vent.endX = vent.startX;
        vent.startX = temp;
      }
      if (vent.endY < vent.startY) {
        let temp;
        //Swap
        temp = vent.endY;
        vent.endY = vent.startY;
        vent.startY = temp;
      }
      return vent;
    })
    .map((vent) => {
      Object.keys(vent).forEach((key) => {
        vent[key] = parseInt(vent[key]);
      });
      return vent;
    });

  // console.log(JSON.stringify(vents));
  const ventSpots = {};

  vents.forEach((vent) => {
    for (let y = vent.startY; y <= vent.endY; y++) {
      ventSpots[y] = ventSpots[y] || {};
      for (let x = vent.startX; x <= vent.endX; x++) {
        ventSpots[y][x] = ventSpots[y][x] + 1 || 1;
      }
    }
  });

  console.log(ventSpots);

  let result = 0;
  Object.keys(ventSpots).forEach((y) => {
    Object.keys(ventSpots[y]).forEach((x) => {
      if (1 < ventSpots[y][x]) {
        result = result + 1;
      }
    });
  });
  console.log('Result is', result);
  // 5708 too low
} catch (err) {
  console.error(err);
}
