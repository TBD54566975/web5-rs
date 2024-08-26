import 'package:flutter/material.dart';
import 'package:web5_flutter_rust_bridge/src/rust/api/simple.dart';
import 'package:web5_flutter_rust_bridge/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`\n\nAction: Call Rust `create_did_jwk()`\nResult: `${createDidJwk()}`'),
        ),
      ),
    );
  }
}
