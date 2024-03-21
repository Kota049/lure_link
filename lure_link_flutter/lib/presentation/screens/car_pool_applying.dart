import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/entity/carpool.dart';
import 'package:lure_link_flutter/domains/value_object/carpool_status.dart';


class CarPoolDetailScreen extends StatelessWidget {
  final Carpool carpool;
  const CarPoolDetailScreen({super.key, required this.carpool});
  @override
  Widget build(BuildContext context) {
    return DataTable(
        headingRowHeight: 0,
        columns:const [
          DataColumn(label: Text("項目")),
          DataColumn(label: Text("データ"))
        ],
        rows: [
          DataRow(cells: [
            const DataCell(Text("募集状況")),
            DataCell(Text(carpool.status.display))
          ]),
          DataRow(cells: [
            const DataCell(Text("開始日時")),
            DataCell(Text(carpool.startTime.toString()))
          ]),
          DataRow(cells: [
            const DataCell(Text("終了日時")),
            DataCell(Text(carpool.endTime.toString()))
          ]),
          DataRow(cells: [
            const DataCell(Text("目的地")),
            DataCell(Text(carpool.destinationPoint))
          ]),
          DataRow(cells: [
            const DataCell(Text("都道府県(目的地)")),
            DataCell(Text(carpool.destinationPrefecture))
          ]),
          DataRow(cells: [
            const DataCell(Text("市町村(目的地)")),
            DataCell(Text(carpool.destinationMunicipality))
          ]),
          DataRow(cells: [
            const DataCell(Text("出発地")),
            DataCell(Text(carpool.departurePoint))
          ]),
          DataRow(cells: [
            const DataCell(Text("都道府県(出発地)")),
            DataCell(Text(carpool.departurePrefecture))
          ]),
          DataRow(cells: [
            const DataCell(Text("市町村(出発地)")),
            DataCell(Text(carpool.departureMunicipality))
          ]),
          DataRow(cells: [
            const DataCell(Text("予算")),
            DataCell(Text(carpool.budget.toString()))
          ]),
          DataRow(cells: [
            const DataCell(Text("釣行人数")),
            DataCell(Text(carpool.maxParticipant.toString()))
          ]),
          DataRow(cells: [
            const DataCell(Text("目的地")),
            DataCell(Text(carpool.destinationPoint))
          ]),
        ],
    );
  }
}