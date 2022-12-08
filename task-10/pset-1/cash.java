import java.util.*;
public class cash{
    public static void main(String []args){
        Scanner sc =new Scanner(System.in);
        System.out.println("change owed in dollar:");
        float change = sc.nextFloat();
        int cents = (int) Math.round(change * 100);
        int count = 0;
        while (cents >= 25){
            cents -= 25;
            count++;
        }
        while (cents >= 10){
            cents -= 10;
            count++;
        }
        while (cents >= 5){
            cents -= 5;
            count++;
        }
        while (cents >= 1){
            cents -= 1;
            count++;
        }
        System.out.println("Number of coins:"+count);   
    }   
}