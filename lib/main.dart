import 'package:flutter/material.dart';
import 'src/rust/frb_generated.dart';
import 'src/rust/api/interpreter.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const NavoaApp());
}

class NavoaApp extends StatelessWidget {
  const NavoaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Navoa Studio',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(useMaterial3: true),
      home: const NavoaHomeScreen(),
    );
  }
}

class NavoaHomeScreen extends StatefulWidget {
  const NavoaHomeScreen({super.key});

  @override
  State<NavoaHomeScreen> createState() => _NavoaHomeScreenState();
}

class _NavoaHomeScreenState extends State<NavoaHomeScreen> {
  final TextEditingController _codeController = TextEditingController(
    text: 'println("Olá, Navoa!");',
  );
  String _output = '';
  bool _isRunning = false;

  void _runCode() async {
    setState(() {
      _isRunning = true;
    });

    try {
      final session = NavoaSession();
      final result = await session.executeCode(code: _codeController.text);
      setState(() {
        _output = result;
      });
    } catch (e) {
      setState(() {
        _output = 'Erro na execução: $e';
      });
    } finally {
      setState(() {
        _isRunning = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Navoa Studio'),
        actions: [
          IconButton(
            icon: _isRunning
                ? const SizedBox(
                    width: 20,
                    height: 20,
                    child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                  )
                : const Icon(Icons.play_arrow, color: Colors.greenAccent),
            onPressed: _isRunning ? null : _runCode,
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Expanded(
              flex: 2,
              child: TextField(
                controller: _codeController,
                maxLines: null,
                expands: true,
                style: const TextStyle(fontFamily: 'monospace', fontSize: 14),
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: 'Código Navoa',
                  alignLabelWithHint: true,
                ),
              ),
            ),
            const SizedBox(height: 16),
            Expanded(
              flex: 1,
              child: Container(
                width: double.infinity,
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Colors.black87,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: SingleChildScrollView(
                  child: Text(
                    _output.isEmpty ? 'Consola de saída...' : _output,
                    style: const TextStyle(
                      fontFamily: 'monospace',
                      color: Colors.greenAccent,
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
