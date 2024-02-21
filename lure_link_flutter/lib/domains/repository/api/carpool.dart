import 'package:dartz/dartz.dart';
import 'package:lure_link_flutter/domains/entity/Carpool.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';

abstract interface class CarpoolRepositoryInterface {
  Future<Either<CustomError,List<Carpool>>>index();
  Future<Either<CustomError,Carpool>>getCarpool();
}

// ===============concrete
class CarPoolRepository extends CarpoolRepositoryInterface {
  @override
  Future<Either<CustomError, Carpool>> getCarpool() {
    // TODO: implement getCarpool
    throw UnimplementedError();
  }

  @override
  Future<Either<CustomError, List<Carpool>>> index() {
    // TODO: implement index
    throw UnimplementedError();
  }
}
