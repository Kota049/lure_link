import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: Scaffold(
        appBar: AppBar(
            backgroundColor: Colors.blueAccent, title: const Text("LURELINK")),
        body: const ConsentPage(),
      ),
    );
  }
}

class ConsentPage extends StatefulWidget {
  const ConsentPage({super.key});

  @override
  _ConsentPage createState() => _ConsentPage();
}

class _ConsentPage extends State<ConsentPage> {
  bool _checkBox = false;
  void _checkConsent(checkBox){
    setState(() {
      _checkBox = checkBox;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Text(
          '同意画面'
        ),
        Text(
          '以下を読んで同意してください。'
        ),
        Text(
          'テキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキストテキスト'
        ),
        CheckboxListTile(
          value:_checkBox,
          controlAffinity: ListTileControlAffinity.leading,
          title: Text(
            '同意する',
          ),
          onChanged: _checkConsent,
        ),
        ElevatedButton(
          child:Text('送信'),
          onPressed: null,
        ),
      ],
    );
  }
}



