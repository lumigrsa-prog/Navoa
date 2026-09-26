import 'package:flutter/material.dart';

void main() {
  runApp(const NavoaApp());
}

class NavoaApp extends StatelessWidget {
  const NavoaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Navoa - Noir Detective',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        brightness: Brightness.dark,
        scaffoldBackgroundColor: Colors.black,
        primarySwatch: Colors.grey,
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
    {'name': 'Français', 'code': 'fr'},
    {'name': 'Italiano', 'code': 'it'},
    {'name': 'Español', 'code': 'es'},
    {'name': 'Deutsch', 'code': 'de'},
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NAVOA // SELECT LANGUAGE'),
        backgroundColor: Colors.black,
        centerTitle: true,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Padding(
              padding: EdgeInsets.symmetric(vertical: 20.0),
              child: Text(
                'SELECIONE O IDIOMA DO SISTEMA',
                style: TextStyle(
                  color: Colors.greenAccent,
                  fontFamily: 'Courier',
                  fontSize: 16,
                  letterSpacing: 2.0,
                ),
                textAlign: TextAlign.center,
              ),
            ),
            Expanded(
              child: ListView.builder(
                itemCount: languages.length,
                itemBuilder: (context, index) {
                  return Container(
                    margin: const EdgeInsets.symmetric(vertical: 8.0),
                    child: ElevatedButton(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.grey[900],
                        foregroundColor: Colors.greenAccent,
                        side: const BorderSide(color: Colors.greenAccent, width: 1),
                        padding: const EdgeInsets.symmetric(vertical: 16.0),
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
                        style: const TextStyle(
                          fontFamily: 'Courier',
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),
          ],
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
      appBar: AppBar(
        title: const Text('NAVOA // INTRO'),
        backgroundColor: Colors.black,
      ),
      body: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'LISBOA, 2026.\nNevoeiro sobre o Tejo.\nA cidade esconde segredos nas sombras da noite.',
              style: TextStyle(
                color: Colors.greenAccent,
                fontFamily: 'Courier',
                fontSize: 16,
                height: 1.5,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 40),
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.greenAccent,
                foregroundColor: Colors.black,
                padding: const EdgeInsets.symmetric(vertical: 16.0),
              ),
              onPressed: () {
                Navigator.pushReplacement(
                  context,
                  MaterialPageRoute(builder: (context) => const TerminalScreen()),
                );
              },
              child: const Text(
                'INICIAR CONSOLA',
                style: TextStyle(
                  fontFamily: 'Courier',
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ],
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
  String narrativeText = 'As amarras rangem contra o casco do cargueiro ancorado na penumbra. O vento traz o cheiro acre de gasóleo e salitre.';
  
  final List<String> logs = [
    'Sistema Navoa v1.0 inicializado.',
    'Ligação segura estabelecida com o terminal de Lisboa.',
    'Insira um comando para interagir (ex: ajudar, olhar, examinar).'
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
      MaterialPageRoute(builder: (context) => const TerminalScreen()),
    );
  }

  void _handleCommand(String input) {
    setState(() {
      String cmd = input.trim().toLowerCase().replaceAll(RegExp(r'\s+'), ' ');
      logs.add('> $cmd');
      
      if (cmd == 'ajudar' || cmd == 'help') {
        logs.add('Comandos disponíveis: olhar, examinar, inventario, sair');
      } else if (cmd == 'olhar') {
        logs.add('A neblina cobre as docas. Vê-se um armazém abandonado ao fundo.');
      } else if (cmd == 'sair') {
        _exitToMenu();
        return;
      } else {
        logs.add('Comando desconhecido. Escreva "ajudar" para ver os comandos.');
      }
      _controller.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('NAVOA // CONSOLA CRT', style: TextStyle(fontFamily: 'Courier', color: Colors.greenAccent)),
        backgroundColor: Colors.black,
        actions: [
          TextButton(
            onPressed: _exitToMenu,
            child: const Text(
              'SAIR',
              style: TextStyle(color: Colors.redAccent, fontFamily: 'Courier', fontWeight: FontWeight.bold),
            ),
          ),
        ],
      ),
      body: Column(
        children: [
          Container(
            width: double.infinity,
            padding: const EdgeInsets.all(12.0),
            color: Colors.grey[900],
            child: Text(
              narrativeText,
              style: const TextStyle(
                color: Colors.greenAccent,
                fontFamily: 'Courier',
                fontSize: 14,
              ),
            ),
          ),
          const Divider(color: Colors.greenAccent, height: 1),
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.all(8.0),
              itemCount: logs.length,
              itemBuilder: (context, index) {
                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 2.0),
                  child: Text(
                    logs[index],
                    style: const TextStyle(color: Colors.greenAccent, fontFamily: 'Courier', fontSize: 13),
                  ),
                );
              },
            ),
          ),
          Container(
            padding: const EdgeInsets.all(8.0),
            color: Colors.black,
            child: Row(
              children: [
                const Text('> ', style: TextStyle(color: Colors.greenAccent, fontFamily: 'Courier', fontWeight: FontWeight.bold)),
                Expanded(
                  child: TextField(
                    controller: _controller,
                    focusNode: _focusNode,
                    style: const TextStyle(color: Colors.greenAccent, fontFamily: 'Courier'),
                    decoration: const InputDecoration(
                      border: InputBorder.none,
                      hintText: 'introduzir comando...',
                      hintStyle: TextStyle(color: Colors.grey),
                    ),
                    onSubmitted: _handleCommand,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.send, color: Colors.greenAccent),
                  onPressed: () => _handleCommand(_controller.text),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
