
/**
 * Aoc5
 */

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.BinaryOperator;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Aoc5 {

    record Range(long dest, long src, long len) {
    }

    record Mapping(String src, String dest, List<Range> ranges) {
        static Mapping parse(String section) {
            var parts = section.split(":");
            var header = parts[0].replace(" map", "").split("-to-");
            var src = header[0];
            var dest = header[1];
            var ranges = Stream.of(parts[1].trim().split("\n"))
                    .map(line -> {
                        var numbers = line.split(" ");
                        return new Range(Long.parseLong(numbers[0]),
                                Long.parseLong(numbers[1]),
                                Long.parseLong(numbers[2]));
                    }).toList();
            return new Mapping(src, dest, ranges);
        }

        long convert(long number) {
            var value = ranges.stream().filter(r -> number >= r.src && number <= r.src + r.len)
                    .map(r -> number + (r.dest - r.src))
                    .findFirst()
                    .orElse(number);
            return value;
        }
    }

    static class Almanac {

        Map<String, Map<String, Mapping>> getMappingCache = new HashMap<>();
        List<Long> seeds;
        List<Mapping> mappings;

        Almanac(List<Long> seeds, List<Mapping> mappings) {
            this.seeds = seeds;
            this.mappings = mappings;
        }

        static Almanac parse(String fileName) throws IOException {
            var text = Files.readString(Path.of(fileName));
            var sections = text.split("\n\n");
            var seeds = new ArrayList<Long>();
            for (String seed : sections[0].split(":")[1].trim().split(" ")) {
                seeds.add(Long.parseLong(seed.trim()));
            }
            var mappings = Stream.of(sections).skip(1).map(section -> Mapping.parse(section))
                    .toList();
            return new Almanac(seeds, mappings);
        }

        Mapping getMapping(String src, String dest) {
            var srcMap = getMappingCache.get(src);
            if (srcMap != null) {
                return srcMap.get(dest);
            }
            var newMapping = mappings.stream().filter(mapping -> mapping.src.equals(src) && mapping.dest.equals(dest))
                    .findFirst()
                    .orElseThrow(() -> new RuntimeException("Mapping not found: " + src + " " + "dst"));
            getMappingCache.put(src, Map.of(dest, newMapping));
            return newMapping;
        }
    }

    static List<String> categories = List.of(
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location");

    static void part1(Almanac almanac) {
        long lowest = 9999999999l;
        for (long seed : almanac.seeds) {
            var location = seed;
            for (int i = 0; i < categories.size() - 1; i++) {
                var mapping = almanac.getMapping(categories.get(i), categories.get(i + 1));
                location = mapping.convert(location);
            }
            lowest = (long) Math.min(lowest, location);
        }
        System.out.printf("%d\n", lowest);
    }

    record startLen(long start, long len) {
    }

    static void part2(Almanac almanac) {
        var seeds = new ArrayList<startLen>();
        for (int j = 0; j < almanac.seeds.size(); j += 2) {
            seeds.add(new startLen(almanac.seeds.get(j), almanac.seeds.get(j + 1)));
        }
        var lowestAll = seeds.stream().parallel().map(startLen -> {
            var start = startLen.start;
            var len = startLen.len;
            var lowest = 999999999999l;
            for (long seed = start; seed < start + len; seed++) {
                var location = seed;
                for (int i = 0; i < categories.size() - 1; i++) {
                    var mapping = almanac.getMapping(categories.get(i), categories.get(i + 1));
                    location = mapping.convert(location);
                }
                lowest = Math.min(lowest, location);
            }
            return lowest;
        }).mapToLong(n -> n).min().orElse(0);
        System.out.println(lowestAll);
    }

    public static void main(String... args) throws IOException {
        var almanac = Almanac.parse("/home/humberto/projects/aoc/2023/05.input");
        // part1(almanac);
        part2(almanac);
    }

}
