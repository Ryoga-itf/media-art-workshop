function setup() {
  const baseW = 96 * 2;
  const baseH = 72 * 2;
  const scaling = Math.min(windowWidth / baseW, windowHeight / baseH);
  pixelDensity(1);
  const canvas = createCanvas(baseW, baseH, WEBGL);
  canvas.elt.style.width = `${baseW * scaling}px`;
  canvas.elt.style.height = `${baseH * scaling}px`;
  canvas.elt.style.imageRendering = "pixelated";
  noSmooth();

  background(0);
  colorMode(HSL);

  // background
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const nx = 0.02 * x;
      const ny = 0.02 * y;
      const c = 20 * noise(nx, ny);
      stroke(c)
      const cx = x - width / 2;
      const cy = y - height / 2;
      point(cx, cy);
    }
  }

  // stars
  strokeWeight(0.5);
  for (let i = 0; i < 100; i++) {
    stroke(random(40, 150));
    point(random(0, width) - width / 2, random(0, height) - height / 2, 1);
  }

  // planet
  strokeWeight(3);
  drawOrb(26.45, 0, 0.1, (x, y, c) => {
    stroke(120 + 16 * Math.floor(c * 10), 100, 50);
    point(x, y);
  })

  // clouds
  colorMode(RGB);
  stroke(255, 255, 255, 128);
  drawOrb(30, 50, 0.085, (x, y, c) => {
    if (c >= 0.58) {
      point(x, y);
    }
  })

  // moon?
  drawOrb(6, 0, 0.1, (x, y, c) => {
    stroke(120 + 16 * Math.floor(c * 10), 100, 50);
    point(x + 50, y - 30);
  })
}

function drawOrb(r, t, noiseScale, drawFunc) {
  for (let y = 0; y <= 2 * (r + 1); y++) {
    for (let x = 0; x <= 2 * (r + 1); x++) {
      const nx = noiseScale * x;
      const ny = noiseScale * y;

      const cx = x - r;
      const cy = y - r;
      const c = noise(nx, ny, t);
      if (cx * cx + cy * cy < r * r) {
        drawFunc(cx, cy, c);
      }
    }
  }
}
