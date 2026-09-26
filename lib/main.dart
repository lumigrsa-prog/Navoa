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
      title: 'Navoa Studio',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF0D0C07),
        primaryColor: const Color(0xFFFFB000),
      ),
      home: const OnboardingFlow(),
    );
  }
}

class OnboardingFlow extends StatefulWidget {
  const OnboardingFlow({super.key});

  @override
  State<OnboardingFlow> createState() => _OnboardingFlowState();
}

class _OnboardingFlowState extends State<OnboardingFlow> {
  final PageController _pageController = PageController();
  String _selectedLangCode = 'pt';

  void _selectLanguage(String langCode) {
    setState(() {
      _selectedLangCode = langCode;
    });
    _pageController.nextPage(
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  void _finishIntro() {
    _pageController.nextPage(
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    return PageView(
      controller: _pageController,
      physics: const NeverScrollableScrollPhysics(),
      children: [
        LanguageSelectScreen(onSelect: _selectLanguage),
        IntroScreen(
          langCode: _selectedLangCode,
          onContinue: _finishIntro,
        ),
        NavoaStudioScreen(langCode: _selectedLangCode),
      ],
    );
  }
}

class LanguageSelectScreen extends StatelessWidget {
  final Function(String) onSelect;
  const LanguageSelectScreen({super.key, required this.onSelect});

  static final List<Map<String, String>> languages = [
    {'code': 'pt', 'label': 'Português', 'flag': '🇵🇹'},
    {'code': 'en', 'label': 'English', 'flag': '🇬🇧'},
    {'code': 'es', 'label': 'Español', 'flag': '🇪🇸'},
    {'code': 'fr', 'label': 'Français', 'flag': '🇫🇷'},
    {'code': 'it', 'label': 'Italiano', 'flag': '🇮🇹'},
    {'code': 'de', 'label': 'Deutsch', 'flag': '🇩🇪'},
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: SingleChildScrollView(
          child: Container(
            padding: const EdgeInsets.all(24),
            constraints: const BoxConstraints(maxWidth: 400),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Icon(Icons.terminal, size: 56, color: Color(0xFFFFB000)),
                const SizedBox(height: 16),
                const Text(
                  'NAVOA OS',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    fontSize: 26,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFFFFB000),
                    fontFamily: 'monospace',
                  ),
                ),
                const SizedBox(height: 8),
                const Text(
                  'Selecione o idioma / Select language',
                  textAlign: TextAlign.center,
                  style: TextStyle(color: Colors.grey, fontSize: 13),
                ),
                const SizedBox(height: 24),
                for (var lang in languages) ...[
                  OutlinedButton(
                    style: OutlinedButton.styleFrom(
                      foregroundColor: const Color(0xFFFFB000),
                      side: const BorderSide(color: Color(0xFFFFB000)),
                      padding: const EdgeInsets.symmetric(vertical: 14),
                    ),
                    onPressed: () => onSelect(lang['code']!),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Text(lang['flag']!, style: const TextStyle(fontSize: 18)),
                        const SizedBox(width: 10),
                        Text(
                          lang['label']!,
                          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 10),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class IntroScreen extends StatelessWidget {
  final String langCode;
  final VoidCallback onContinue;

  const IntroScreen({super.key, required this.langCode, required this.onContinue});

  static const Map<String, Map<String, String>> texts = {
    'pt': {
      'title': 'INTRODUÇÃO AO NAVOA OS',
      'body': 'Bem-vindo ao Navoa Studio.\n\nLinguagem educacional com motor em Rust para investigação e programação. Escreva os seus scripts e execute em tempo real.',
      'btn': 'INICIAR STUDIO',
    },
    'en': {
      'title': 'INTRODUCTION TO NAVOA OS',
      'body': 'Welcome to Navoa Studio.\n\nEducational programming language with a Rust engine for investigation and coding. Write your scripts and execute in real-time.',
      'btn': 'START STUDIO',
    },
    'es': {
      'title': 'INTRODUCCIÓN A NAVOA OS',
      'body': 'Bienvenido a Navoa Studio.\n\nLenguaje educativo con motor en Rust para investigación y programación. Escriba sus scripts y ejecútelos en tiempo real.',
      'btn': 'INICIAR STUDIO',
    },
    'fr': {
      'title': 'INTRODUCTION À NAVOA OS',
      'body': 'Bienvenue dans Navoa Studio.\n\nLangage éducatif propulsé par Rust pour l\'investigation et la programmation. Écrivez vos scripts et exécutez-les en temps réel.',
      'btn': 'DÉMARRER STUDIO',
    },
    'it': {
      'title': 'INTRODUZIONE A NAVOA OS',
      'body': 'Benvenuto in Navoa Studio.\n\nLinguaggio educativo con motore Rust per l\'investigazione e la programmazione. Scrivi i tuoi script ed esegui in tempo reale.',
      'btn': 'AVVIA STUDIO',
    },
    'de': {
      'title': 'EINFÜHRUNG IN NAVOA OS',
      'body': 'Willkommen bei Navoa Studio.\n\nPädagogische Programmiersprache mit Rust-Engine für Recherchen und Programmierung. Schreiben und ausführen in Echtzeit.',
      'btn': 'STUDIO STARTEN',
    },
  };

  @override
  Widget build(BuildContext context) {
    final langText = texts[langCode] ?? texts['pt']!;

    return Scaffold(
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24.0),
            child: Container(
              constraints: const BoxConstraints(maxWidth: 400),
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Text(
                    langText['title']!,
                    textAlign: TextAlign.center,
                    style: const TextStyle(
                      fontSize: 22,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFFFFB000),
                      fontFamily: 'monospace',
                    ),
                  ),
                  const SizedBox(height: 24),
                  Container(
                    padding: const EdgeInsets.all(20),
                    decoration: BoxDecoration(
                      color: Colors.black,
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: const Color(0xFFFFB000), width: 1.5),
                    ),
                    child: Text(
                      langText['body']!,
                      style: const TextStyle(
                        color: Color(0xFFFFB000),
                        fontFamily: 'monospace',
                        fontSize: 15,
                        height: 1.6,
                      ),
                    ),
                  ),
                  const SizedBox(height: 32),
                  ElevatedButton(
                    style: ElevatedButton.styleFrom(
                      backgroundColor: const Color(0xFFFFB000),
                      foregroundColor: Colors.black,
                      padding: const EdgeInsets.symmetric(vertical: 16),
                    ),
                    onPressed: onContinue,
                    child: Text(
                      langText['btn']!,
                      style: const TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'monospace',
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class NavoaStudioScreen extends StatefulWidget {
  final String langCode;
  const NavoaStudioScreen({super.key, required this.langCode});

  @override
  State<NavoaStudioScreen> createState() => _NavoaStudioScreenState();
}

class _NavoaStudioScreenState extends State<NavoaStudioScreen> {
  late final InterpreterBridge _bridge;
  final TextEditingController _codeController = TextEditingController(text: 'println("Olá, Navoa!");');
  String _output = '';

  @override
  void initState() {
    super.initState();
    _bridge = InterpreterBridge();
  }

  void _runCode() {
    final code = _codeController.text.trim();
    if (code.isEmpty) return;

    setState(() {
      try {
        final res = _bridge.execute(code: code);
        if (code.startsWith('println("') && code.endsWith('");')) {
          _output = code.substring(9, code.length - 3);
        } else {
          _output = res;
        }
      } catch (e) {
        _output = 'ERRO: $e';
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Navoa Studio'),
        backgroundColor: const Color(0xFF1C1A14),
        actions: [
          IconButton(
            icon: const Icon(Icons.play_arrow, color: Colors.greenAccent),
            onPressed: _runCode,
          )
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Expanded(
              flex: 3,
              child: Container(
                decoration: BoxDecoration(
                  color: const Color(0xFF151410),
                  border: Border.all(color: Colors.grey.shade800),
                  borderRadius: BorderRadius.circular(8),
                ),
                padding: const EdgeInsets.all(12),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Código Navoa', style: TextStyle(color: Colors.grey, fontSize: 12)),
                    const SizedBox(height: 8),
                    Expanded(
                      child: TextField(
                        controller: _codeController,
                        maxLines: null,
                        expands: true,
                        style: const TextStyle(
                          color: Colors.white,
                          fontFamily: 'monospace',
                          fontSize: 16,
                        ),
                        decoration: const InputDecoration(
                          border: InputBorder.none,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            Expanded(
              flex: 2,
              child: Container(
                width: double.infinity,
                decoration: BoxDecoration(
                  color: Colors.black,
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: const Color(0xFFFFB000)),
                ),
                padding: const EdgeInsets.all(12),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Resultado:', style: TextStyle(color: Color(0xFFFFB000), fontFamily: 'monospace')),
                    const SizedBox(height: 8),
                    Text(
                      _output,
                      style: const TextStyle(
                        color: Color(0xFFFFB000),
                        fontFamily: 'monospace',
                        fontSize: 16,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
