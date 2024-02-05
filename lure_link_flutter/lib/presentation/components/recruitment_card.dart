import 'package:flutter/material.dart';

class RecruitmentCard extends StatelessWidget{
  final String destinationPrefecture;
  final String destinationCity;
  final String destination;
  final String user;
  final String startDate;
  final String departure;
  final String budget;
  final String memberCount;

  const RecruitmentCard(
      {super.key,
        required this.destinationPrefecture,
        required this.destinationCity,
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