import 'package:dartz/dartz.dart';
import '../../entity/user.dart';

abstract interface class UserRepository {
  Future<Either<User, Error>> login(String lineToken);

  Future<Either<User, Error>> updateUser(User user);
}
