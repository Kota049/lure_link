import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_user_status.dart';

class ApplyButton extends StatelessWidget {
  ApplyButton({super.key,required this.carPoolUserStatus});
  CarPoolUserStatus carPoolUserStatus;



  @override
  Widget build(BuildContext context){
    return ElevatedButton(
      onPressed:switch(carPoolUserStatus){
        CarPoolUserStatus.applying => () {Navigator.of(context).pushNamed("/applying");},
        CarPoolUserStatus.canApl => () {Navigator.of(context).pushNamed("/applying/new");},
        CarPoolUserStatus.cannotApl => null,
        CarPoolUserStatus.owner => () {Navigator.of(context).pushNamed("/");},
        CarPoolUserStatus.notLogin => () {Navigator.of(context).pushNamed("/login");},
        CarPoolUserStatus.undefined => null,
      },
      child: Text(carPoolUserStatus.btnText),
    );
  }
}

