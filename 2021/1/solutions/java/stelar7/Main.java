import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        String line = null;

        List<Integer> data = new ArrayList<Integer>();
        while ((line = br.readLine()) != null) {
            data.add(Integer.parseInt(line));
        }

        int current = 0;
        int last = 0;
        int count = 0;
        int current2 = 0;
        int last2 = 0;
        int count2 = 0;

        for (int i = 0; i < data.size(); i++) {
            current = data.get(i);
            
            if (i < data.size() - 2) {
                current2 = data.get(i) + data.get(i + 1) + data.get(i + 2);
            }

            if (i > 0) {
                if (current > last) {
                    count++;
                }

                if (current2 > last2) {
                    count2++;
                }
            }

            last = current;
            last2 = current2;
        }

        System.out.println(count);
        System.out.println(count2);
    }
}