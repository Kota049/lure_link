import 'package:dartz/dartz.dart';
import 'package:flutter/cupertino.dart';
import 'package:lure_link_flutter/domains/entity/carpool.dart';
import 'package:lure_link_flutter/domains/repository/api/carpool.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_user_status.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';

class CarPoolUseCase extends ChangeNotifier{
  final CarpoolRepositoryInterface carPoolRepository ;

  CarPoolUseCase(this.carPoolRepository);

  Future<Either<CustomError, List<Carpool>>> index() async {
    final res =  await carPoolRepository.index();
    return res;
  }

  Future<Either<CustomError, CarPoolUserStatus>> canApl(int carpoolId,String userToken) async{
    final res = await carPoolRepository.getCanApl(carpoolId, userToken);
    return res;
  }
}