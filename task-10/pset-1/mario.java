import java.util.*;
public class mario{
    public static void main(String[] args) {
        Scanner sc= new Scanner(System.in);
        System.out.println(" Enter the height: ");
        int height = sc.nextInt();
        if (height <0 || height >8){
            System.out.println("Height:"+height);
        }
        else{
            
            System.out.println("Height:"+height);

            for (int i= 0; i<= height; i++)

            {
            
            for (int j=1; j<=height-i; j++)
            
            {
            
            System.out.print(" ");
            
            }
            
            for (int k=0;k<=i;k++)
            
            {
            
            System.out.print("#");
            
            }
            
            System.out.println("");
            
            }
            
            
        }
    }
}
