
package day1;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.*;
import java.util.*;

import java.util.stream.*;

public class Day1 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        Path path = Paths.get(br.readLine());
        List<String> lines = Files.readAllLines(path);
        List<Integer> listInteger;
        listInteger = lines.stream().map(Integer::parseInt).collect(Collectors.toList());
        List<Integer> sumList = new ArrayList<>();
        int counter = 0;
        int counter2 = 0;
        int filler = 0;

        for (int l = 0; l < listInteger.size()-2; l++) {

            sumList.add(listInteger.get(l) + listInteger.get(l + 1) + listInteger.get(l + 2));

        }
      //This technique also works for part 1
        for (int x = 1; x < sumList.size(); x++) {
            if (sumList.get(x) > sumList.get(x - 1)) {
                counter2++;
            } else {

            }
        }

        System.out.println("Result: "+counter2);

    }

}
