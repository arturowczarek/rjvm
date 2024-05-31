public class MyClass implements A, B{
    private int privateInt = 9;
    public int publicInt = 2;
    String packageString = "jjj";

    public static final int staticFinalValue = 989;
    public int intMethod() {
        return 9;
    }

    public static int staticIntMethod() {
        return 54;
    }

    public int methodFromA(int parameterInA) {
        return parameterInA + 2;
    }

    public String methodFromB() {
        return "methodFromBResponse";
    }

    public static void main(String[] args) {
        int x = 9;
        int y = 11;
        int z = x + y;
        System.out.println("Value is " + z);
    }
}
