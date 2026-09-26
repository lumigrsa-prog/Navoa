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
                'A cidade dorme sob uma camada espessa de nevoeiro que sobe do Tejo. As lâmpadas de vapor de sódio pintam as ruas de âmbar sujo.\n\n'
                'No escritório escuro da Madragoa, o detetive Vicente Palma acende um cigarro cujos anéis de fumo se dissolvem na penumbra do monitor CRT.\n\n'
                'Bem-vindo ao Navoa OS. Aqui, a programação e a investigação noir fundem-se.',
                style: TextStyle(
                  color: Colors.greenAccent,
                  fontSize: 15,
                  height: 1.6,
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
                    'ENTRAR NO ESCRITÓRIO',
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
  final FocusNode _focusNode = FocusNode();
  int currentChapter = 1;
  int stepInChapter = 0;
  bool chapterCompleted = false;

  final List<String> _history = [
    'Navoa OS [Versão 1.0.0 - Terminal Investigativo]',
    '--------------------------------------------------',
    '=== CAPÍTULO 1: O Nevoeiro sobre o Tejo ===',
    'Lisboa, 03:15 AM. O som dos barcos distantes ecoa na amurada.',
    'Vicente Palma olha para o terminal à espera de pistas.',
    '-> Escreva: escrever [alvo: cais_do_sodre]'
  ];

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void _exitToMenu() {
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(builder: (context) => const LanguageSelectScreen()),
    );
  }

  void _handleCommand(String input) {
    setState(() {
      String cmd = input.trim().toLowerCase().replaceAll(RegExp(r'\s+'), ' ');
      _history.add('> $input');

      if (cmd == 'ajuda') {
        _history.add('Comandos: progresso, reiniciar, limpar, sair');
        _focusNode.requestFocus();
        return;
      }
      if (cmd == 'limpar') {
        _history.clear();
        _focusNode.requestFocus();
        return;
      }
      if (cmd == 'sair') {
        _exitToMenu();
        return;
      }

      if (currentChapter == 1) {
        if (stepInChapter == 0 && cmd.startsWith('escrever ')) {
          _history.add('\n[REGISTO] A humidade condensa-se no vidro. O cais está deserto.');
          _history.add('-> Próximo passo: definir sombraco vicente');
          stepInChapter++;
        } else if (stepInChapter == 1 && cmd.startsWith('definir sombraco ')) {
          _history.add('\n[DETETIVE] Vicente assume o caso nas sombras.');
          _history.add('-> Passo final do cap. 1: compilar_caso');
          stepInChapter++;
        } else if (stepInChapter == 2 && cmd == 'compilar_caso') {
          _history.add('\n*** CAPÍTULO 1 CONCLUÍDO ***');
          _history.add('O relatório ganha forma no ecrã verde phosphor.');
          _history.add('-> Digite "proximo" para entrar nas brumas do Capítulo 2.');
          chapterCompleted = true;
          stepInChapter = 0;
        } else if (chapterCompleted && cmd == 'proximo') {
          currentChapter = 2;
          chapterCompleted = false;
          _history.add('\n--------------------------------------------------');
          _history.add('=== CAPÍTULO 2: Sombras na Madragoa ===');
          _history.add('As ruelas estreitas escondem passos apressados e segredos antigos.');
          _history.add('-> Escreva: varrer madragoa');
        } else {
          _history.add('[Noir] O eco responde na sala vazia. Tente o comando correto ou digite "ajuda".');
        }
      } else if (currentChapter == 2) {
        if (stepInChapter == 0 && (cmd == 'varrer madragoa' || cmd.contains('varrer'))) {
          _history.add('\n[INVESTIGAÇÃO] A vassoura de palha agita o pó sob a luz ténue do candeeiro.');
          _history.add('-> Próximo passo: analisar ruelas');
          stepInChapter++;
        } else if (stepInChapter == 1 && (cmd == 'analisar ruelas' || cmd.contains('analisar'))) {
          _history.add('\n*** CAPÍTULO 2 CONCLUÍDO ***');
          _history.add('Encontraste um bilhete amarrotado no chão com o selo da Alfândega.');
          _history.add('-> Digite "proximo" para avançar para o Capítulo 3.');
          chapterCompleted = true;
          stepInChapter = 0;
        } else if (chapterCompleted && cmd == 'proximo') {
          currentChapter = 3;
          chapterCompleted = false;
          _history.add('\n--------------------------------------------------');
          _history.add('=== CAPÍTULO 3: O Cais do Tejo ===');
          _history.add('O vento corta como lâmina fria junto à água escura.');
          _history.add('-> Escreva: interceptar navio');
        } else {
          _history.add('[Noir] A neblina confunde os sentidos. Escreva exatamente o comando pedido.');
        }
      } else if (currentChapter == 3) {
        if (stepInChapter == 0 && cmd.contains('interceptar')) {
          _history.add('\n[CLÍMAX] Os cabos soltam-se. O mistério do Tejo começa a desvendar-se...');
          _history.add('\n*** PARABÉNS! CONCLUÍSTE O ECOSSISTEMA NAVOA COM SUCESSO! ***');
          chapterCompleted = true;
        } else {
          _history.add('[Noir] O tempo esgota-se. Escreva: interceptar navio');
        }
      }
    });
    _controller.clear();
    FocusScope.of(context).requestFocus(_focusNode);
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
        actions: [
          TextButton.icon(
            style: TextButton.styleFrom(foregroundColor: green),
            onPressed: _exitToMenu,
            icon: const Icon(Icons.exit_to_app, size: 16),
            label: const Text('SAIR', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 12)),
          ),
        ],
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
                    focusNode: _focusNode,
                    autofocus: true,
                    style: const TextStyle(color: green, fontFamily: 'monospace'),
                    decoration: const InputDecoration(border: InputBorder.none, hintText: 'digite o comando...'),
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
