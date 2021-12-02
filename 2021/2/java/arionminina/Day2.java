package day2;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.io.IOException;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 *
 * @author Arioniii
 */
public class Day2 {

    /**
     * @param args the command line arguments
     * @throws java.io.IOException
     */
    public static void main(String[] args) throws IOException {
        String source = "input.txt";
        Path path = Paths.get(source);
        List<String> lines = Files.readAllLines(path);
        int forward = 0;
        int depth = 0;
        int aim = 0;

        for (int i = 0; i < lines.size(); i++) {
            Pattern pattern = Pattern.compile("[a-z]", Pattern.CASE_INSENSITIVE);
            Matcher matcher = pattern.matcher(lines.get(i));
            if (matcher.find()) {

                if ("f".equals(matcher.group())) {
                    Pattern pattern1 = Pattern.compile("[0-9]");
                    Matcher matcher1 = pattern1.matcher(lines.get(i));
                    if (matcher1.find()) {
                        int b = Integer.parseInt(matcher1.group());

                        forward = forward + b; 
                        //Delete the part before the colon for Part 2: depth = depth + aim * b;

                    }
                } else if ("d".equals(matcher.group())) {
                    Pattern pattern1 = Pattern.compile("[0-9]");
                    Matcher matcher1 = pattern1.matcher(lines.get(i));
                    if (matcher1.find()) {
                        int b = Integer.parseInt(matcher1.group());
                        depth = depth + b; //Delete this line of code for Part 2
                        //Delete the part before the colon for Part 2: aim = aim + b;

                    }
                } else if ("u".equals(matcher.group())) {
                    Pattern pattern1 = Pattern.compile("[0-9]");
                    Matcher matcher1 = pattern1.matcher(lines.get(i));
                    if (matcher1.find()) {
                        int b = Integer.parseInt(matcher1.group());
                         depth = depth - b;  //Delete this line of code for Part 2
                        //Delete the part before the colon for Part 2: aim = aim - b;

                    }
                }
            }

        }

        System.out.println("Result:" + (depth * forward));

    }

}
