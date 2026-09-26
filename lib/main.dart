import 'package:flutter/material.dart';
import 'src/rust/api/interpreter.dart';
import 'src/rust/frb_generated.dart';

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
      title: 'Navoa OS',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF0D0C07),
      ),
      home: const NavoaScreen(),
    );
  }
}

class NavoaScreen extends StatefulWidget {
  const NavoaScreen({super.key});

  @override
  State<NavoaScreen> createState() => _NavoaScreenState();
}

class _NavoaScreenState extends State<NavoaScreen> {
  late final InterpreterBridge _bridge;
  final TextEditingController _controller = TextEditingController();
  final List<String> _logs = [
    "> SISTEMA OPERACIONAL NAVOA // TERMINAL DE INVESTIGAÇÃO",
    "> Digita 'inspecionar quarto' para começar."
  ];

  @override
  void initState() {
    super.initState();
    _bridge = InterpreterBridge();
  }

  void _executarComando() {
    final cmd = _controller.text.trim();
    if (cmd.isEmpty) return;

    setState(() {
      _logs.add("> $cmd");
      try {
        final res = _bridge.execute(code: cmd);
        _logs.add(res);
      } catch (e) {
        _logs.add("ERRO: $e");
      }
      _controller.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Navoa OS - Retro Noir'),
        backgroundColor: const Color(0xFF1C1A14),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Expanded(
              child: Container(
                padding: const EdgeInsets.all(12),
                width: double.infinity,
                decoration: BoxDecoration(
                  color: Colors.black,
                  border: Border.all(color: const Color(0xFFFFB000)),
                ),
                child: ListView.builder(
                  itemCount: _logs.length,
                  itemBuilder: (context, index) {
                    return Text(
                      _logs[index],
                      style: const TextStyle(
                        color: Color(0xFFFFB000),
                        fontFamily: 'monospace',
                        fontSize: 16,
                      ),
                    );
                  },
                ),
              ),
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _controller,
                    style: const TextStyle(color: Color(0xFFFFB000)),
                    decoration: const InputDecoration(
                      hintText: 'digita um comando...',
                      hintStyle: TextStyle(color: Colors.grey),
                      enabledBorder: OutlineInputBorder(
                        borderSide: BorderSide(color: Color(0xFFFFB000)),
                      ),
                      focusedBorder: OutlineInputBorder(
                        borderSide: BorderSide(color: Color(0xFFFFB000)),
                      ),
                    ),
                    onSubmitted: (_) => _executarComando(),
                  ),
                ),
                const SizedBox(width: 8),
                ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFFFFB000),
                    foregroundColor: Colors.black,
                  ),
                  onPressed: _executarComando,
                  child: const Text('EXECUTAR'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
