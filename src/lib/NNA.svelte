<script async lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  type pointofinterest = {
    x: number;
    y: number;
  };

  var width = 800;
  var height = 600;
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
    for (let i = 0; i < getRandomInt(10, 3); i++) {
      points.push({ x: getRandomInt(499, 1), y: getRandomInt(499, 1) });
    }
    ctx = canvas.getContext("2d");
    draw();

    return stop;
  });
  function getRandomInt(max, min) {
    return Math.floor(Math.random() * max) + min;
  }
  appWindow.onResized(async ({ payload: size }) => {
    width = size.width;
    height = size.height;
  });

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
      console.table(Order);
      await drawLine(
        Order[Order.length - 2],
        Order[Order.length - 1],
        "#3febeb"
      );
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
    ctx.lineWidth = 2;
    ctx.strokeStyle = color;
    ctx.fill();
    ctx.stroke();
  };
  const Rechnen = async () => {
    let lowets = undefined;
    for (let i = 0; i < points.length; i++) {
      let nowX = points[i].x;
      let nowY = points[i].y;
      if (!Order.includes(points[i])) {
        let nowDis = Math.sqrt(Math.pow(nowX, 2) + Math.pow(nowY, 2));
        await drawLine(Order[Order.length - 1], points[i], "#9c0000");
        await Sleep(500);
        await drawLine(Order[Order.length - 1], points[i], "#303030");
        console.log(nowDis);
        if (
          (nowDis < lowets && !Order.includes(points[i])) ||
          (lowets == undefined && !Order.includes(points[i]))
        ) {
          lowets = i;
        }
      }
    }
    return lowets;
  };
  function Sleep(milliseconds) {
    return new Promise((resolve) => setTimeout(resolve, milliseconds));
  }
</script>

<h1>NNA</h1>
<canvas width="500px" height="500px" id="Fild" bind:this={canvas} />
<button class="Trigger" on:click={cheack}>Start</button>

<style>
  canvas {
    background-color: #303030;
  }
</style>
