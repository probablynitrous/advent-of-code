const fs = require('fs');

try {
  const data = fs.readFileSync('input.txt', 'utf8');
  const lines = data.split('\n\n');
  const draws = lines.shift().split(',');

  const boards = lines.map((board) => {
    let rows = board
      .split('\n')
      .map((row) => row.split(' ').filter((val) => val != ''))
      .filter((row) => 0 < row.length);

    const columns = Array.from(Array(rows[0].length).keys()).map(
      (columnIndex) => {
        const column = {};
        rows.forEach((row) => {
          column[row[columnIndex]] = false;
        });
        return column;
      }
    );

    rows = rows.map((rowValues) => {
      const row = {};
      rowValues.forEach((rowValue) => {
        row[rowValue] = false;
      });
      return row;
    });

    return { rows, columns };
  });

  let winningBoard;
  let lastDraw;

  draws.forEach((draw) => {
    boards.forEach((board) => {
      board.rows.forEach((row) => {
        if (!winningBoard) {
          delete row[draw];
          if (Object.keys(row).length < 1) {
            // Bingo
            winningBoard = board;
            lastDraw = draw;
          }
        }
      });

      board.columns.forEach((column) => {
        if (!winningBoard) {
          delete column[draw];
          if (Object.keys(column).length < 1) {
            // Bingo
            winningBoard = board;
            lastDraw = draw;
          }
        }
      });
    });
  });

  // Calculate score
  console.log('Calcualting winning score');
  let result = 0;
  winningBoard.rows.forEach((row) => {
    Object.keys(row).forEach((value) => {
      result = result + parseInt(value);
    });
  });
  result = result * lastDraw;
  console.log('Result is', result);
} catch (err) {
  console.error(err);
}
