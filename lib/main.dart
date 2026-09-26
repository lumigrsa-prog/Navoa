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
        scaffoldBackgroundColor: const Color(0xFF050806),
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
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
            colors: [Color(0xFF050806), Color(0xFF0A140F)],
          ),
        ),
        child: Center(
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    border: Border.all(color: Colors.greenAccent, width: 2),
                  ),
                  child: const Column(
                    children: [
                      Text(
                        'NAVOA OS',
                        style: TextStyle(
                          color: Colors.greenAccent,
                          fontSize: 32,
                          fontWeight: FontWeight.bold,
                          letterSpacing: 4,
                          fontFamily: 'monospace',
                        ),
                      ),
                      SizedBox(height: 8),
                      Text(
                        '// NEVOEIRO SOBRE O TEJO',
                        style: TextStyle(color: Colors.grey, fontSize: 11, fontFamily: 'monospace'),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 40),
                const Text(
                  'SELECIONE O IDIOMA / SELECT LANGUAGE',
                  style: TextStyle(color: Colors.greenAccent, fontSize: 13, letterSpacing: 1),
                ),
                const SizedBox(height: 24),
                SizedBox(
                  width: 280,
                  child: ListView.builder(
                    shrinkWrap: true,
                    itemCount: languages.length,
                    itemBuilder: (context, index) {
                      return Padding(
                        padding: const EdgeInsets.symmetric(vertical: 6.0),
                        child: OutlinedButton(
                          style: OutlinedButton.styleFrom(
                            side: const BorderSide(color: Colors.greenAccent, width: 1.5),
                            padding: const EdgeInsets.symmetric(vertical: 14),
                            backgroundColor: Colors.black45,
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
                              color: Colors.greenAccent,
                              fontSize: 16,
                              fontWeight: FontWeight.bold,
                              fontFamily: 'monospace',
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
      body: Container(
        decoration: const BoxDecoration(color: Color(0xFF0A0F0D)),
        child: SafeArea(
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      '// DOSSIÊ: INTRODUÇÃO',
                      style: TextStyle(color: Colors.greenAccent, letterSpacing: 2, fontFamily: 'monospace'),
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
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.black.withOpacity(0.6),
                    border: Border.all(color: Colors.greenAccent.withOpacity(0.5)),
                  ),
                  child: const Text(
                    'Lisboa, Madrugada Fria.\n\n'
                    'O nevoeiro do Tejo entra pelas frechas da janela do escritório. As lâmpadas de sódio lá fora projetam sombras compridas sobre a secretária onde descansa o terminal.\n\n'
                    'O detetive Vicente Palma respira fundo o cheiro a tabaco velho e humidade. O caso das sombras no cais exige precisão.\n\n'
                    'Bem-vindo ao ecossistema Navoa. A investigação começa agora.',
                    style: TextStyle(
                      color: Colors.greenAccent,
                      fontSize: 15,
                      height: 1.6,
                      fontFamily: 'monospace',
                    ),
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
                      style: TextStyle(fontWeight: FontWeight.bold, letterSpacing: 1.5, fontFamily: 'monospace'),
                    ),
                  ),
                ),
              ],
            ),
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

  // Estado atual da narrativa e do que o jogador deve fazer
  String narrativeTitle = 'CAPÍTULO 1: O Nevoeiro sobre o Tejo';
  String narrativeText = 'O rio murmura contra as pedras da margem. O som de passos ecoa na rua vazia da Madragoa.\n\nVicente Palma olha para o terminal à espera de pistas.';
  String expectedInstruction = 'Escreva exatamente: escrever [alvo: cais_do_sodre]';
  String feedbackMessage = '';

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

      if (cmd == 'ajuda') {
        feedbackMessage = '[AJUDA] Comandos: progresso, limpar, sair';
        _controller.clear();
        _focusNode.requestFocus();
        return;
      }
      if (cmd == 'limpar') {
        feedbackMessage = '';
        _controller.clear();
        _focusNode.requestFocus();
        return;
      }
      if (cmd == 'sair') {
        _exitToMenu();
        return;
      }

      if (currentChapter == 1) {
        if (stepInChapter == 0 && cmd.startsWith('escrever ')) {
          feedbackMessage = '[OK] A humidade condensa-se no vidro. O cais está deserto.';
          expectedInstruction = 'Escreva exatamente: definir sombraco vicente';
          stepInChapter++;
        } else if (stepInChapter == 1 && cmd.startsWith('definir sombraco ')) {
          feedbackMessage = '[OK] Vicente assume o caso nas sombras do Tejo.';
          expectedInstruction = 'Escreva exatamente: compilar_caso';
          stepInChapter++;
        } else if (stepInChapter == 2 && cmd == 'compilar_caso') {
          currentChapter = 2;
          stepInChapter = 0;
          narrativeTitle = 'CAPÍTULO 2: Sombras na Madragoa';
          narrativeText = 'As ladeiras íngremes da Madragoa cheiram a café tostado e humidade antiga. É aqui que o informador costuma cruzar-se com o perigo.';
          expectedInstruction = 'Escreva exatamente: varrer madragoa';
          feedbackMessage = '*** CAPÍTULO 1 CONCLUÍDO! O caso avança nas brumas de Lisboa. ***';
        } else {
          feedbackMessage = '[Erro] Comando incorreto. Siga a instrução indicada.';
        }
      } else if (currentChapter == 2) {
        if (stepInChapter == 0 && cmd == 'varrer madragoa') {
          feedbackMessage = '[OK] A vassoura de ramos secos levanta poeira antiga debaixo do lampião fundido.';
          expectedInstruction = 'Escreva exatamente: analisar ruelas';
          stepInChapter++;
        } else if (stepInChapter == 1 && cmd == 'analisar ruelas') {
          currentChapter = 3;
          stepInChapter = 0;
          narrativeTitle = 'CAPÍTULO 3: O Cais do Tejo';
          narrativeText = 'As amarras rangem contra o casco do cargueiro ancorado na penumbra. O vento traz o cheiro acre de gasóleo e salitre.';
          expectedInstruction = 'Escreva exatamente: interceptar navio';
          feedbackMessage = '*** CAPÍTULO 2 CONCLUÍDO! Encontrou o bilhete da Alfândega. ***';
        } else {
          feedbackMessage = '[Erro] Escreva exatamente o comando exigido.';
        }
      } else if (currentChapter == 3) {
        if (stepInChapter == 0 && cmd == 'interceptar navio') {
          feedbackMessage = '*** PARABÉNS! O mistério do Tejo foi desvendado nas sombras! ***';
          expectedInstruction = 'Fim da história. Escreva "sair" para voltar ao menu.';
        } else {
          feedbackMessage = '[Erro] Escreva exatamente: interceptar navio';
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
      // AppBar fixa e inamovível no topo com o botão SAIR sempre visível
      appBar: AppBar(
        backgroundColor: const Color(0xFF050806),
        elevation: 4,
        shadowColor: green.withOpacity(0.4),
        title: Row(
          children: [
            Container(width: 8, height: 8, decoration: const BoxDecoration(color: green, shape: BoxShape.circle)),
            const SizedBox(width: 8),
            Text('NAVOA // CAP. $currentChapter', style: const TextStyle(color: green, fontSize: 13, fontFamily: 'monospace', fontWeight: FontWeight.bold)),
          ],
        ),
        automaticallyImplyLeading: false,
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 12.0),
            child: OutlinedButton.icon(
              style: OutlinedButton.styleFrom(
                side: const BorderSide(color: green, width: 1),
                foregroundColor: green,
                backgroundColor: Colors.black,
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              ),
              onPressed: _exitToMenu,
              icon: const Icon(Icons.exit_to_app, size: 14),
              label: const Text('SAIR', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 12, fontFamily: 'monospace')),
            ),
          ),
        ],
      ),
      body: Column(
        children: [
          LinearProgressIndicator(
            value: currentChapter == 1 ? 0.33 : (currentChapter == 2 ? 0.66 : 1.0),
            backgroundColor: Colors.black,
            color: green,
            minHeight: 2,
          ),
          // Janela central dedicada à História e Narrativa (sem acumular texto desnecessário)
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    narrativeTitle,
                    style: const TextStyle(color: Colors.amberAccent, fontSize: 15, fontWeight: FontWeight.bold, fontFamily: 'monospace'),
                  ),
                  const SizedBox(height: 12),
                  Container(
                    width: double.infinity,
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: Colors.black,
                      border: Border.all(color: green.withOpacity(0.4)),
                    ),
                    child: Text(
                      narrativeText,
                      style: const TextStyle(color: green, fontSize: 14, height: 1.5, fontFamily: 'monospace'),
                    ),
                  ),
                  const SizedBox(height: 16),
                  Container(
                    width: double.infinity,
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: const Color(0xFF0A140F),
                      border: Border.all(color: Colors.lightGreenAccent.withOpacity(0.3)),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          '>> AÇÃO NECESSÁRIA:',
                          style: TextStyle(color: Colors.white70, fontSize: 11, fontWeight: FontWeight.bold, fontFamily: 'monospace'),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          expectedInstruction,
                          style: const TextStyle(color: Colors.lightGreenAccent, fontSize: 13, fontFamily: 'monospace', fontWeight: FontWeight.bold),
                        ),
                      ],
                    ),
                  ),
                  if (feedbackMessage.isNotEmpty) ...[
                    const SizedBox(height: 12),
                    Text(
                      feedbackMessage,
                      style: TextStyle(
                        color: feedbackMessage.contains('Erro') ? Colors.redAccent : Colors.amberAccent,
                        fontSize: 12,
                        fontFamily: 'monospace',
                      ),
                    ),
                  ],
                ],
              ),
            ),
          ),
          // Linha de Comandos Fixa na Base
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 8.0),
            decoration: const BoxDecoration(
              color: Color(0xFF0A0F0D),
              border: Border(top: BorderSide(color: Colors.greenAccent, width: 0.5)),
            ),
            child: Row(
              children: [
                const Text('> ', style: TextStyle(color: green, fontWeight: FontWeight.bold, fontFamily: 'monospace')),
                Expanded(
                  child: TextField(
                    controller: _controller,
                    focusNode: _focusNode,
                    autofocus: true,
                    style: const TextStyle(color: green, fontFamily: 'monospace', fontSize: 14),
                    decoration: const InputDecoration(
                      border: InputBorder.none,
                      hintText: 'introduzir comando...',
                      hintStyle: TextStyle(color: Colors.grey, fontSize: 13),
                      isDense: true,
                    ),
                    onSubmitted: _handleCommand,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.send, color: green, size: 18),
                  onPressed: () {
                    if (_controller.text.isNotEmpty) {
                      _handleCommand(_controller.text);
                    }
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
