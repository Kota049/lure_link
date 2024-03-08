import 'package:flutter/material.dart';

class BottomNavigation extends StatelessWidget{
  final Function(int) updated;
  final int selectedIndex;

  const BottomNavigation(this.updated, this.selectedIndex, {super.key});

  @override
  Widget build(BuildContext context){
    return BottomNavigationBar(
      items: const [
        BottomNavigationBarItem(
          icon:Icon(
            Icons.edit_document,
          ),
          label:'申し込み一覧',
        ),
        BottomNavigationBarItem(
          icon: Icon(
            Icons.home,
          ),
          label:'TOP',
        ),
        BottomNavigationBarItem(
          icon: Icon(
            Icons.directions_car_filled,
          ),
          label:'自分の募集',
        ),
      ],
      currentIndex: selectedIndex,
      selectedItemColor: Colors.amber[800],
      onTap: updated,
    );
  }
}