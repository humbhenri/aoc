import * as fs from "node:fs";
const input = fs.readFileSync("/home/humberto/projects/aoc/2023/03.input", "utf8").split("\n");
let result = 0;

for (let i = 0; i < input.length; i++) {
    const numbers = input[i].replace(/\./g, " ");
    for (const match of numbers.matchAll(/\d+/g)) {
        for (let j = match.index; j < match.index + match[0].length; j++) {
            const surrounding = [
                (input[i - 1] ?? "")[j - 1] ?? ".",
                (input[i - 1] ?? "")[j] ?? ".",
                (input[i - 1] ?? "")[j + 1] ?? ".",
                (input[i] ?? "")[j - 1] ?? ".",
                (input[i] ?? "")[j] ?? ".",
                (input[i] ?? "")[j + 1] ?? ".",
                (input[i + 1] ?? "")[j - 1] ?? ".",
                (input[i + 1] ?? "")[j] ?? ".",
                (input[i + 1] ?? "")[j + 1] ?? "."
            ];
            if (surrounding.some(x => /[^0-9.]/.test(x))) {
                result += parseInt(match[0]);
                break;
            }
        }
    }
}

console.log(result);
