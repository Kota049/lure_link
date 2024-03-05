import 'package:flutter/material.dart';

class CommonNavigationBar extends StatefulWidget{
  const CommonNavigationBar ({Key? key}) : super(key: key);
  @override
  State<CommonNavigationBar> createState() => _CommonNavigationBarState();
}

class _CommonNavigationBarState extends State<CommonNavigationBar>{
  int selectedIndex = 0;

  @override
  Widget build(BuildContext context){
    return BottomNavigationBar(
      items: [
        BottomNavigationBarItem(
          icon:Icon(
            Icons.edit_document,
          ),
          label:'申し込む',
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
          label:'TOP',
        ),
      ],
    );
  }
}