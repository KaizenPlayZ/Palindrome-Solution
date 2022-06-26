package org.example;
class Main {
    public static void main(String[] args) {
        System.out.println(Main.isPalindrome(121));
    }
    public static boolean isPalindrome(int x) {
        if(x < 0 || (x % 10 == 0 && x != 0)) {
            return false;
        }
        int y = 0;
        int z = x;
        while (z > 0) {
            y = y*10 + z%10;
            z /=10;
        }
        if (x == y) {
            return true;
        }
        return false;
    }
}
