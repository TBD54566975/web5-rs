## Example Description

This C example demonstrates basic usage of the Web5 SDK functions through the C bindings. Specifically, it showcases:

1. Resolving a `did:dht` identifier
2. Printing the resolved DID document or an error message

This example demonstrates how to resolve a DHT-based DID using the Web5 SDK in C. It uses the `did_dht_resolve` function with a hardcoded DID URI and the default gateway. The program then prints either the resolved DID document or an error message to the console.

## Running the Example

To run this C example:

1. Ensure you have a C compiler installed (e.g. GCC).

2. Navigate to the root directory of the project (where you cloned the git repository). It's the same location this [Justfile](../../Justfile) is located.

3. Generate the C dynamic library by running:

   ```shell
   just bindc
   ```

   This command will build the necessary C bindings for the Web5 SDK.

4. Navigate to the `examples/CExample` directory:

   ```shell
   cd examples/CExample
   ```

5. Compile the example using the Justfile recipe:

   ```shell
   just compile
   ```

   This command will create a `build` directory and compile the C example using GCC with the appropriate flags and library paths.

6. Run the compiled example:

   ```shell
   just run
   ```

   This will compile (if not already done) and execute the example program, demonstrating the usage of Web5 SDK functions in C.

7. To clean up the build artifacts:

   ```shell
   just clean
   ```

   This will remove the `build` directory.

## Available Justfile Recipes

Before running any of the recipes below, ensure you have executed the following command from the root directory of the project.

The recipes available for this example are:

| Recipe   | Description                                                           |
|----------|-----------------------------------------------------------------------|
| compile  | Compiles the C example, creating a `build` directory with the binary  |
| run      | Compiles (if needed) and runs the example program                     |
| clean    | Removes the `build` directory, cleaning up all build artifacts        |