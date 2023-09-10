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
          fit:FlexFit.loose,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(20), // Image border
            child: SizedBox.fromSize(
              size: const Size.fromRadius(30), // Image radius
              child: Image.asset('images/saba.png', fit: BoxFit.cover),
            ),
          ),
        ),
        Flexible(
          flex: 5,
          fit: FlexFit.loose,
          child: ListTile(
            subtitle: Text(user),
            title: Text(title),
          ),
        ),
        const Flexible(
          flex: 3,
          fit: FlexFit.loose,
          child: Column(
            children: [
              Text("釣行費用"),
              Text("3000円"),
              Text("釣行人数"),
              Text("0/2人")
            ],
          ),
          
        )
      ],
    ));
  }
}
