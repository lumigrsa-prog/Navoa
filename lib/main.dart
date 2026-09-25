import 'package:flutter/material.dart';

void main() {
  runApp(const NavoaApp());
}

class NavoaApp extends StatelessWidget {
  const NavoaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Navoa Studio',
      theme: ThemeData.dark(),
      home: Scaffold(
        appBar: AppBar(title: const Text('Navoa Engine')),
        body: const Center(
          child: Text('Ambiente Navoa Pronto!'),
        ),
      ),
    );
  }
}
