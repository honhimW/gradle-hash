import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;

public class GradleHash {

    public static void main(String[] args) throws Exception {
        String arg = args[0];
        MessageDigest messageDigest = MessageDigest.getInstance("MD5");
        messageDigest.update(arg.getBytes(StandardCharsets.UTF_8));
        byte[] digest = messageDigest.digest();

        BigInteger bigInteger = new BigInteger(1, digest);
        System.out.println(bigInteger.toString(36));
    }

}
