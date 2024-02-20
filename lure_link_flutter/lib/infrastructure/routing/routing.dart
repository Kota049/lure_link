import 'package:flutter/cupertino.dart';
import '../../presentation/screens/login.dart';
import '../../presentation/screens/recruitment_list.dart';

Map<String, Widget Function(BuildContext)> routing() {
  return {
    "/": (BuildContext context) => const RecruitmentList(),
    "/login": (BuildContext context) => const LoginScreen(),
  };
}
