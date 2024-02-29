import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../domains/use_case/user_use_case.dart';

class NotLoginAppBar extends StatelessWidget implements PreferredSizeWidget{
  const NotLoginAppBar({super.key , required this.pageName});

  final String pageName;

  @override
  Size get preferredSize{
    return Size(double.infinity, 56.0);
  }

  @override
  Widget build(BuildContext context){
    final userUseCase = Provider.of<UserUseCase>(context);
    return AppBar(
      backgroundColor: Colors.blueAccent,
      title : Text(pageName),
      actions : [
        !userUseCase.isAuthenticated()
            ? TextButton(
            onPressed: () async {
              Navigator.of(context).pushNamed('/login');
            },
            child: const Text("LINE LOGIN"))
            : const SizedBox(),
      ]
    );
  }
}