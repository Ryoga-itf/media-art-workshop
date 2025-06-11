const DISPLAY_TEXT = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!?';
const FONT_SIZE = 130;

let font;
let glyphs = [];

function preload() {
  font = loadFont('Lato-Regular.ttf');
}

function setup() {
  const baseW = 1280;
  const baseH = 720;
  const scaling = Math.min(windowWidth / baseW, windowHeight / baseH);
  const canvas = createCanvas(baseW, baseH);
  canvas.elt.style.width = `${baseW * scaling}px`;
  canvas.elt.style.height = `${baseH * scaling}px`;

  noFill();
  strokeWeight(3);
  stroke(64);
  textFont(font);
  textSize(FONT_SIZE);
  initGlyphs();
}

function draw() {
  background(252, 245, 237);

  const t = getSawtooth(2600);
  const lenFact = getSine01(16000) * 0.5;

  drawAllGlyphs(t, lenFact);
}

function initGlyphs() {
  // get font's outlines
  for (let ch of DISPLAY_TEXT) {
    const pts = font.textToPoints(ch, 0, 0, FONT_SIZE, {
      sampleFactor: 0.25,
      simplifyThreshold: 0
    });
    const adv = textWidth(ch);
    glyphs.push({ pts, adv });
  }
}

function drawAllGlyphs(t, lenFact) {
  const BASE_X = 70;
  const MAX_X = 1120;
  let penX = BASE_X;
  let penY = textAscent();

  for (let g of glyphs) {
    drawGlyphSegments(g, penX, penY, t, lenFact);
    penX += g.adv;
    if (penX > MAX_X) {
      penX = BASE_X;
      penY += FONT_SIZE;
    }
  }
}

function drawGlyphSegments(glyph, penX, penY, t, lenFact) {
  const pts = glyph.pts;
  const N = pts.length;
  if (N === 0) return;

  const segLen = floor(N * lenFact);
  const start1 = floor(t * N);
  const start2 = floor((t + 0.5) * N);

  const seg1 = extractSegment(pts, start1, segLen);
  const seg2 = extractSegment(pts, start2, segLen);

  beginShape();
  seg1.concat(seg2.reverse()).forEach(p => {
    vertex(p.x + penX, p.y + penY);
  });
  endShape(CLOSE);
}

// utilities

// sawtooth -> [0, 1]
function getSawtooth(periodMs) {
  return (millis() % periodMs) / periodMs;
}

// sin -> [0, 1]
function getSine01(periodMs) {
  return (sin(TWO_PI * (millis() % periodMs) / periodMs) + 1) * 0.5;
}

function extractSegment(pts, start, len) {
  const seg = [];
  const N = pts.length;
  for (let i = 0; i < len; i++) {
    seg.push(pts[(start + i) % N]);
  }
  return seg;
}
