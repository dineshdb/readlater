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

buildIcon("gray", "rl", "#f5f5f5", "#333", 128);
buildIcon("gray", "rl", "#f5f5f5", "#333", 19);
buildIcon("gray", "rl", "#f5f5f5", "#333", 24);
buildIcon("gray", "rl", "#f5f5f5", "#333", 38);
buildIcon("gray", "rl", "#f5f5f5", "#333", 48);
buildIcon("gray", "rl", "#f5f5f5", "#333", 96);
buildIcon("orange", "rl", "orange", "white", 48);
buildIcon("green", "rl", "green", "white", 48);
buildIcon("red", "rl", "red", "white", 48);
