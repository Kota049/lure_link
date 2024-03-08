import 'package:flutter/material.dart';
import 'package:lure_link_flutter/presentation/screens/car_pool.dart';

import '../widgets/app_bar_with_login.dart';
import '../widgets/bottom_navigation.dart';

class TopScreen extends StatefulWidget {
  const TopScreen({Key? key}) : super(key: key);

  @override
  State<TopScreen> createState() => _TopScreenState();
}

class _TopScreenState extends State<TopScreen> {
  int selectedScreenNum = 1;

  void changeScreen(int index) {
    setState(() {
      selectedScreenNum = index;
    });
  }

  static const List<Widget> _screens = [
    Text("申し込み一覧画面"),
    CarPoolScreen(),
    Text("自分の募集画面")
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const AppBarWithLoginButton(pageName: "LURELINK"),
      body: _screens.elementAt(selectedScreenNum),
      bottomNavigationBar: BottomNavigation(changeScreen, selectedScreenNum),
    );
  }
}
