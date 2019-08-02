``` js
const scene = Photon.createScene();
scene.addObject({
  transform: {
    position: [0.0, 0.0, 0.0],
    scale: [0.0, 0.0, 0.0],
    rotation: [0.0, 0.0, 0.0, 1.0],
  },
  intersectable: "sphere",
})

const stream = Photon.createRenderStream(scene, camera, imgData);

stream.on("update", () => {
  canvas.putImageData(imgData, 0, 0);
});

stream.on("finish", () => {

});

stream.on("start", () => {

});

stream.updateCamera(camera);
stream.updateScene(scene);
stream.start();
```