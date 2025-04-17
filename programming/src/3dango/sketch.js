let font;
function preload() {
  font = loadFont("./PICO-8_mono_upper.ttf");
}

function setup() {
  const baseW = 96;
  const baseH = 72;
  const scaling = Math.min(windowWidth / baseW, windowHeight / baseH);
  pixelDensity(1);
  const canvas = createCanvas(baseW, baseH, WEBGL);
  canvas.elt.style.width = `${baseW * scaling}px`;
  canvas.elt.style.height = `${baseH * scaling}px`;
  canvas.elt.style.imageRendering = "pixelated";
  noSmooth();
  textFont(font, 5);
  textAlign(CENTER, TOP);
}

function draw() {
  const factor = Math.min(width, height);
  const r = factor * 0.3;
  const d = factor * 0.15;
  background(170);

  stroke(194, 127, 127);
  strokeWeight(factor * 0.04);
  line(-2.5 * d, 2.5 * d, d, -d);

  noStroke();
  fill("#b7d28f");
  circle(-d, d, r);
  fill("#fff");
  circle(0, 0, r);
  fill("#f3d1e1");
  circle(d, -d, r);

  fill("gray");
  text("3-shoku dango", 0, 0);
}
