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
        RecruitmentCard(
          destination: "富津岬",
          user: '村岡正憲',
          startDate: "2023/09/10 18:00",
          departure: "東京都新宿区",
          budget: "3000円",
          memberCount: "0/2人",
        ),
        RecruitmentCard(
          destination: "潮風公園",
          user: 'RED鈴木',
          startDate: "2023/09/24 0:00",
          departure: "茨城県つくば市",
          budget: "10000円",
          memberCount: "1/2人",
        ),
        RecruitmentCard(
          destination: "豊洲ぐるり公園",
          user: '高橋優介',
          startDate: "2023/09/10 18:00",
          departure: "東京都江東区",
          budget: "1000円",
          memberCount: "0/1人",
        ),
        RecruitmentCard(
          destination: "武庫川一文字",
          user: '橋本翔大',
          startDate: "2023/09/10 18:00",
          departure: "東京都新宿区",
          budget: "3000円",
          memberCount: "0/2人",
        ),
      ],
    );
  }
}

class RecruitmentCard extends StatelessWidget {
  final String destination;
  final String user;
  final String startDate;
  final String departure;
  final String budget;
  final String memberCount;

  const RecruitmentCard(
      {super.key,
      required this.destination,
      required this.user,
      required this.startDate,
      required this.departure,
      required this.budget,
      required this.memberCount});

  @override
  Widget build(BuildContext context) {
    return Card(
        child: Row(
      children: <Widget>[
        Flexible(
          flex: 2,
          fit: FlexFit.tight,
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 5.0),
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
                      child: FittedBox(
                        fit: BoxFit.scaleDown,
                        child: Text(
                          user,
                        ),
                      ),
                    ))
              ],
            ),
          ),
        ),
        Flexible(
          flex: 6,
          fit: FlexFit.loose,
          child: ListTile(
            title: Column(
              children: [
                FittedBox(fit: BoxFit.scaleDown, child: Text(startDate)),
                Row(
                  children: [
                    const Icon(IconData(0xe3ab, fontFamily: 'MaterialIcons')),
                    Text(destination),
                  ],
                ),
              ],
            ),
            subtitle: FittedBox(
                fit: BoxFit.scaleDown,
                child: Row(
                  children: [
                    const Icon(IconData(0xe1d7, fontFamily: 'MaterialIcons')),
                    Text(departure),
                  ],
                )),
          ),
        ),
        Flexible(
          flex: 4,
          fit: FlexFit.loose,
          child: Column(
            children: [Text(budget), Text(memberCount)],
          ),
        )
      ],
    ));
  }
}
