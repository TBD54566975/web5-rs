import web5.sdk.rust.UniffiLib;

/**
 * A simple main class to act as an acceptance smoke test for the Kotlin
 * web5 binary.
 *
 * See README.md in this folder for usage and purpose.
 */
public class Web5AcceptanceTest {

    public static void main(String... args) {
        System.setProperty("WEB5_SDK_LOG_LEVEL", "debug");
        UniffiLib.Companion.getINSTANCE$web5();
        System.out.println(
                "Successfully loaded shared library for " +
                        System.getProperty("uniffi.component.web5.libraryOverride"));
    }
}
