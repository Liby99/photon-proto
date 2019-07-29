const Photon = require("../../lib/index");

const canvas = document.getElementById("main-canvas");
const { width, height } = canvas;
const context = canvas.getContext("2d");
const imgData = context.createImageData(width, height);

Photon.render(imgData);

context.putImageData(imgData, 0, 0);