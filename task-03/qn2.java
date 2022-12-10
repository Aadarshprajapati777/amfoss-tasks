import java.util.Scanner;
public class qn2{

    public static void main(String[] args) {
        
        Scanner s = new Scanner(System.in);
    
        // System.out.println("Enter the no of groups :: ");
        int t = s.nextInt();
    
while(t>0){
    boolean flag=true;

    // System.out.println("Enter the no of monsters :: ");
    int n = s.nextInt();
    
    // System.out.println("Enter the health of monsters :: ");
    int health[] = new int[n];
    for( int i = 0; i < n; i++) {
        health[i] = s.nextInt();
    }

 
if(health[0] == 1 ){
      System.out.println("YES");
    }
else{
    for (int i = 1; i < n; i++) {
        if(health[i]%health[0]!=0){
            flag = false;
        }
    }
    if(flag){
        System.out.println("YES");
    }
    else{
        System.out.println("NO");
    }
    }
    t--;
}
    }
}