const Photon = require("../../lib/index");
const $ = require("jquery");

// First initialize the event callbacks
let MAX_INCLINE = Math.PI / 2 - 0.01, MIN_INCLINE = -MAX_INCLINE;
let mouseDown = false;
let currX = 0.0, currY = 0.0;
let azimuth = 0.0, incline = 0.0;
let stream = undefined;

const $canvas = $("#main-canvas");

// Then enter render loop
const canvas = document.getElementById("main-canvas");
const { width, height } = canvas;
const context = canvas.getContext("2d");
const imgData = context.createImageData(width, height);

$("#stop-button").click(() => {
  if (stream) {
    stream.close();
    stream = undefined;
  }
})

$canvas.mousedown((event) => {
  mouseDown = true;
  currX = event.screenX;
  currY = event.screenY;
});

$canvas.mousemove((event) => {
  if (mouseDown) {
    const nextX = event.screenX;
    const nextY = event.screenY;

    const diffX = nextX - currX;
    const diffY = nextY - currY;

    // Calculate new azimuth and incline
    Photon.mainCamera.azimuth += diffX * 0.01;
    Photon.mainCamera.incline = Math.max(Math.min(incline + diffY * 0.01, MAX_INCLINE), MIN_INCLINE);

    currX = nextX;
    currY = nextY;

    // if (stream) {
    //   stream.close();
    //   stream = undefined;
    // }
    // startRenderStream();
  }
});

$canvas.mouseup(() => {
  mouseDown = false;
});

function startRenderStream() {
  stream = Photon.createRenderStream(imgData, (event) => {
    if (event.type === "update") {
      context.putImageData(imgData, 0, 0);
    }
  });
}

startRenderStream();