import 'package:flutter/cupertino.dart';
import 'package:lure_link_flutter/presentation/screens/top_page.dart';
import '../../presentation/screens/login.dart';

Map<String, Widget Function(BuildContext)> routing() {
  return {
    "/": (BuildContext context) => const TopScreen(),
    "/login": (BuildContext context) => const LoginScreen(),
  };
}
