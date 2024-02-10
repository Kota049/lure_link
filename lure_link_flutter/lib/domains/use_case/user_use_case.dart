import 'package:flutter/cupertino.dart';
import 'package:lure_link_flutter/domains/repository/line_login/login.dart';
import '../repository/api/user.dart';
import '../repository/storage/storage.dart';
import '../value_object/login_status.dart';

class UserUseCase extends ChangeNotifier {
  // state
  LoginStatus loginStatus;
  String? accessToken;

  // depends on
  final LineLoginRepository lineRepo;
  final UserRepository userRepository;
  final StorageRepository storageRepository;

  UserUseCase(this.lineRepo, this.userRepository, this.storageRepository)
      : loginStatus = LoginStatus.undefined,
        accessToken = null;

  Future<void> loginManger() async {
    // storageにトークンが保存されているかのチェック
  }

  // トークンが保存されていれば、それを使って検証、なければ取得
  Future<void> storageTokenManager() async {
    if (loginStatus == LoginStatus.authenticated) {
      return;
    }
  }
}
