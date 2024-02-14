import 'package:dartz/dartz.dart';
import 'package:flutter_line_sdk/flutter_line_sdk.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';

abstract interface class LineLoginRepository {
  Future<Either<CustomError, String>> login();
}

// =======================concrete

class LineLogin implements LineLoginRepository {
  @override
  Future<Either<CustomError, String>> login() async {
    try {
      final result = await LineSDK.instance.login();
      final accessToken = result.accessToken.toString();
      return Right(accessToken);
    } catch (_) {
      return Left(CustomError("failed to login to LINE"));
    }
  }
}
