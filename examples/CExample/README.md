## Example Description

This C example demonstrates basic usage of the Web5 SDK functions through the C bindings. Specifically, it showcases:

1. Resolving a `did:dht` identifier
2. Printing the resolved DID document or an error message

This example demonstrates how to resolve a DHT-based DID using the Web5 SDK in C. It uses the `did_dht_resolve` function with a hardcoded DID URI and the default gateway. The program then prints either the resolved DID document or an error message to the console.


## Running the Example

To run this C example:

1. Ensure you have a C compiler installed (e.g. GCC) and [Just](https://github.com/casey/just) command runner.

2. Navigate to the root directory of the project where the main Justfile is located.

3. Generate the C dynamic library by running:

   ```
   just bindc
   ```

   This command will build the necessary C bindings for the Web5 SDK.

4. Navigate to the `examples/CExample` directory where the example-specific Justfile is located.

5. Compile the example using the Justfile recipe:

   ```
   just compile
   ```

   This command will create a `build` directory and compile the C example using GCC with the appropriate flags and library paths.

6. Run the compiled example:

   ```
   just run
   ```

   This will compile (if not already done) and execute the example program, demonstrating the usage of Web5 SDK functions in C.

7. To clean up the build artifacts:

   ```
   just clean
   ```

   This will remove the `build` directory.
