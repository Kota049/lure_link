import 'package:dartz/dartz.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';
import '../../entity/user.dart';

abstract interface class UserRepositoryInterface {
  Future<Either<CustomError, User>> login(String lineToken);

  Future<Either<CustomError, User>> loginWithLineToken(
      String applicationAccessToken);

  Future<Either<CustomError, User>> updateUser(User user);
}

// ===============concrete
class UserRepository extends UserRepositoryInterface {
  @override
  Future<Either<CustomError, User>> login(String lineToken) {
    // TODO: implement login
    throw UnimplementedError();
  }

  @override
  Future<Either<CustomError, User>> loginWithLineToken(String applicationAccessToken) {
    // TODO: implement loginWithLineToken
    throw UnimplementedError();
  }

  @override
  Future<Either<CustomError, User>> updateUser(User user) {
    // TODO: implement updateUser
    throw UnimplementedError();
  }
}
