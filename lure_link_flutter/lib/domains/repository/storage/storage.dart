import 'package:dartz/dartz.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../../value_object/custom_error.dart';

abstract interface class StorageRepository {
  Future<Either<String, CustomError>> readStoredAccessToken();

  Future<Either<void, CustomError>> updateStorageAccessToken(
      String accessToken);
}

// ===============concrete
class Storage extends StorageRepository {
  final FlutterSecureStorage secureStorage = const FlutterSecureStorage();

  @override
  Future<Either<String, CustomError>> readStoredAccessToken() async {
    try {
      String? accessToken =
          await secureStorage.read(key: 'lure_link_access_token');
      if (accessToken == null) {
        return Right(CustomError("cannot get access token from storage"));
      }
      return Left(accessToken!);
    } catch (_) {
      return Right(CustomError("error in secure storage: Read"));
    }
  }

  @override
  Future<Either<void, CustomError>> updateStorageAccessToken(
      String accessToken) async {
    try {
      await secureStorage.write(
          key: 'lure_link_access_token', value: accessToken);
      return Left(null);
    } catch (_) {
      return Right(CustomError("error in secure storage: Update"));
    }
  }
}
