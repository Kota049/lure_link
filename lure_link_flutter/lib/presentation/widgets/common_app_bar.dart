import 'package:flutter/material.dart';

class CommonAppBar extends StatelessWidget implements PreferredSizeWidget{
  const CommonAppBar({super.key , required this.pageName});

  final String pageName;

  @override
  Size get preferredSize{
    return Size(double.infinity, 56.0);
  }

  @override
  Widget build(BuildContext cotext){
    return AppBar(
      backgroundColor: Colors.blueAccent,
      title : Text(pageName),
    );
  }
}