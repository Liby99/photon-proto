var addon = require('../native');

class RenderStream {
  constructor(imgData, camera, callback) {
    this.imgData = imgData;
    this.camera = camera;
    this.stream = new addon.RenderStream(imgData, camera);
    this.finished = false;

    let self = this;

    function pollEvent() {
      if (self.finished) {
        self.stream.shutdown();
        return;
      }

      self.stream.poll((_, event) => {
        if (event) {
          if (event.type === 'set_pixel') {
            for (let j = event.y; j < event.y + event.h; j++) {
              for (let i = event.x; i < event.x + event.w; i++) {
                const index = (j * imgData.width + i) * 4;
                self.imgData.data[index] = event.r;
                self.imgData.data[index + 1] = event.g;
                self.imgData.data[index + 2] = event.b;
                self.imgData.data[index + 3] = event.a;
              }
            }
            setImmediate(pollEvent);
          } else if (event.type === 'update') {
            callback(event);
            setImmediate(pollEvent);
          } else if (event.type === 'finish') {
            callback(event);
            self.finished = true;
          }
        } else {
          console.log("undefined event");
          setImmediate(pollEvent);
        }
      });
    }

    setImmediate(pollEvent);
  }

  close() {
    this.finished = true;
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
    incline: 0.3,
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