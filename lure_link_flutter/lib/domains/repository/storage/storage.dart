import 'package:dartz/dartz.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../../value_object/custom_error.dart';

abstract interface class StorageRepository {
  Future<Either<CustomError, String>> readStoredAccessToken();

  Future<Either<CustomError, void>> updateStorageAccessToken(
      String accessToken);
}

// ===============concrete
class Storage extends StorageRepository {
  final FlutterSecureStorage secureStorage = const FlutterSecureStorage();

  @override
  Future<Either<CustomError, String>> readStoredAccessToken() async {
    try {
      String? accessToken =
          await secureStorage.read(key: 'lure_link_access_token');
      if (accessToken == null) {
        return Left(CustomError("cannot get access token from storage"));
      }
      return Right(accessToken!);
    } catch (_) {
      return Left(CustomError("error in secure storage: Read"));
    }
  }

  @override
  Future<Either<CustomError, void>> updateStorageAccessToken(
      String accessToken) async {
    try {
      await secureStorage.write(
          key: 'lure_link_access_token', value: accessToken);
      return const Right(null);
    } catch (_) {
      return Left(CustomError("error in secure storage: Update"));
    }
  }
}
