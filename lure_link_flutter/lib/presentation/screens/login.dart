import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/use_case/login_manager.dart';
import 'package:provider/provider.dart';

import '../../domains/value_object/custom_error.dart';
import '../widgets/common_app_bar.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  LoginScreenState createState() => LoginScreenState();
}

class LoginScreenState extends State<LoginScreen> {
  bool _isAgreed = false;
  final List<CustomError> _errors = [];

  @override
  Widget build(BuildContext context) {
    final userUseCase = Provider.of<UserUseCase>(context);

    return Scaffold(
      appBar: const CommonAppBar(pageName: "LURELINK"),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              '利用規約',
              style: TextStyle(fontSize: 24.0, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16.0),
            const SingleChildScrollView(
              child: Text(
                  '利用規約本文やでえええええええええええええええええええええええええええええええ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ\nえ',
                  style: TextStyle(fontSize: 16.0),
                  overflow: TextOverflow.ellipsis),
            ),
            const SizedBox(height: 16.0),
            CheckboxListTile(
              title: const Text('同意します'),
              value: _isAgreed,
              onChanged: (value) {
                setState(() {
                  _isAgreed = value!;
                });
              },
            ),
            const SizedBox(height: 16.0),
            ElevatedButton(
              onPressed: _isAgreed
                  ? () async {
                      final res = await userUseCase.loginForLine();
                      res.fold(
                          (l) => {
                                setState(() {
                                  _errors.add(l);
                                })
                              },
                          (_) => {if (context.mounted) Navigator.of(context).pushNamed("/")});
                    }
                  : null,
              child: const Text('ログイン'),
            ),
          ],
        ),
      ),
    );
  }
}
