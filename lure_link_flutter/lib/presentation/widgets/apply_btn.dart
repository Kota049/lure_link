import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_user_status.dart';

class ApplyBtn extends StatelessWidget {
  const ApplyBtn({super.key});

  @override
  Widget build(BuildContext context){
    CarPoolUserStatus status = CarPoolUserStatus.applying;

    return ElevatedButton(
      onPressed: () {
      },
      child: Text(status.btnText),
    );
  }
}