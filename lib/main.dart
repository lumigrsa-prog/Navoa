import 'package:flutter/material.dart';

void main() {
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
        scaffoldBackgroundColor: Colors.black,
        primaryColor: Colors.greenAccent,
      ),
      home: const LanguageSelectScreen(),
    );
  }
}

class LanguageSelectScreen extends StatelessWidget {
  const LanguageSelectScreen({super.key});

  final List<Map<String, String>> languages = const [
    {'name': 'Português', 'code': 'pt'},
    {'name': 'English', 'code': 'en'},
    {'name': 'Español', 'code': 'es'},
    {'name': 'Français', 'code': 'fr'},
    {'name': 'Italiano', 'code': 'it'},
    {'name': 'Deutsch', 'code': 'de'},
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Text(
                'NAVOA OS',
                style: TextStyle(
                  color: Colors.greenAccent,
                  fontSize: 32,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 3,
                ),
              ),
              const SizedBox(height: 10),
              const Text(
                'SELECIONE O IDIOMA / SELECT LANGUAGE',
                style: TextStyle(color: Colors.grey, fontSize: 12),
              ),
              const SizedBox(height: 40),
              SizedBox(
                width: 300,
                child: ListView.builder(
                  shrinkWrap: true,
                  itemCount: languages.length,
                  itemBuilder: (context, index) {
                    return Padding(
                      padding: const EdgeInsets.symmetric(vertical: 6.0),
                      child: OutlinedButton(
                        style: OutlinedButton.styleFrom(
                          side: const BorderSide(color: Colors.greenAccent),
                          padding: const EdgeInsets.symmetric(vertical: 14),
                        ),
                        onPressed: () {
                          Navigator.pushReplacement(
                            context,
                            MaterialPageRoute(
                              builder: (context) => IntroScreen(lang: languages[index]['code']!),
                            ),
                          );
                        },
                        child: Text(
                          languages[index]['name']!,
                          style: const TextStyle(color: Colors.greenAccent, fontSize: 16),
                        ),
                      ),
                    );
                  },
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class IntroScreen extends StatelessWidget {
  final String lang;
  const IntroScreen({super.key, required this.lang});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text(
                    '// INTRODUÇÃO',
                    style: TextStyle(color: Colors.greenAccent, letterSpacing: 2),
                  ),
                  TextButton(
                    style: TextButton.styleFrom(
                      side: const BorderSide(color: Colors.greenAccent, width: 0.5),
                    ),
                    onPressed: () {
                      Navigator.pushReplacement(
                        context,
                        MaterialPageRoute(builder: (context) => const TerminalScreen()),
                      );
                    },
                    child: const Text(
                      'SALTAR >>',
                      style: TextStyle(color: Colors.greenAccent, fontWeight: FontWeight.bold),
                    ),
                  ),
                ],
              ),
              const Spacer(),
              const Text(
                'A iniciar sistema...\n\nBem-vindo ao Navoa OS.\nUm ecossistema retro noir sobre o Tejo onde cada capítulo guia o utilizador através de múltiplos comandos para criar um programa elaborado.',
                style: TextStyle(
                  color: Colors.greenAccent,
                  fontSize: 16,
                  height: 1.5,
                  fontFamily: 'monospace',
                ),
              ),
              const Spacer(),
              SizedBox(
                width: double.infinity,
                child: ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.greenAccent,
                    foregroundColor: Colors.black,
                    padding: const EdgeInsets.symmetric(vertical: 16),
                  ),
                  onPressed: () {
                    Navigator.pushReplacement(
                      context,
                      MaterialPageRoute(builder: (context) => const TerminalScreen()),
                    );
                  },
                  child: const Text(
                    'INICIAR TERMINAL',
                    style: TextStyle(fontWeight: FontWeight.bold, letterSpacing: 1.5),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class TerminalScreen extends StatefulWidget {
  const TerminalScreen({super.key});

  @override
  State<TerminalScreen> createState() => _TerminalScreenState();
}

class _TerminalScreenState extends State<TerminalScreen> {
  final TextEditingController _controller = TextEditingController();
  int currentChapter = 1;
  int stepInChapter = 0;
  bool chapterCompleted = false;

  final List<String> _history = [
    'Navoa OS [Versão 1.0.0 - Módulo Investigativo]',
    '=== CAPÍTULO 1: O Nevoeiro sobre o Tejo ===',
    'Objectivo: Iniciar o sistema de escuta, declarar a pista principal e compilar o relatório.',
    'Comando necessário 1: escrever [alvo: cais_do_sodre]',
    'Digite o primeiro comando:\n'
  ];

  void _handleCommand(String input) {
    setState(() {
      _history.add('> $input');
      String cmd = input.trim().toLowerCase();

      if (cmd == 'ajuda') {
        _history.add('Comandos: progresso, reiniciar, limpar, sair');
        return;
      }
      if (cmd == 'limpar') {
        _history.clear();
        return;
      }
      if (cmd == 'sair') {
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (context) => const LanguageSelectScreen()),
        );
        return;
      }

      if (currentChapter == 1) {
        if (stepInChapter == 0 && cmd.startsWith('escrever ')) {
          _history.add('[OK] Alvo registado. Comando 2: definir sombraco vicente');
          stepInChapter++;
        } else if (stepInChapter == 1 && cmd.startsWith('definir sombraco ')) {
          _history.add('[OK] Detetive atribuído. Comando 3: compilar_caso');
          stepInChapter++;
        } else if (stepInChapter == 2 && cmd == 'compilar_caso') {
          _history.add('\n*** CAPÍTULO 1 CONCLUÍDO! Digite "proximo" para avançar. ***\n');
          chapterCompleted = true;
          stepInChapter = 0;
        } else if (chapterCompleted && cmd == 'proximo') {
          currentChapter = 2;
          chapterCompleted = false;
          _history.add('=== CAPÍTULO 2: Sombras na Madragoa ===\nComando necessário 1: varrer madragoa');
        } else {
          _history.add('[Erro] Comando inválido para este passo. Digite "ajuda".');
        }
      }
    });
    _controller.clear();
  }

  @override
  Widget build(BuildContext context) {
    const green = Colors.greenAccent;
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.black,
        title: Text('NAVOA (CAP. $currentChapter)', style: const TextStyle(color: green, fontSize: 14)),
        iconTheme: const IconThemeData(color: green),
      ),
      body: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          children: [
            Expanded(
              child: ListView.builder(
                itemCount: _history.length,
                itemBuilder: (context, index) {
                  return Padding(
                    padding: const EdgeInsets.symmetric(vertical: 4.0),
                    child: Text(_history[index], style: const TextStyle(color: green, fontFamily: 'monospace', fontSize: 14)),
                  );
                },
              ),
            ),
            const Divider(color: green),
            Row(
              children: [
                const Text('> ', style: TextStyle(color: green, fontWeight: FontWeight.bold)),
                Expanded(
                  child: TextField(
                    controller: _controller,
                    style: const TextStyle(color: green, fontFamily: 'monospace'),
                    decoration: const InputDecoration(border: InputBorder.none, hintText: 'comando...'),
                    onSubmitted: _handleCommand,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
