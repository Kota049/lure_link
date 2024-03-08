import 'package:dartz/dartz.dart';
import 'package:lure_link_flutter/domains/entity/carpool.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_status.dart';
import 'package:lure_link_flutter/domains/value_object/custom_error.dart';

abstract interface class CarpoolRepositoryInterface {
  Future<Either<CustomError,List<Carpool>>>index();
  Future<Either<CustomError,Carpool>>get();
}

// ===============concrete
class CarPoolRepository extends CarpoolRepositoryInterface {
  @override
  Future<Either<CustomError, Carpool>> get() {
    // TODO: implement getCarpool
    throw UnimplementedError();
  }

  @override
  Future<Either<CustomError, List<Carpool>>> index() {
    // TODO: implement index
    throw UnimplementedError();
  }
}


// ================mock for test
class MockCarPoolReopsitory extends CarpoolRepositoryInterface{
  @override
  Future<Either<CustomError, Carpool>> get() {
    // TODO: implement get
    throw UnimplementedError();
  }

  @override
  Future<Either<CustomError, List<Carpool>>> index() async {
    Either<CustomError,List<Carpool>> res = right([
      Carpool(1, 1, "テスト太郎1", DateTime(2024,1,1,1,0,0,0),DateTime(2024,1,2,1,0,0,0) , DateTime(2024,1,3,1,0,0,0), "川越駅東口", "川越市", "埼玉県","富津岬" , "富津市", "千葉県", 3000, 2, 0,  CarPoolStatus.applying,"アクアラインを使うので、高速代がかかります"),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "ファミリーマート東中島店", "大阪市東淀川区", "大阪府","武庫川一文字" , "尼崎市", "兵庫県", 2000, 2, 0, CarPoolStatus.applying, "渡船代金は別途支払いが必要です"),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
      Carpool(1, 1, "テスト花子2", DateTime(2024,1,2,1,0,0,0),DateTime(2024,1,3,1,0,0,0) , DateTime(2024,1,4,1,0,0,0), "東京競馬場", "府中市", "東京都","荒川中川合流地点" , "江東区", "東京都", 2000, 2, 0, CarPoolStatus.applying, ""),
    ]);
    return res;
  }

}
