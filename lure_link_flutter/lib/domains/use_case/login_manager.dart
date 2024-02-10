import 'package:dartz/dartz.dart';
import 'package:flutter/cupertino.dart';
import 'package:lure_link_flutter/domains/repository/line_login/login.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';
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

  Future<void> init() async {
    await initAccessToken();
    await verifyToken();
  }

  // トークンの初期化(ストレージにトークンが格納されていればそれをステートにぶちこむ)
  Future<void> initAccessToken() async {
    Either<String, CustomError> storedToken =
        await storageRepository.readStoredAccessToken();
    storedToken.fold((l) => {accessToken = l}, (r) => {null});
  }

  // トークンを検証する
  Future<void> verifyToken() async {
    try {
      Either<void, CustomError> res =
          await userRepository.verifyCurrentToken(accessToken!);
      res.fold((l) => loginStatus = LoginStatus.authenticated,
          (r) => loginStatus = LoginStatus.unauthenticated);
    } catch (_) {
      loginStatus = LoginStatus.unauthenticated;
    }
    loginStatus = LoginStatus.unauthenticated;
  }
}
