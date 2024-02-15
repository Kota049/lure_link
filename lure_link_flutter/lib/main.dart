import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter_line_sdk/flutter_line_sdk.dart';
import 'presentation/widgets//common_app_bar.dart';
import 'presentation/screens/recruitment_list.dart';

void main() async {
  await dotenv.load(fileName: ".env");
  final line_id = dotenv.get("LINE_CHANNEL_ID", fallback: "000000000");
  WidgetsFlutterBinding.ensureInitialized();
  LineSDK.instance.setup(line_id).then((_) {
    print('LineSDK Prepared');
  });
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
        appBar: CommonAppBar(pageName: "LURELINK"),
        body: const RecruitmentList(),
      ),
    );
  }
}
