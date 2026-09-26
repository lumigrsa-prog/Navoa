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
      title: 'Navoa OS - Retro Noir',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF0D0C07),
        primaryColor: const Color(0xFFFFB000),
      ),
      home: const MainNavigationFlow(),
    );
  }
}

class MainNavigationFlow extends StatefulWidget {
  const MainNavigationFlow({super.key});

  @override
  State<MainNavigationFlow> createState() => _MainNavigationFlowState();
}

class _MainNavigationFlowState extends State<MainNavigationFlow> {
  int _currentStep = 0; // 0: Idiomas, 1: Introdução Gráfica, 2: Terminal
  String _selectedLang = 'pt';

  void _onLanguageSelected(String langCode) {
    setState(() {
      _selectedLang = langCode;
      _currentStep = 1; // Avança obrigatoriamente para a Introdução Gráfica
    });
  }

  void _onStartStudio() {
    setState(() {
      _currentStep = 2; // Avança para o Terminal/Studio
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_currentStep == 0) {
      return LanguageSelectScreen(onSelect: _onLanguageSelected);
    } else if (_currentStep == 1) {
      return IntroGraphicalScreen(
        langCode: _selectedLang,
        onContinue: _onStartStudio,
      );
    } else {
      return NavoaTerminalScreen(langCode: _selectedLang);
    }
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
          padding: const EdgeInsets.all(24),
          child: Container(
            constraints: const BoxConstraints(maxWidth: 400),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Icon(Icons.terminal, size: 64, color: Color(0xFFFFB000)),
                const SizedBox(height: 16),
                const Text(
                  'NAVOA OS',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFFFFB000),
                    fontFamily: 'monospace',
                    letterSpacing: 2,
                  ),
                ),
                const SizedBox(height: 8),
                const Text(
                  'SELECIONE O IDIOMA / SELECT LANGUAGE',
                  textAlign: TextAlign.center,
                  style: TextStyle(color: Colors.grey, fontSize: 12, fontFamily: 'monospace'),
                ),
                const SizedBox(height: 24),
                for (var lang in languages) ...[
                  OutlinedButton(
                    style: OutlinedButton.styleFrom(
                      foregroundColor: const Color(0xFFFFB000),
                      side: const BorderSide(color: Color(0xFFFFB000), width: 1.5),
                      padding: const EdgeInsets.symmetric(vertical: 14),
                    ),
                    onPressed: () => onSelect(lang['code']!),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Text(lang['flag']!, style: const TextStyle(fontSize: 20)),
                        const SizedBox(width: 12),
                        Text(
                          lang['label']!,
                          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold, fontFamily: 'monospace'),
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

class IntroGraphicalScreen extends StatelessWidget {
  final String langCode;
  final VoidCallback onContinue;

  const IntroGraphicalScreen({super.key, required this.langCode, required this.onContinue});

  static const Map<String, Map<String, String>> texts = {
    'pt': {
      'title': '⚡ NAVOA OS // INTRODUÇÃO',
      'sub': 'SISTEMA OPERACIONAL DE INVESTIGAÇÃO',
      'body': 'Bem-vindo ao ambiente de desenvolvimento Navoa.\n\n• Linguagem nativa multilíngue (PT, EN, ES, FR, IT, DE)\n• Execução de código e variáveis em tempo real\n• Motor de alta performance escrito em Rust\n\nInstruções suportadas em Português:\n> escrever("mensagem") ou imprimir("mensagem")\n> variavel = valor\n> inspecionar quarto',
      'btn': 'INICIAR TERMINAL CRT',
    },
    'en': {
      'title': '⚡ NAVOA OS // INTRODUCTION',
      'sub': 'INVESTIGATION OPERATING SYSTEM',
      'body': 'Welcome to the Navoa development environment.\n\n• Multilingual native language support\n• Real-time code and variable execution\n• High-performance engine built with Rust\n\nCommands supported:\n> print("message")\n> variable = value\n> inspect room',
      'btn': 'START CRT TERMINAL',
    },
  };

  @override
  Widget build(BuildContext context) {
    final langText = texts[langCode] ?? texts['pt']!;

    return Scaffold(
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(20.0),
            child: Container(
              constraints: const BoxConstraints(maxWidth: 420),
              decoration: BoxDecoration(
                color: const Color(0xFF12110C),
                border: Border.all(color: const Color(0xFFFFB000), width: 2),
                borderRadius: BorderRadius.circular(12),
                boxShadow: [
                  BoxShadow(
                    color: const Color(0xFFFFB000).withOpacity(0.15),
                    blurRadius: 12,
                    spreadRadius: 2,
                  )
                ],
              ),
              padding: const EdgeInsets.all(20),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Text(
                    langText['title']!,
                    textAlign: TextAlign.center,
                    style: const TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFFFFB000),
                      fontFamily: 'monospace',
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    langText['sub']!,
                    textAlign: TextAlign.center,
                    style: const TextStyle(color: Colors.grey, fontSize: 11, fontFamily: 'monospace'),
                  ),
                  const Divider(color: Color(0xFFFFB000), height: 24, thickness: 1),
                  Container(
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: Colors.black,
                      borderRadius: BorderRadius.circular(6),
                      border: Border.all(color: const Color(0xFFFFB000).withOpacity(0.5)),
                    ),
                    child: Text(
                      langText['body']!,
                      style: const TextStyle(
                        color: Color(0xFFFFB000),
                        fontFamily: 'monospace',
                        fontSize: 13,
                        height: 1.5,
                      ),
                    ),
                  ),
                  const SizedBox(height: 24),
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
                        fontSize: 15,
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

class NavoaTerminalScreen extends StatefulWidget {
  final String langCode;
  const NavoaTerminalScreen({super.key, required this.langCode});

  @override
  State<NavoaTerminalScreen> createState() => _NavoaTerminalScreenState();
}

class _NavoaTerminalScreenState extends State<NavoaTerminalScreen> {
  final TextEditingController _controller = TextEditingController();
  final List<String> _history = [
    '> SISTEMA OPERACIONAL NAVOA // TERMINAL CRT',
    '> Suporte a comandos em Português ativo.',
    '> Digita \'inspecionar quarto\' ou \'escrever("olá")\' para testar.'
  ];
  final Map<String, String> _variables = {};

  void _executeCommand() {
    final cmd = _controller.text.trim();
    if (cmd.isEmpty) return;

    setState(() {
      _history.add('> $cmd');
      _controller.clear();

      // Suporte a escrever("...") / imprimir("...")
      if ((cmd.startsWith('escrever(') || cmd.startsWith('imprimir(') || cmd.startsWith('println(')) && cmd.endsWith(')')) {
        final content = cmd.substring(cmd.indexOf('(') + 1, cmd.lastIndexOf(')')).replaceAll('"', '').replaceAll("'", '');
        if (_variables.containsKey(content)) {
          _history.add('📣 ${_variables[content]}');
        } else {
          _history.add('📣 $content');
        }
      } 
      // Suporte a inspecionar <algo>
      else if (cmd.startsWith('inspecionar ')) {
        final target = cmd.substring(12).trim();
        if (target == 'quarto') {
          _history.add('🔍 PISTA: O quadro elétrico precisa de energia. Cria a variável \'energia=100\'.');
        } else {
          _history.add('🔍 PISTA: Nada de especial encontrado em \'$target\'.');
        }
      } 
      // Suporte a atribuição de variáveis (ex: energia=100)
      else if (cmd.contains('=')) {
        final parts = cmd.split('=');
        final varName = parts[0].trim();
        final varVal = parts[1].trim();
        _variables[varName] = varVal;
        _history.add('📦 Variável \'$varName\' definida como $varVal!');
        if (varName == 'energia' && varVal == '100') {
          _history.add('💡 A energia flui. Os sistemas do quarto ligaram-se.');
        }
      } 
      // Caso comando não reconhecido
      else {
        _history.add('❓ Comando não reconhecido. Tenta \'escrever("texto")\', \'inspecionar quarto\' ou \'variavel=valor\'.');
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Navoa OS - Retro Noir', style: TextStyle(fontFamily: 'monospace', fontSize: 16)),
        backgroundColor: const Color(0xFF181610),
        elevation: 0,
      ),
      body: Padding(
        padding: const EdgeInsets.all(12.0),
        child: Column(
          children: [
            Expanded(
              child: Container(
                width: double.infinity,
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Colors.black,
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: const Color(0xFFFFB000), width: 1.5),
                ),
                child: ListView.builder(
                  itemCount: _history.length,
                  itemBuilder: (context, index) {
                    return Padding(
                      padding: const EdgeInsets.only(bottom: 6.0),
                      style: const TextStyle(
                        color: Color(0xFFFFB000),
                        fontFamily: 'monospace',
                        fontSize: 14,
                        height: 1.4,
                      ),
                      child: Text(_history[index]),
                    );
                  },
                ),
              ),
            ),
            const SizedBox(height: 10),
            Row(
              children: [
                Expanded(
                  child: Container(
                    padding: const EdgeInsets.horizontal(12),
                    decoration: BoxDecoration(
                      color: const Color(0xFF151410),
                      border: Border.all(color: const Color(0xFFFFB000)),
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: TextField(
                      controller: _controller,
                      style: const TextStyle(color: Color(0xFFFFB000), fontFamily: 'monospace'),
                      decoration: const InputDecoration(
                        hintText: 'digita um comando...',
                        hintStyle: TextStyle(color: Colors.grey, fontFamily: 'monospace'),
                        border: InputBorder.none,
                      ),
                      onSubmitted: (_) => _executeCommand(),
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFFFFB000),
                    foregroundColor: Colors.black,
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
                  ),
                  onPressed: _executeCommand,
                  child: const Text('EXECUTAR', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'monospace')),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
