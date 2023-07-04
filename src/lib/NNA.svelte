<script async lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  type pointofinterest = {
    x: number;
    y: number;
  };

  var width = 500;
  var height = 500;
  var minAnzahl = 3;
  var maxAnzahl = 10;
  let fortschrit = 0;
  let fortschritTest = 0;

  var points: pointofinterest[] = [
    {
      x: getRandomInt(499, 1),
      y: getRandomInt(499, 1),
    },
  ];
  var Order: pointofinterest[] = [];
  let ctx;
  let canvas;

  onMount(() => {
    for (let i = 0; i < getRandomInt(maxAnzahl, minAnzahl); i++) {
      points.push({
        x: getRandomInt(width - 1, 1),
        y: getRandomInt(height - 1, 1),
      });
    }
    ctx = canvas.getContext("2d");
    draw();

    return stop;
  });
  function getRandomInt(max, min) {
    return Math.floor(Math.random() * max) + min;
  }

  function rest() {
    ctx.clearRect(0, 0, width, height);
    points = [];
    Order = [];
    for (let i = 0; i < getRandomInt(maxAnzahl, minAnzahl); i++) {
      points.push({
        x: getRandomInt(width - 1, 1),
        y: getRandomInt(height - 1, 1),
      });
    }
    draw();
  }
  function draw() {
    for (let i = 0; i < points.length; i++) {
      ctx.beginPath();
      ctx.ellipse(
        points[i].x,
        points[i].y,
        2,
        2,
        (2 * Math.PI * 0) / 360 / 2 + Math.PI / 2,
        0,
        2 * Math.PI
      );
      ctx.lineWidth = 1;
      ctx.strokeStyle = "#d9e62d";
      ctx.fillStyle = "#d9e62d";
      ctx.fill();
      ctx.stroke();
    }
  }
  const cheack = async () => {
    Order.push(points[0]);
    while (Order.length < points.length) {
      let newInOrder = await Rechnen();
      await Order.push(points[newInOrder]);
      await drawLine(
        Order[Order.length - 2],
        Order[Order.length - 1],
        "#3febeb"
      );
      fortschrit = (100 / points.length) * Order.length;
    }
  };
  const redraw = async () => {
    ctx.clearRect(0, 0, width, height);
    draw();
    let lastpoint;
    for (let i = 0; i < Order.length; i++) {
      if (i > 0) {
        drawLine(lastpoint, Order[i], "#3febeb");
      }
      lastpoint = Order[i];
    }
  };
  const drawLine = async (
    start: pointofinterest,
    end: pointofinterest,
    color: string
  ) => {
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.lineWidth = 1.5;
    ctx.strokeStyle = color;
    ctx.fill();
    ctx.stroke();
  };
  const Rechnen = async () => {
    fortschritTest = 0;
    let lowets = Number.MAX_VALUE;
    let lowersID = 2;
    for (let i = 0; i < points.length; i++) {
      fortschritTest = (100 / points.length) * i;
      if (!Order.includes(points[i])) {
        let nowDis = 0;
        let nowX = Order[Order.length - 1].x - points[i].x;
        let nowY = Order[Order.length - 1].y - points[i].y;
        if (nowX < -1) {
          nowX *= -1;
        } else if (nowX === 0) {
          nowDis = nowY;
        }
        if (nowY < -1) {
          nowY *= -1;
        } else if (nowY === 0) {
          nowDis = nowX;
        }
        nowDis = Math.sqrt(Math.pow(nowX, 2) + Math.pow(nowY, 2));
        await drawLine(Order[Order.length - 1], points[i], "#80000085");
        await Sleep(5);
        await redraw();
        if (
          (nowDis < lowets && !Order.includes(points[i])) ||
          (lowets == undefined && !Order.includes(points[i]))
        ) {
          lowets = nowDis;
          lowersID = i;
        }
      }
    }
    await redraw();
    return lowersID;
  };
  function Sleep(milliseconds) {
    return new Promise((resolve) => setTimeout(resolve, milliseconds));
  }
</script>

<h1>NNA</h1>
<h3>{fortschrit}%</h3>
<h5>{fortschritTest}%</h5>
<canvas {width} {height} id="Fild" bind:this={canvas} />
<div class="nice">
  <button class="Trigger" on:click={cheack}>Start</button>
  <button class="Trigger" on:click={rest}>Rest</button>
  <label for="Width">Width:</label>
  <input
    type="number"
    bind:value={width}
    name="Width"
    placeholder="with"
    min="500"
  />

  <label for="height">height:</label>
  <input
    type="number"
    name="height"
    bind:value={height}
    placeholder="height"
    min="500"
  />

  <label for="maxAnzahl">maxAnzahl:</label>
  <input
    type="number"
    name="maxAnzahl"
    bind:value={maxAnzahl}
    placeholder="maxAnzahl"
  />

  <label for="minAnzahl">minAnzahl:</label>
  <input
    type="number"
    name="minAnzahl"
    bind:value={minAnzahl}
    placeholder="minAnzahl"
  />
</div>

<style>
  canvas {
    background-color: #303030;
  }
  .nice {
    display: flex;
    flex-direction: column;
  }
</style>
