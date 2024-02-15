import 'package:dartz/dartz.dart';
import 'package:flutter/cupertino.dart';
import 'package:lure_link_flutter/domains/entity/user.dart';
import 'package:lure_link_flutter/domains/repository/line_login/login.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';
import '../repository/api/user.dart';
import '../repository/storage/storage.dart';
import '../value_object/login_status.dart';

class UserUseCase extends ChangeNotifier {
  // state
  LoginStatus loginStatus;
  User? user;

  // depends on
  final LineLoginRepository lineRepo;
  final UserRepositoryInterface userRepository;
  final StorageRepositoryInterface storageRepository;

  UserUseCase(this.lineRepo, this.userRepository, this.storageRepository)
      : loginStatus = LoginStatus.undefined,
        user = null;

  Future<void> init() async {
    // ローカルストレージからトークンを取得する
    Either<CustomError, String> storedToken =
        await storageRepository.readStoredAccessToken();
    if (storedToken.isLeft()) {
      updateUnAuthenticated();
      return;
    }

    // トークンが有効かどうかを検証する
    String token = storedToken.toOption().toNullable()!;
    final user = await userRepository.login(token);
    if (user.isLeft()) {
      updateUnAuthenticated();
      return;
    }

    // ステートとストレージのトークンを更新
    final currentUser = user.toOption().toNullable()!;
    final res = await newUpdateAuthenticated(currentUser);
    if (res.isLeft()) {
      updateUnAuthenticated();
      return;
    }
    return;
  }

  Future<Either<CustomError, void>> loginForLine() async {
    // line ログインを行いLINEトークンを取得
    Either<CustomError, String> resLineToken = await lineRepo.login();
    if (resLineToken.isLeft()) {
      updateUnAuthenticated();
      return resLineToken.map((_) => {});
    }
    // lineトークンを元にユーザーを取得
    String lineToken = resLineToken.toOption().toNullable()!;
    final resUser = await userRepository.loginWithLineToken(lineToken);
    if (resUser.isLeft()) {
      updateUnAuthenticated();
      return resUser.map((_) => {});
    }

    // 正常に取得できた場合にはステートを更新
    User currentUser = resUser.toOption().toNullable()!;
    final res = await newUpdateAuthenticated(currentUser);
    if (res.isLeft()) {
      updateUnAuthenticated();
      return res.map((_) => {});
    }
    return const Right(null);
  }

  void updateUnAuthenticated() {
    loginStatus = LoginStatus.unauthenticated;
    user = null;
  }

  Future<Either<CustomError, void>> newUpdateAuthenticated(
      User currentUser) async {
    // ステータスの更新
    loginStatus = LoginStatus.authenticated;
    user = currentUser;

    // アクセストークンの更新
    final res = await storageRepository
        .updateStorageAccessToken(currentUser.applicationToken);
    return res.map((_) => {});
  }
}
