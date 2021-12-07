const fs = require('fs');

try {
  const data = fs.readFileSync('input.txt', 'utf8');
  const lines = data.split('\n');
  // Remove emtpty line
  lines.pop();

  const re =
    /^(?<startX>[0-9]+),(?<startY>[0-9]+) -> (?<endX>[0-9]+),(?<endY>[0-9]+)/;
  let vents = lines
    .map((line) => line.match(re).groups)
    // .filter((vent) => vent.startX === vent.endX || vent.startY === vent.endY)
    .map((vent) => {
      Object.keys(vent).forEach((key) => {
        vent[key] = parseInt(vent[key]);
      });
      return vent;
    })
    // .map((vent) => {
    //   // Make sure all lines have a positive X component
    //   if (vent.endX < vent.startX) {
    //     let temp;
    //     //Swap
    //     temp = vent.endX;
    //     vent.endX = vent.startX;
    //     vent.startX = temp;

    //     temp = vent.endY;
    //     vent.endY = vent.startY;
    //     vent.startY = temp;
    //   }
    //   return vent;
    // })
    .map((vent) => {
      // Get gradients
      vent.deltaX = vent.endX - vent.startX;
      if (vent.deltaX < 0) {
        vent.gradX = -1;
      } else if (0 < vent.deltaX) {
        vent.gradX = 1;
      } else {
        vent.gradX = 0;
      }

      vent.deltaY = vent.endY - vent.startY;
      if (vent.deltaY < 0) {
        vent.gradY = -1;
      } else if (0 < vent.deltaY) {
        vent.gradY = 1;
      } else {
        vent.gradY = 0;
      }

      return vent;
    });

  const ventSpots = {};

  vents.forEach((vent) => {
    const length = [Math.abs(vent.deltaX), Math.abs(vent.deltaY)].sort().pop();
    vent.length = length;
    for (let i = 0; i <= length; i++) {
      ventSpots[vent.startY + i * vent.gradY] =
        ventSpots[vent.startY + i * vent.gradY] || {};

      ventSpots[vent.startY + i * vent.gradY][vent.startX + i * vent.gradX] =
        ventSpots[vent.startY + i * vent.gradY][vent.startX + i * vent.gradX] +
          1 || 1;
    }
  });

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
