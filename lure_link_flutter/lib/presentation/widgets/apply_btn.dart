import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_status.dart';

class ApplyBtn extends StatelessWidget {
  final CarPoolStatus status; // CarPoolStatusのインスタンスを保持する
  ApplyBtn({required this.status});

  @override
  Widget build(BuildContext context){
    switch(status){
      case CarPoolStatus.applying:
        return ElevatedButton(
          onPressed: () {
          },
          child: Text('申し込む'),
        );
      case CarPoolStatus.aplComplete:
        return ElevatedButton(
          onPressed: () {
          },
          child: Text('定員到達'),
        );
      case CarPoolStatus.cancel:
        return ElevatedButton(
          onPressed: () {
          },
          child: Text('募集中止'),
        );
      case CarPoolStatus.finished:
        return ElevatedButton(
          onPressed: () {
          },
          child: Text('受付終了'),
        );
      default:
        return Text('申込不可');
    }
  }
}