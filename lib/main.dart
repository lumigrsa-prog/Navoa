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

  static const Map<String, Map<String, String>> localizedText = {
    'pt': {
      'intro': 'LISBOA, 2026.\nNevoeiro sobre o Tejo.\nA cidade esconde segredos nas sombras da noite.',
      'btn': 'INICIAR CONSOLA',
    },
    'en': {
      'intro': 'LISBON, 2026.\nFog over the Tagus.\nThe city hides secrets in the shadows of the night.',
      'btn': 'START CONSOLE',
    },
    'fr': {
      'intro': 'LISBONNE, 2026.\nBrouillard sur le Tage.\nLa ville cache des secrets dans les ombres de la nuit.',
      'btn': 'DÉMARRER LA CONSOLE',
    },
    'it': {
      'intro': 'LISBONA, 2026.\nNebbia sul Tago.\nLa città nasconde segreti nelle ombre della notte.',
      'btn': 'AVVIA CONSOLE',
    },
    'es': {
      'intro': 'LISBOA, 2026.\nNiebla sobre el Tajo.\nLa ciudad esconde secretos en las sombras de la noche.',
      'btn': 'INICIAR CONSOLA',
    },
    'de': {
      'intro': 'LISSABON, 2026.\nNebel über dem Tejo.\nDie Stadt verbirgt Geheimnisse im Schatten der Nacht.',
      'btn': 'KONSOLE STARTEN',
    },
  };

  @override
  Widget build(BuildContext context) {
    final textData = localizedText[lang] ?? localizedText['pt']!;

    return Scaffold(
      appBar: AppBar(
        title: Text('NAVOA // INTRO [${lang.toUpperCase()}]'),
        backgroundColor: Colors.black,
      ),
      body: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              textData['intro']!,
              style: const TextStyle(
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
                  MaterialPageRoute(builder: (context) => TerminalScreen(lang: lang)),
                );
              },
              child: Text(
                textData['btn']!,
                style: const TextStyle(
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
  final String lang;
  const TerminalScreen({super.key, required this.lang});

  @override
  State<TerminalScreen> createState() => _TerminalScreenState();
}

class _TerminalScreenState extends State<TerminalScreen> {
  final TextEditingController _controller = TextEditingController();
  final FocusNode _focusNode = FocusNode();

  late String narrativeText;
  late List<String> logs;

  @override
  void initState() {
    super.initState();
    _setupLanguageContent();
  }

  void _setupLanguageContent() {
    switch (widget.lang) {
      case 'en':
        narrativeText = 'The moorage lines creak against the hull of the cargo ship anchored in the dim light. The wind brings the acrid stench of diesel and saltpeter.';
        logs = [
          'Navoa System v1.0 initialized.',
          'Secure connection established with Lisbon terminal.',
          'Enter a command to interact (e.g.: help, look, examine).'
        ];
        break;
      case 'fr':
        narrativeText = 'Les amarres grincent contre la coque du cargo ancré dans la pénombre. Le vent apporte l\'odeur ocre de mazout et de salpêtre.';
        logs = [
          'Système Navoa v1.0 initialisé.',
          'Connexion sécurisée établie avec le terminal de Lisbonne.',
          'Entrez une commande pour interagir (ex: aide, regarder, examiner).'
        ];
        break;
      case 'it':
        narrativeText = 'Gli ormeggi stridono contro lo scafo della nave da carico ancorata nella penombra. Il vento porta l\'odore acre di gasolio e salnitro.';
        logs = [
          'Sistema Navoa v1.0 inizializzato.',
          'Connessione sicura stabilita con il terminale di Lisbona.',
          'Inserisci un comando per interagire (es: aiuto, guarda, esamina).'
        ];
        break;
      case 'es':
        narrativeText = 'Las amarras crujen contra el casco del carguero anclado en la penumbra. El viento trae el olor acre de diésel y salitre.';
        logs = [
          'Sistema Navoa v1.0 inicializado.',
          'Conexión segura establecida con el terminal de Lisboa.',
          'Ingrese un comando para interactuar (ej: ayuda, mirar, examinar).'
        ];
        break;
      case 'de':
        narrativeText = 'Die Tauwerke knarren am Rumpf des Frachters im Halbdunkel. Der Wind bringt den beißenden Geruch von Diesel und Salpeter.';
        logs = [
          'Navoa System v1.0 initialisiert.',
          'Sichere Verbindung zum Terminal Lissabon hergestellt.',
          'Geben Sie einen Befehl ein (z. B. hilfe, schauen, untersuchen).'
        ];
        break;
      case 'pt':
      default:
        narrativeText = 'As amarras rangem contra o casco do cargueiro ancorado na penumbra. O vento traz o cheiro acre de gasóleo e salitre.';
        logs = [
          'Sistema Navoa v1.0 inicializado.',
          'Ligação segura estabelecida com o terminal de Lisboa.',
          'Insira um comando para interagir (ex: ajudar, olhar, examinar).'
        ];
        break;
    }
  }

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
      logs.add('> $cmd');

      if (cmd == 'ajudar' || cmd == 'help' || cmd == 'aide' || cmd == 'aiuto' || cmd == 'ayuda' || cmd == 'hilfe') {
        logs.add('Comandos disponíveis / Available commands: olhar/look, examinar/examine, sair/exit');
      } else if (cmd == 'olhar' || cmd == 'look' || cmd == 'regarder' || cmd == 'guarda' || cmd == 'mirar' || cmd == 'schauen') {
        logs.add('A neblina cobre as docas. Vê-se um armazém abandonado ao fundo.');
      } else if (cmd == 'sair' || cmd == 'exit' || cmd == 'quitter') {
        _exitToMenu();
        return;
      } else {
        logs.add('Comando desconhecido / Unknown command.');
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
                      hintText: 'comando...',
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
