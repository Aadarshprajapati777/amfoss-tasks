pimport java.io.*;
import java.util.*;

public class Solution {

    public static void main(String[] args) {
        /* Enter your code here. Read input from STDIN. Print output to STDOUT. Your class should be named Solution. */
        Scanner s = new Scanner(System.in);
        // System.out.println("Enter the no of facitlites:: ");
        int t = s.nextInt();


        while(t>0){
            int count=0;

    // System.out.println("Enter the no of water tanks :: ");
    int n = s.nextInt();
    // System.out.println("Enter the amount of water in each tank :: ");
    int water[] = new int[n];
    for( int i = 0; i < n; i++) {
        water[i] = s.nextInt();
    }


    for (int i = 1; i < n; i++) {
if(water[i]==0 && water[i-1]!=0)     {
        water[i] = water[i]+1;
        water[i-1] = water[i-1]-1;
        count++;
}
        }
    for(int i=0;i<n-1;i++){
        count=count+water[i];
    };
    System.out.println(count);
   
    t--;
}
  
    
    }



        
    }
