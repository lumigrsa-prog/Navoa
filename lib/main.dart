import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:navoa/src/rust/frb_generated.dart';
import 'package:navoa/src/rust/api/interpreter.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  try {
    await RustLib.init();
  } catch (e) {
    debugPrint('Aviso de inicialização do motor Rust: $e');
  }
  runApp(const NavoaApp());
}

class NavoaApp extends StatelessWidget {
  const NavoaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Navoa Studio',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF1E1E1E),
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFF007ACC),
          surface: Color(0xFF252526),
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFF252526),
          elevation: 0,
        ),
      ),
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
    text: '// Exemplo de script na linguagem Navoa\n10 + 20 * 2',
  );
  
  String _output = 'Consola pronta. Escreva o seu código e clique em "Executar".\n';
  bool _isExecuting = false;
  String _statusMessage = 'Pronto';
  Color _statusColor = Colors.grey;

  Future<void> _runCode() async {
    final code = _codeController.text.trim();
    if (code.isEmpty) {
      setState(() {
        _output = '⚠️ O editor está vazio. Digite expressões ou código para executar.\n';
        _statusMessage = 'Código Vazio';
        _statusColor = Colors.orange;
      });
      return;
    }

    setState(() {
      _isExecuting = true;
      _statusMessage = 'A executar no motor Rust...';
      _statusColor = Colors.blue;
    });

    final stopwatch = Stopwatch()..start();

    try {
      final session = NavoaSession();
      final result = await session.executeCode(code: code);
      stopwatch.stop();

      setState(() {
        _output = '[SAÍDA DE EXECUÇÃO - ${stopwatch.elapsedMilliseconds}ms]\n$result\n';
        _statusMessage = 'Sucesso (${stopwatch.elapsedMilliseconds}ms)';
        _statusColor = Colors.green;
      });
    } catch (e) {
      stopwatch.stop();
      setState(() {
        _output = '[ERRO DE EXECUÇÃO]\n$e\n';
        _statusMessage = 'Erro na execução';
        _statusColor = Colors.red;
      });
    } finally {
      setState(() {
        _isExecuting = false;
      });
    }
  }

  void _clearConsole() {
    setState(() {
      _output = 'Consola limpa.\n';
      _statusMessage = 'Pronto';
      _statusColor = Colors.grey;
    });
  }

  void _loadSampleCode() {
    setState(() {
      _codeController.text = '// Operações aritméticas e expressões Navoa\n(100 - 25) / 5 + 3';
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Row(
          children: const [
            Icon(Icons.code_rounded, color: Color(0xFF007ACC)),
            SizedBox(width: 10),
            Text('Navoa Studio', style: TextStyle(fontWeight: FontWeight.bold)),
          ],
        ),
        actions: [
          TextButton.icon(
            onPressed: _loadSampleCode,
            icon: const Icon(Icons.lightbulb_outline, color: Colors.amber, size: 18),
            label: const Text('Exemplo', style: TextStyle(color: Colors.amber)),
          ),
          IconButton(
            icon: const Icon(Icons.delete_outline_rounded),
            tooltip: 'Limpar Editor',
            onPressed: () => _codeController.clear(),
          ),
          const SizedBox(width: 8),
          ElevatedButton.icon(
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF007ACC),
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
            ),
            onPressed: _isExecuting ? null : _runCode,
            icon: _isExecuting
                ? const SizedBox(
                    width: 16,
                    height: 16,
                    child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                  )
                : const Icon(Icons.play_arrow_rounded, size: 20),
            label: Text(_isExecuting ? 'A Executar...' : 'Executar'),
          ),
          const SizedBox(width: 12),
        ],
      ),
      body: Column(
        children: [
          // Painel do Editor de Código (60% da altura disponível)
          Expanded(
            flex: 6,
            child: Container(
              margin: const EdgeInsets.all(8.0),
              decoration: BoxDecoration(
                color: const Color(0xFF1E1E1E),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: const Color(0xFF333333)),
              ),
              child: Column(
                crossAxisAlignment: CrossAlignment.start,
                children: [
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                    decoration: const BoxDecoration(
                      color: Color(0xFF2D2D2D),
                      borderRadius: BorderRadius.vertical(top: Radius.circular(7)),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: const [
                        Text('script.navoa', style: TextStyle(fontSize: 12, color: Colors.grey)),
                        Icon(Icons.edit_note_rounded, size: 16, color: Colors.grey),
                      ],
                    ),
                  ),
                  Expanded(
                    child: Padding(
                      padding: const EdgeInsets.all(12.0),
                      child: TextField(
                        controller: _codeController,
                        maxLines: null,
                        expands: true,
                        keyboardType: TextInputType.multiline,
                        style: const TextStyle(
                          fontFamily: 'monospace',
                          fontSize: 14,
                          color: Color(0xFFD4D4D4),
                          height: 1.4,
                        ),
                        decoration: const InputDecoration(
                          border: InputBorder.none,
                          hintText: 'Escreva o código Navoa aqui...',
                          hintStyle: TextStyle(color: Colors.grey),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),

          // Painel da Consola de Saída (40% da altura disponível)
          Expanded(
            flex: 4,
            child: Container(
              margin: const EdgeInsets.only(left: 8.0, right: 8.0, bottom: 8.0),
              decoration: BoxDecoration(
                color: const Color(0xFF181818),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: const Color(0xFF333333)),
              ),
              child: Column(
                crossAxisAlignment: CrossAlignment.start,
                children: [
                  // Cabeçalho da Consola
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                    decoration: const BoxDecoration(
                      color: Color(0xFF252526),
                      borderRadius: BorderRadius.vertical(top: Radius.circular(7)),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Row(
                          children: [
                            const Icon(Icons.terminal_rounded, size: 16, color: Colors.grey),
                            const SizedBox(width: 8),
                            const Text('Consola de Saída', style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold)),
                            const SizedBox(width: 12),
                            Container(
                              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                              decoration: BoxDecoration(
                                color: _statusColor.withOpacity(0.2),
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(color: _statusColor, width: 0.8),
                              ),
                              child: Text(
                                _statusMessage,
                                style: TextStyle(fontSize: 10, color: _statusColor, fontWeight: FontWeight.bold),
                              ),
                            ),
                          ],
                        ),
                        Row(
                          children: [
                            IconButton(
                              icon: const Icon(Icons.copy_rounded, size: 16, color: Colors.grey),
                              tooltip: 'Copiar Saída',
                              onPressed: () {
                                Clipboard.setData(ClipboardData(text: _output));
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(content: Text('Saída copiada para a área de transferência!')),
                                );
                              },
                            ),
                            IconButton(
                              icon: const Icon(Icons.clear_all_rounded, size: 16, color: Colors.grey),
                              tooltip: 'Limpar Consola',
                              onPressed: _clearConsole,
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                  // Conteúdo da Consola
                  Expanded(
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.all(12.0),
                      child: SizedBox(
                        width: double.infinity,
                        child: SelectableText(
                          _output,
                          style: const TextStyle(
                            fontFamily: 'monospace',
                            fontSize: 13,
                            color: Color(0xFF4EC9B0),
                            height: 1.3,
                          ),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
