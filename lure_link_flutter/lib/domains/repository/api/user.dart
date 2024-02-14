import 'package:dartz/dartz.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';
import '../../entity/user.dart';

abstract interface class UserRepository {
  Future<Either<CustomError, User>> login(String lineToken);

  Future<Either<CustomError, User>> loginWithLineToken(
      String applicationAccessToken);

  Future<Either<CustomError, User>> updateUser(User user);
}
