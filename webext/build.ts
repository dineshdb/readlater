#!/usr/bin/env -S pkgx deno@2 run -A

import sharp from "npm:sharp";
import { Buffer } from "node:buffer";

export function icon(text: string, background: string, color: string): string {
  return `
	<svg width="2048" height="2048" viewBox="0 0 2048 2048" xmlns="http://www.w3.org/2000/svg">
		<rect width="2048" height="2048" fill="${background}" rx="320"/>
		<text x="50%" y="83%" font-family="Arial, sans-serif" font-size="2000" fill="${color}" 
			text-anchor="middle" alignment-baseline="middle">${text}</text>
	</svg>
	`;
}

export function buildIcon(
  variant: string,
  text: string,
  background: string,
  color: string,
  size: number,
) {
  sharp(Buffer.from(icon(text, background, color))).resize(size, size).toFile(
    `icons/icon-${variant}-${size}.png`,
  );
}

const icons = {
  gray: {
    background: "#f5f5f5", color: "#333", sizes: [
      16, 19, 32, 38, 48, 64, 128,
    ]
  },
  orange: { background: "orange", color: "white", sizes: [19, 38] },
  green: { background: "green", color: "white", sizes: [19, 38] },
  red: { background: "red", color: "white", sizes: [19, 38] },
}

for (const [variant, { background, color, sizes }] of Object.entries(icons)) {
  for (const size of sizes) {
    buildIcon(variant, "rl", background, color, size);
  }
}
