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
        body: const RecruitmentList(),
      ),
    );
  }
}

class RecruitmentList extends StatelessWidget {
  const RecruitmentList({super.key});

  @override
  Widget build(BuildContext context) {
    return const Column(
      children: [
        RecruitmentCard(title: '富津岬', user: '村岡正憲'),
        RecruitmentCard(title: '国府津海岸', user: 'RED鈴木')
      ],
    );
  }
}

class RecruitmentCard extends StatelessWidget {
  final String title;
  final String user;

  const RecruitmentCard({super.key, required this.title, required this.user});

  @override
  Widget build(BuildContext context) {
    return Card(
        child: Row(
      children: <Widget>[
        Flexible(
          flex: 1,
          fit: FlexFit.tight,
          child: Column(
            children: [
              ClipRRect(
                borderRadius: BorderRadius.circular(10), // Image border
                child: AspectRatio(
                    aspectRatio: 1 / 1,
                    child: Image.asset('images/saba.png', fit: BoxFit.cover)),
              ),
              SizedBox(
                  width: double.infinity,
                  child: Center(
                    child: Text(
                      user,
                      style: const TextStyle(fontSize: 8),
                    ),
                  ))
            ],
          ),
        ),
        Flexible(
          flex: 5,
          fit: FlexFit.loose,
          child: ListTile(
            title: Column(
              children: [
                const FittedBox(
                    fit: BoxFit.scaleDown, child: Text("2023/09/10 18:00")),
                Row(
                  children: [
                    const Icon(IconData(0xe3ab, fontFamily: 'MaterialIcons')),
                    Text(title),
                  ],
                ),
              ],
            ),
            subtitle: const FittedBox(
                fit: BoxFit.scaleDown,
                child: Row(
                  children: [
                    Icon(IconData(0xe1d7, fontFamily: 'MaterialIcons')),
                    Text("東京都新宿区"),
                  ],
                )),
          ),
        ),
        const Flexible(
          flex: 3,
          fit: FlexFit.loose,
          child: Column(
            children: [Text("釣行費用"), Text("3000円"), Text("釣行人数"), Text("0/2人")],
          ),
        )
      ],
    ));
  }
}
