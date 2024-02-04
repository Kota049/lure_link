import 'package:dartz/dartz.dart';
import '../../entity/user.dart';

abstract interface class UserRepository {
  Future<Either<User, Error>> login();

  Future<Either<User, Error>> updateUser();
}
