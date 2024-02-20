import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:flutter_line_sdk/flutter_line_sdk.dart';
import 'package:lure_link_flutter/domains/repository/api/user.dart';
import 'package:lure_link_flutter/domains/repository/line_login/login.dart';
import 'package:lure_link_flutter/domains/repository/storage/storage.dart';
import 'package:lure_link_flutter/domains/use_case/login_manager.dart';
import 'package:provider/provider.dart';
import 'presentation/screens/login.dart';
import 'presentation/screens/recruitment_list.dart';

void main() async {
  await dotenv.load(fileName: ".env");
  final lineChannelId = dotenv.get("LINE_CHANNEL_ID", fallback: "000000000");
  WidgetsFlutterBinding.ensureInitialized();
  LineSDK.instance.setup(lineChannelId).then((_) {
    print('LineSDK Prepared');
  });
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    final lineRepo = LineLogin();
    final userRepository = UserRepository();
    final storageRepository = StorageRepository();
    return ChangeNotifierProvider<UserUseCase>(
      create: (context) =>
          UserUseCase(lineRepo, userRepository, storageRepository),
      child: MaterialApp(
        title: 'Flutter Demo',
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
          useMaterial3: true,
        ),
        initialRoute: "/",
        routes:
        {
          "/":(BuildContext context)=>const RecruitmentList(),
          "/login":(BuildContext context)=>const LoginScreen(),
        },
      ),
    );
  }
}
