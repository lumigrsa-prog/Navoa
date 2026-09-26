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

// 1. ECRÃ DE SELEÇÃO DE IDIOMA (6 Idiomas)
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

// 2. ECRÃ DE INTRODUÇÃO COM OPÇÃO DE SALTAR
class IntroScreen extends StatelessWidget {
  final String lang;
  const IntroScreen({super.key, required this.lang});

  String getIntroText() {
    switch (lang) {
      case 'en':
        return 'System booting...\n\nWelcome to Navoa OS.\nA retro noir ecosystem where every chapter teaches you to code a complete investigative script.\n\nType multi-line commands to solve the mystery.';
      case 'es':
        return 'Iniciando sistema...\n\nBienvenido a Navoa OS.\nUn ecosistema retro noir donde cada capítulo te enseña a programar un script de investigación completo.';
      case 'fr':
        return 'Démarrage du système...\n\nBienvenue sur Navoa OS.\nUn écosystème rétro noir interactif.';
      case 'it':
        return 'Avvio del sistema...\n\nBenvenuto in Navoa OS.\nUn ecosistema retro noir investigativo.';
      case 'de':
        return 'Systemstart...\n\nWillkommen bei Navoa OS.\nEin interaktives Retro-Noir-Ökosystem.';
      default:
        return 'A iniciar sistema...\n\nBem-vindo ao Navoa OS.\nUm ecossistema retro noir sobre o Tejo onde cada capítulo guia o utilizador através de múltiplos comandos para criar um programa elaborado.';
    }
  }

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
              Text(
                getIntroText(),
                style: const TextStyle(
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

// 3. TERMINAL CRT COM CAPÍTULOS GUIADOS E MULTI-COMANDOS
class TerminalScreen extends StatefulWidget {
  const TerminalScreen({super.key});

  @override
  State<TerminalScreen> createState() => _TerminalScreenState();
}

class _TerminalScreenState extends State<TerminalScreen> {
  final TextEditingController _controller = TextEditingController();
  
  // Estado do tutorial/capítulos
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

      if (cmd == 'ajuda' || cmd == 'help') {
        _history.add('Comandos globais: \n - progresso: Mostra o estado do capítulo atual\n - reiniciar: Reinicia o capítulo\n - limpar: Limpa o ecrã\n - sair: Volta ao menu de idiomas');
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

      if (cmd == 'progresso') {
        _history.add('Capítulo $currentChapter | Passo atual: ${stepInChapter + 1}/3');
        return;
      }

      if (cmd == 'reiniciar') {
        stepInChapter = 0;
        _history.add('Capítulo reiniciado. Siga os passos indicados.');
        return;
      }

      // LÓGICA DOS CAPÍTULOS COM MULTI-COMANDOS PROGRESSIVOS
      if (currentChapter == 1) {
        if (stepInChapter == 0) {
          if (cmd.startsWith('escrever ')) {
            _history.add('[OK] Alvo registado com sucesso.');
            _history.add('Comando necessário 2: definir sombraco (ex: definir sombraco: vicente)');
            stepInChapter++;
          } else {
            _history.add('[Erro] Use o comando: escrever [alvo: cais_do_sodre]');
          }
        } else if (stepInChapter == 1) {
          if (cmd.startsWith('definir sombraco ')) {
            _history.add('[OK] Detetive atribuído.');
            _history.add('Comando necessário 3: compilar_caso');
            stepInChapter++;
          } else {
            _history.add('[Erro] Use o comando: definir sombraco [nome]');
          }
        } else if (stepInChapter == 2) {
          if (cmd == 'compilar_caso') {
            _history.add('\n*** PROGRAMA CONCLUÍDO COM SUCESSO! ***');
            _history.add('Script gerado: Investigação Cais do Sodre (Atribuído a Vicente).');
            _history.add('-> Pressione Enter ou digite "proximo" para avançar ao Capítulo 2.\n');
            chapterCompleted = true;
            stepInChapter = 0;
          } else {
            _history.add('[Erro] Finalize o programa escrevendo: compilar_caso');
          }
        } else if (chapterCompleted && (cmd == 'proximo' || cmd == '')) {
          currentChapter = 2;
          chapterCompleted = false;
          _history.add('=== CAPÍTULO 2: Sombras na Madragoa ===');
          _history.add('Objectivo: Iniciar rastreio nocturno.');
          _history.add('Comando necessário 1: varrer [zona: madragoa]');
        }
      } else if (currentChapter == 2) {
        if (stepInChapter == 0) {
          if (cmd.startsWith('varrer ')) {
            _history.add('[OK] Zona escaneada.');
            _history.add('Comando necessário 2: escutar [frequencia: 104.2]');
            stepInChapter++;
          } else {
            _history.add('[Erro] Use o comando: varrer [zona: madragoa]');
          }
        } else if (stepInChapter == 1) {
          if (cmd.startsWith('escutar ')) {
            _history.add('[OK] Frequência sintonizada. Apanhou-se uma escuta suspeita.');
            _history.add('Comando necessário 3: gerar_relatorio_final');
            stepInChapter++;
          } else {
            _history.add('[Erro] Use o comando: escutar [frequencia: 104.2]');
          }
        } else if (stepInChapter == 2) {
          if (cmd == 'gerar_relatorio_final') {
            _history.add('\n*** PARABÉNS! PROGRAMA FINAL ELABORADO! ***');
            _history.add('Criaste com sucesso um script completo de investigação noir no Navoa!');
            _history.add('O caso está encerrado.\n');
          } else {
            _history.add('[Erro] Conclua o programa com: gerar_relatorio_final');
          }
        }
      }
    });
    _controller.clear();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.black,
        title: Text('NAVOA TERMINAL (CAP. $currentChapter)', style: const TextStyle(color: Colors.greenAccent, fontSize: 14, letterSpacing: 2)),
        iconTheme: const IconThemeData(color: Colors.greenAccent),
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
                    child: Text(
                      _history[index],
                      style: const TextStyle(color: Colors.greenAccent, fontFamily: 'monospace', fontSize: 14),
                    ),
                  );
                },
              ),
            ),
            const Divider(color: Colors.greenAccent),
            Row(
              children: [
                const Text('> ', style: TextStyle(color: Colors.greenAccent, fontWeight: FontWeight.bold)),
                Expanded(
                  child: TextField(
                    controller: _controller,
                    style: const TextStyle(color: Colors.greenAccent, fontFamily: 'monospace'),
                    decoration: const InputDecoration(
                      border: InputBorder.none,
                      hintText: 'digite o comando do capítulo...',
                      hintStyle: TextStyle(color: Colors.green300),
                    ),
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
