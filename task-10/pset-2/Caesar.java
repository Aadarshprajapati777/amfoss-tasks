//CS50 week2 problem set 2
import java.util.Scanner;

public class Caesar{
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    System.out.print("Enter a string: ");
    String str = scanner.nextLine();

    System.out.print("Enter an int: ");
    int num = scanner.nextInt();

    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < str.length(); i++) {
      char ch = str.charAt(i);
      ch = (char)(ch + num);
      sb.append(ch);
    }
    System.out.println(sb);
  }
}
