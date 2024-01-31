import 'package:flutter/material.dart';
import 'presentation/components/common_app_bar.dart';
import 'presentation/screens/recruitment_list.dart';

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
        appBar: CommonAppBar(pageName:"LURELINK"),
        body: const RecruitmentList(),
      ),
    );
  }
}

