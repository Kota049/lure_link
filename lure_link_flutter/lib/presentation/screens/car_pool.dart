import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/use_case/car_pool_use_case.dart';
import 'package:lure_link_flutter/presentation/widgets/car_pool_summary.dart';
import '../../domains/entity/carpool.dart';
import 'package:provider/provider.dart';
import '../../domains/value_object/custom_error.dart';

class CarPoolScreen extends StatefulWidget {
  const CarPoolScreen({super.key});

  @override
  CarPoolScreenState createState() => CarPoolScreenState();
}

class CarPoolScreenState extends State<CarPoolScreen> {
  List<Carpool> _carPoolList = [];
  final List<CustomError> _errors = [];

  @override
  void initState() {
    Future(() async {
      final carPoolUseCase =
          Provider.of<CarPoolUseCase>(context, listen: false);
      final res = await carPoolUseCase.index();
      setState(() {
        res.fold((l) => _errors.add(l), (r) => _carPoolList = r);
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return _carPoolList.isEmpty
        ? const SizedBox()
        : Column(
            children:
                _carPoolList.map((el) => CarPoolSummary(carpool: el)).toList(),
          );
  }
}
