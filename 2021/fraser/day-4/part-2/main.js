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

  let losingBoard;
  let lastDraw;
  let winners = {};
  draws.forEach((draw) => {
    let boardIndex = 0;

    boards.forEach((board) => {
      board.rows.forEach((row) => {
        if (!losingBoard) {
          delete row[draw];
          if (Object.keys(row).length < 1) {
            winners[boardIndex] = true;
            if (Object.keys(winners).length === boards.length) {
              // We were the last winner
              losingBoard = board;
              lastDraw = draw;
            }
          }
        }
      });

      board.columns.forEach((column) => {
        if (!losingBoard) {
          delete column[draw];
          if (Object.keys(column).length < 1) {
            winners[boardIndex] = true;
            if (Object.keys(winners).length === boards.length) {
              // We were the last winner
              losingBoard = board;
              lastDraw = draw;
            }
          }
        }
      });

      boardIndex = boardIndex + 1;
    });
  });

  // Calculate score
  console.log('Calcualting losing score');
  let result = 0;
  losingBoard.rows.forEach((row) => {
    Object.keys(row).forEach((value) => {
      result = result + parseInt(value);
    });
  });
  result = result * lastDraw;
  console.log('Result is', result);
} catch (err) {
  console.error(err);
}
