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
  final LineLoginRepository _lineRepo;
  final UserRepositoryInterface _userRepository;
  final StorageRepositoryInterface _storageRepository;

  UserUseCase(this._lineRepo, this._userRepository, this._storageRepository)
      : loginStatus = LoginStatus.undefined,
        user = null;

  bool isAuthenticated() {
    return loginStatus == LoginStatus.authenticated;
  }

  Future<void> init() async {
    // ローカルストレージからトークンを取得する
    Either<CustomError, String> storedToken =
        await _storageRepository.readStoredAccessToken();
    if (storedToken.isLeft()) {
      _updateUnAuthenticated();
      return;
    }

    // トークンが有効かどうかを検証する
    String token = storedToken.toOption().toNullable()!;
    final user = await _userRepository.login(token);
    if (user.isLeft()) {
      _updateUnAuthenticated();
      return;
    }

    // ステートとストレージのトークンを更新
    final currentUser = user.toOption().toNullable()!;
    final res = await _newUpdateAuthenticated(currentUser);
    if (res.isLeft()) {
      _updateUnAuthenticated();
      return;
    }
    return;
  }

  Future<Either<CustomError, void>> loginForLine() async {
    // line ログインを行いLINEトークンを取得
    Either<CustomError, String> resLineToken = await _lineRepo.login();
    if (resLineToken.isLeft()) {
      _updateUnAuthenticated();
      return resLineToken.map((_) => {});
    }
    // lineトークンを元にユーザーを取得
    String lineToken = resLineToken.toOption().toNullable()!;
    final resUser = await _userRepository.loginWithLineToken(lineToken);
    if (resUser.isLeft()) {
      _updateUnAuthenticated();
      return resUser.map((_) => {});
    }

    // 正常に取得できた場合にはステートを更新
    User currentUser = resUser.toOption().toNullable()!;
    final res = await _newUpdateAuthenticated(currentUser);
    if (res.isLeft()) {
      _updateUnAuthenticated();
      return res.map((_) => {});
    }
    return const Right(null);
  }

  void _updateUnAuthenticated() {
    loginStatus = LoginStatus.unauthenticated;
    user = null;
  }

  Future<Either<CustomError, void>> _newUpdateAuthenticated(
      User currentUser) async {
    // ステータスの更新
    loginStatus = LoginStatus.authenticated;
    user = currentUser;

    // アクセストークンの更新
    final res = await _storageRepository
        .updateStorageAccessToken(currentUser.applicationToken);
    return res.map((_) => {});
  }
}
