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
          '以下を読んで同意してください。\n'
        ),
        Text(
          '本サービスのご利用および決済処理において、お客様のメールアドレスが使用される場合がございます。メールアドレスは、お支払いの確認や重要なお知らせの送信に利用されます。\nお客様は、以下の内容に同意されたものとみなされます。\n\n1. メールアドレスの使用目的:お客様のメールアドレスは、決済処理においてお支払い確認やご利用内容に関する通知を受信するために使用されます。\n\n2. 重要な情報の送信:お客様が行った取引に関する重要な情報や更新、変更に関する通知が、メールアドレスを通じてお客様に送信される可能性があります。\n\n3. セキュリティの確保:お客様の情報は適切に保護され、第三者に漏洩させません。詳細については、プライバシーポリシーをご確認ください。\n\n上記内容に同意いただける場合のみ、本サービスをご利用いただけます。なお、本内容はサービスの性質や法令の変更により変更される可能性があります。\nご不明点やご質問がございましたら、お気軽にお問い合わせください。'
        ),
        CheckboxListTile(
          value:_checkBox,
          controlAffinity: ListTileControlAffinity.leading,
          title: Text(
            '上記内容に同意する',
          ),
          onChanged: _checkConsent,
        ),
        ElevatedButton(
          child:Text('送信'),
          onPressed: _checkBox == false ? null :(){}
        ),
      ],
    );
  }
}



