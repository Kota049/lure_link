import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/entity/carpool.dart';

class CarPoolSummary extends StatelessWidget {
  final Carpool carpool;

  const CarPoolSummary({super.key, required this.carpool});

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
                          carpool.organizerNickName,
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
                FittedBox(
                    fit: BoxFit.scaleDown,
                    child: Text(carpool.startTime.toString())),
                Row(
                  children: [
                    const Icon(IconData(0xe3ab, fontFamily: 'MaterialIcons')),
                    Text(carpool.destinationPoint),
                  ],
                ),
              ],
            ),
            subtitle: FittedBox(
                fit: BoxFit.scaleDown,
                child: Row(
                  children: [
                    const Icon(IconData(0xe1d7, fontFamily: 'MaterialIcons')),
                    Text(carpool.destinationPoint),
                  ],
                )),
          ),
        ),
        Flexible(
          flex: 4,
          fit: FlexFit.loose,
          child: Column(
            children: [
              Text(carpool.budget.toString()),
              Text(carpool.maxParticipant.toString())
            ],
          ),
        )
      ],
    ));
  }
}
