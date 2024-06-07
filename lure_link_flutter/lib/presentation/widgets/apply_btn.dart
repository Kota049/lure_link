import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_user_status.dart';

class ApplyButton extends StatelessWidget {
  ApplyButton({super.key,required this.carPoolUserStatus});
  CarPoolUserStatus carPoolUserStatus;

  @override
  Widget build(BuildContext context){
    return ElevatedButton(
      onPressed: () {
      },
      child: Text(carPoolUserStatus.btnText),
    );
  }
}