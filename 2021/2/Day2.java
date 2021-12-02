/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package day2;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.io.IOException;
import static java.lang.Integer.parseInt;
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
    public static void main(String[] args)throws IOException {
        String source = "input.txt";
        Path path = Paths.get(source);
        List<String> lines = Files.readAllLines(path);
        int forward = 0;
        int depth = 0;
        
        for(int i = 0;i<lines.size();i++)
        {
          Pattern pattern = Pattern.compile("[a-z]", Pattern.CASE_INSENSITIVE);
    Matcher matcher = pattern.matcher(lines.get(i));
            if(matcher.find())
            {
                
                if("f".equals(matcher.group()))
                {
                    Pattern pattern1 = Pattern.compile("[0-9]", Pattern.CASE_INSENSITIVE);
    Matcher matcher1 = pattern1.matcher(lines.get(i));
                if(matcher1.find())
                {
                    int b = Integer.parseInt(matcher1.group());

                    forward = forward + b;
                    
                }
                }
                
                else if("d".equals(matcher.group()))
                        {
                            Pattern pattern1 = Pattern.compile("[0-9]", Pattern.CASE_INSENSITIVE);
    Matcher matcher1 = pattern1.matcher(lines.get(i));
                            if(matcher1.find())
                {
                    int b = Integer.parseInt(matcher1.group());

                    depth = depth + b;
                    
                }
                        }
                
                else if("u".equals(matcher.group()))
                        {
                            Pattern pattern1 = Pattern.compile("[0-9]", Pattern.CASE_INSENSITIVE);
    Matcher matcher1 = pattern1.matcher(lines.get(i));
                            if(matcher1.find())
                {
                    int b = Integer.parseInt(matcher1.group());

                    depth = depth - b;
                    
                }
                        }
            }
            
        }
        System.out.println(depth);
            System.out.println(forward);
    }
    
}
