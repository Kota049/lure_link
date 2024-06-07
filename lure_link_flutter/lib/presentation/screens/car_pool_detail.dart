import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/entity/carpool.dart';
import 'package:lure_link_flutter/domains/use_case/user_use_case.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_status.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_user_status.dart';
import 'package:provider/provider.dart';
import '../../domains/use_case/car_pool_use_case.dart';
import '../../domains/value_object/custom_error.dart';
import '../widgets/app_bar_with_login.dart';
import '../widgets/apply_btn.dart';

class CarPoolDetailScreen extends StatefulWidget{
  final Carpool carpool;
  const CarPoolDetailScreen({super.key,required this.carpool});

  @override
  CarPoolDetailScreenState createState() => CarPoolDetailScreenState();
}
class CarPoolDetailScreenState extends State<CarPoolDetailScreen>{
  CarPoolUserStatus _carPoolUserStatus = CarPoolUserStatus.undefined;
  final List<CustomError> _errors = [];
  @override
  void initState() {
    Future(() async {
      final carPoolUseCase = Provider.of<CarPoolUseCase>(context, listen: false);
      final userRepository = Provider.of<UserUseCase>(context,listen: false);
      if (userRepository.user?.applicationToken == null){
        return;
      }
      final res = await carPoolUseCase.canApl(widget.carpool.id, userRepository.user!.applicationToken);
      setState(() {
        res.fold((l) => _errors.add(l), (r) => _carPoolUserStatus = r);
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: const AppBarWithLoginButton(pageName: "募集詳細"),
        body: Column(children: [
          DataTable(
            headingRowHeight: 0,
            columns: const [
              DataColumn(label: Text("項目")),
              DataColumn(label: Text("データ"))
            ],
            rows: [
              DataRow(cells: [
                const DataCell(Text("募集状況")),
                DataCell(Text(widget.carpool.status.display))
              ]),
              DataRow(cells: [
                const DataCell(Text("開始日時")),
                DataCell(Text(widget.carpool.startTime.toString()))
              ]),
              DataRow(cells: [
                const DataCell(Text("終了日時")),
                DataCell(Text(widget.carpool.endTime.toString()))
              ]),
              DataRow(cells: [
                const DataCell(Text("目的地")),
                DataCell(Text(widget.carpool.destinationPoint))
              ]),
              DataRow(cells: [
                const DataCell(Text("都道府県(目的地)")),
                DataCell(Text(widget.carpool.destinationPrefecture))
              ]),
              DataRow(cells: [
                const DataCell(Text("市町村(目的地)")),
                DataCell(Text(widget.carpool.destinationMunicipality))
              ]),
              DataRow(cells: [
                const DataCell(Text("出発地")),
                DataCell(Text(widget.carpool.departurePoint))
              ]),
              DataRow(cells: [
                const DataCell(Text("都道府県(出発地)")),
                DataCell(Text(widget.carpool.departurePrefecture))
              ]),
              DataRow(cells: [
                const DataCell(Text("市町村(出発地)")),
                DataCell(Text(widget.carpool.departureMunicipality))
              ]),
              DataRow(cells: [
                const DataCell(Text("予算")),
                DataCell(Text(widget.carpool.budget.toString()))
              ]),
              DataRow(cells: [
                const DataCell(Text("釣行人数")),
                DataCell(Text(widget.carpool.maxParticipant.toString()))
              ]),
              DataRow(cells: [
                const DataCell(Text("目的地")),
                DataCell(Text(widget.carpool.destinationPoint))
              ]),
            ],
          ),
          ApplyButton(carPoolUserStatus: _carPoolUserStatus),
        ]));
  }
}
