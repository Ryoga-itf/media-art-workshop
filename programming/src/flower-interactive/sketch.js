let faceMesh, video, faces = [];
let options = { maxFaces: 1, refineLandmarks: false, flipHorizontal: false };
let mouthOpenness = 0;

let patals, baseR, hand;

function preload() {
  faceMesh = ml5.faceMesh(options);
  hand = loadImage("hand.png");
}

function setup() {
  const baseW = 728, baseH = 500;
  const scaling = Math.min(windowWidth / baseW, windowHeight / baseH);
  const canvas = createCanvas(baseW, baseH);
  canvas.elt.style.width = `${baseW * scaling}px`;
  canvas.elt.style.height = `${baseH * scaling}px`;

  background(0);

  video = createCapture(VIDEO);
  video.size(width, height);
  video.hide();

  faceMesh.detectStart(video, (results) => {
    faces = results;
    if (faces.length) {
      const keypoints = faces[0].keypoints;
      const upperLip = keypoints[13], lowerLip = keypoints[14];
      mouthOpenness = upperLip && lowerLip ? dist(upperLip.x, upperLip.y, lowerLip.x, lowerLip.y) : 0;
    } else {
      mouthOpenness = 0;
    }
  });

  petals = 10;
  baseR = min(width, height) * 0.5;
}

function draw() {
  colorMode(HSB, 360, 100, 100, 100);
  const t = hour() + minute() / 60;
  const bgHue = map(t, 0, 24, 220, 40);
  background(bgHue, 40, 15, 100);

  const radiusFactor = lerp(0.6, 1.6, constrain(mouthOpenness, 0, 50) / 50);
  const img = drawShiftGlitch(drawFlower(radiusFactor), 10);
  image(img, 0, 0);

  stroke(0, 50);
  strokeWeight(1);
  for (let i = 0; i < height; i += height / 200) {
    line(0, i, width, i);
  }
}

function drawFace(keypoints) {
  const baseW = 100, baseH = 100;
  if (!keypoints || keypoints.length == 0) {
    return;
  }

  // bounding box
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const kp of keypoints) {
    if (kp) {
      minX = min(minX, kp.x);
      minY = min(minY, kp.y);
      maxX = max(maxX, kp.x);
      maxY = max(maxY, kp.y);
    }
  }

  // is bounding box is valid?
  if (!isFinite(minX) || !isFinite(minY) || !isFinite(maxX) || !isFinite(maxY)) {
    return;
  }

  const pointsWidth = maxX - minX;
  const pointsHeight = maxY - minY;

  if (pointsWidth <= 0 || pointsHeight <= 0) {
    return;
  }

  const scale = min(baseW / pointsWidth, baseH / pointsHeight);

  push();
  translate((width - baseW) / 2, (height - baseH) / 2);
  fill(255);
  noStroke();

  for (const kp of keypoints) {
    if (kp) {
      circle((kp.x - minX) * scale, (kp.y - minY) * scale, 2);
    }
  }
  pop();
}

function drawFlower(factor) {
  push();

  colorMode(HSB, 360, 100, 100, 100);
  imageMode(CENTER);
  translate(width / 2, height / 2);
  rotate(frameCount * 0.003 * factor);

  for (let i = 0; i < petals; i++) {
    push();
    const angle = TWO_PI / petals * i;
    rotate(angle);

    const dynamicR = factor * baseR + sin(frameCount * 0.02 + i) * 15;
    translate(0, -dynamicR);

    rotate(-PI / 2);

    const s = map(sin(frameCount * 0.015 + i), -1, 1, 0.8, 1.2);
    scale(s);

    tint((frameCount * 0.5 + i * (360 / petals)) % 360, 80, 70, 80);

    image(hand, 0, 0);
    pop();
  }

  pop();

  if (faces.length > 0) {
    drawFace(faces[0].keypoints);
  }

  const img = get();
  clear();
  return img;
}

function drawShiftGlitch(img, shift_size) {
  image(img, 0, 0);

  for (let i = 0; i < 100; i++) {
    const sx = random(img.width * 0.5);
    const sy = random(img.height * 0.05);
    const x = random(img.width - sx * 0.5);
    const y = random(img.height - sy * 0.5);
    const ix = x + random(-1, 1) * shift_size;
    const iy = y;
    image(img, ix, iy, sx, sy, x, y, sx, sy);
  }

  const img_glitch = get();
  clear();
  return img_glitch;
}
