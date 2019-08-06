var addon = require('../native');

class RenderStream {
  constructor(imgData, camera, callback) {
    this.imgData = imgData;
    this.camera = camera;
    this.stream = new addon.RenderStream(imgData, camera);
    this.finished = false;

    let self = this;

    function pollEvent() {
      console.log("Starting polling event");
      self.stream.poll((event) => {
        console.log("Polled");
        if (event) {
          console.log("has event");
          if (event.type === 'set_pixel') {
            console.log("setting pixel");
            for (let j = event.y; j < event.y + event.height; j++) {
              for (let i = event.x; i < event.x + event.width; i++) {
                const index = (j * imgData.width + i) * 4;
                imgData[index] = event.r;
                imgData[index + 1] = event.g;
                imgData[index + 2] = event.b;
                imgData[index + 3] = event.a;
              }
            }
            pollEvent();
          } else if (event.type === 'update') {
            console.log("update");
            callback(event);
            pollEvent();
          } else if (event.type === 'finish') {
            console.log("finished");
            callback(event);
            self.finished = true;
          }
        } else {
          console.log("undefined event");
          pollEvent();
        }
      });
    }

    pollEvent();
  }

  close() {
    this.stream.shutdown();
  }

  isFinished() {
    return this.finished;
  }
}

module.exports = {
  mainCamera: {
    target: {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    azimuth: 0.0,
    incline: 0.0,
    distance: 3.0,
  },

  render(imgData) {
    const start = new Date();
    addon.render(imgData, this.mainCamera);
    const end = new Date();
    console.log(`[render] time elapsed: ${end - start}`);
  },

  fillBlack(imgData) {
    const start = new Date();
    addon.fillBlack(imgData);
    const end = new Date();
    console.log(`[fillBlackRust] time elapsed: ${end - start}`);
  },

  createRenderStream(imgData, callback) {
    return new RenderStream(imgData, this.mainCamera, callback);
  },
}