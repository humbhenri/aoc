import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Stream;

public class Prob05 {
    private static HashMap<String, List<String>> rules = new HashMap<>();
    private static List<List<String>> updates = new ArrayList<>();
    private static List<List<String>> correctUpdates = new ArrayList<>();
    private static List<List<String>> incorrectUpdates = new ArrayList<>();

    private static void parseRules(String text) {
        Stream.of(text.split("\n")).forEach(line -> {
            var parts = line.split("\\|");
            var key = parts[0];
            var value = parts[1];
            rules.computeIfAbsent(key, k -> new ArrayList<>()).add(value);
        });
    }

    private static void parseUpdates(String text) {
        Stream.of(text.split("\n")).forEach(line -> {
            var update = new ArrayList<String>();
            Stream.of(line.split(",")).forEach(number -> {
                update.add(number);
            });
            updates.add(update);
        });
    }

    private static int middleValuesSum(List<List<String>> updates) {
        return updates.stream().mapToInt(update -> {
            var middleIndex = update.size() / 2;
            return Integer.parseInt(update.get(middleIndex));
        }).sum();
    }

    private static void fixOrder(List<String> update) {
        for (int i = 0; i < update.size(); i++) {
            for (int j = i + 1; j < update.size(); j++) {
                boolean wrongOrder = rules.containsKey(update.get(j))
                        && rules.get(update.get(j)).contains(update.get(i));
                if (wrongOrder) {
                    var temp = update.get(i);
                    update.set(i, update.get(j));
                    update.set(j, temp);
                    fixOrder(update);
                }
            }
        }
    }

    private static void findCorrectUpdates() {
        updates.stream().forEach(update -> {
            var copy = new ArrayList<>(update);
            fixOrder(update);
            if (copy.equals(update)) {
                correctUpdates.add(update);
            } else {
                incorrectUpdates.add(update);
            }
        });
    }

    public static void main(String[] args) throws IOException {
        String filename = "05.input";
        Path path = Paths.get(filename);
        String content = Files.readString(path);
        String[] sections = content.split("\n\n");
        parseRules(sections[0]);
        parseUpdates(sections[1]);
        findCorrectUpdates();
        System.out.println("Part 1: " + middleValuesSum(correctUpdates));
        System.out.println("Part 2: " + middleValuesSum(incorrectUpdates));
    }
}
