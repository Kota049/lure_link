import 'package:dartz/dartz.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';
import '../../entity/user.dart';

abstract interface class UserRepository {
  Future<Either<User, Error>> login(String lineToken);

  Future<Either<void, CustomError>> verifyCurrentToken(
      String applicationAccessToken);

  Future<Either<User, Error>> updateUser(User user);
}
