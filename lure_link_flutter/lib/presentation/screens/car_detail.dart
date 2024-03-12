import 'package:flutter/material.dart';
import '../widgets/app_bar_with_login.dart';

class CarDetail extends StatelessWidget{
  @override
  Widget build(BuildContext context){
    return Scaffold(
      appBar: const AppBarWithLoginButton(pageName: "募集詳細"),
      body: Text("募集詳細"),
    );
  }
}