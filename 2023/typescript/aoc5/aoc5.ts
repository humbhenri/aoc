import * as fs from "node:fs";

type converter = {
  dest_range_start: number;
  source_range_start: number;
  range_len: number;
};

function convert(source: number, converters: Array<converter>): number {
  const mapping = converters.filter(
    (c) =>
      source >= c.source_range_start &&
      source <= c.source_range_start + c.range_len
  );
  if (!mapping.length) {
    return source;
  }
  const diff = mapping[0].dest_range_start - mapping[0].source_range_start;
  return source + diff;
}

type mapping = { source: string; dest: string; converters: Array<converter> };

function parse_mapping(text: string): mapping {
  const [header, rest] = text.split(":");
  const { source, dest } = header.match(
    /(?<source>\w+)-to-(?<dest>\w+)/
  ).groups;
  const converters = [];
  rest
    .trim()
    .split("\n")
    .forEach((line) => {
      const [dest_range_start, source_range_start, range_len] = line
        .trim()
        .split(" ")
        .map((n) => parseInt(n, 10));
      converters.push({ dest_range_start, source_range_start, range_len });
    });
  return { source, dest, converters };
}

type almanac = { seeds: Array<number>; mappings: Array<mapping> };

function parse_almanac(file_name: string): almanac {
  const text = fs.readFileSync(file_name, "utf-8");
  const info = text.split("\n\n");
  const seeds = info[0]
    .split(":")[1]
    .trim()
    .split(" ")
    .map((n) => parseInt(n, 10));
  const mappings = info.slice(1).map((line) => parse_mapping(line));
  return { seeds, mappings };
}

function get_mapping(almanac: almanac, source: string, dest: string): mapping {
  const mapping = almanac.mappings.filter(
    (mapping) => mapping.source === source && mapping.dest === dest
  );
  if (!mapping.length) {
    throw `Mapping from ${source} to ${dest} not found`;
  }
  return mapping[0];
}

const categories = [
  "seed",
  "soil",
  "fertilizer",
  "water",
  "light",
  "temperature",
  "humidity",
  "location",
];

function part1(file_name: string) {
  const almanac = parse_almanac(file_name);
  let lowest = 1e9;
  almanac.seeds.forEach((seed) => {
    let location = seed;
    for (let i = 0; i < categories.length - 1; i++) {
      const [source, dest] = [categories[i], categories[i + 1]];
      location = convert(
        location,
        get_mapping(almanac, source, dest).converters
      );
    }
    lowest = Math.min(lowest, location);
  });
  console.log(lowest);
}

part1("/home/humberto/projects/aoc/2023/05.input");
