import "./style.css";
// import the init function, Universe class and the Cell enum from the wasm module
// wasm module は backend/pkg ディレクトリー内に生成されている (`wasm-pack build --target web`)
import init, { Cell, Universe } from "../pkg/backend";

// run() 関数は、wasm module を初期化する
async function run() {
  console.log("Application started: run");
  // 1. initialize the wasm module
  const wasm = await init(); // 非同期初期化
  const memory = wasm.memory;

  // 2. create the universe
  const width = 64;
  const height = 64;
  const universe = Universe.new(width, height);

  const CELL_SIZE = 5; // unit is px

  // 3. setup the canvas
  const canvas = document.getElementById(
    "game-of-life-canvas",
  ) as HTMLCanvasElement;
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext("2d");
  if (!ctx) {
    // 取得失敗
    return;
  }

  // 4. render the universe
  const renderLoop = () => {
    // 4.1 update the universe (rust wasm calculates the next state)
    universe.tick();
    // 4.2 draw the cells
    drawCells();

    //4.3 request the next animation frame
    requestAnimationFrame(renderLoop); // 再帰的に本関数を呼び出す
  };

  const drawCells = () => {
    // 4.2.1 Cell 構造体の配列を取得する(Rust の Linear Memoryから取得)
    const cellPtr = universe.cells();

    // 4.2.2 Uint8Array を作成する
    // Zero Copy を使用することで、Rust の Linear Memory から直接配列を取得する
    // Uint8Arrayは TypeScriptの 型付配列(TypedArray)の一種であり，8bit unsigned integer (0-255)を格納する配列である
    // 特に wasm + javascript でバイナリデータのやり取りを行う際には必須で，zero copyを使用することで
    // パフォーマンスを犠牲にせずにバイナリデータのやり取りを行うことができる
    // Uint8Array は固定長配列でpush/pooなどのように要素数の追加や削除は不可
    // 通常の JavaScript の Arrayよりもメモリ効率が良く画像処理やwasm, 音声処理などで使用される
    // 生のバイナリデータを格納する ArrayBuffer は直接操作できないが，
    // Unint8Array は直接操作が可能である。ArrayBuffer で確保したメモリの操作用の view 層としてUint8Arrayを使用可能
    const cells = new Uint8Array(memory.buffer, cellPtr, width * height);

    // 4.2.3 描画を開始(path)
    ctx.beginPath();

    // 4.2.4 grid を描画
    ctx.fillStyle = "#CCCCCC";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const index = row * width + col;

        // cell が Alive/Dead を欠くにmんして描画する
        ctx.fillStyle = cells[index] === Cell.Alive ? "#000000" : "#FFFFFF";
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE,
        );
      }
    }
    ctx.stroke();
  };

  renderLoop();
}

run();
