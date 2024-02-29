import 'package:flutter/material.dart';
import '../widgets/not_login_app_bar.dart';
import 'package:lure_link_flutter/domains/entity/Carpool.dart';
import 'package:provider/provider.dart';
import '../../domains/use_case/user_use_case.dart';
import '../../domains/value_object/custom_error.dart';
import '../widgets/common_app_bar.dart';
import '../widgets/recruitment_card.dart';

class CarPoolScreen extends StatefulWidget {
  const CarPoolScreen({super.key});

  @override
  CarPoolScreenState createState() => CarPoolScreenState();
}

class CarPoolScreenState extends State<CarPoolScreen>{
  List<Carpool> _carPoolList = [];
  final List<CustomError> _errors = [];

  @override
  void initState() {
    // TODO: implement initState
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final userUseCase = Provider.of<UserUseCase>(context);
    return Scaffold(
      appBar: const CommonAppBar(pageName: "LURELINK"),
      body: Column(
        children: [
          !userUseCase.isAuthenticated()
              ? TextButton(
              onPressed: () async {
                Navigator.of(context).pushNamed('/login');
              },
              child: const Text("LINE LOGIN"))
              : const SizedBox(),
        ],
      ),
    );
  }
}

class RecruitmentList extends StatelessWidget {
  const RecruitmentList({super.key});

  @override
  Widget build(BuildContext context) {
    // final userUseCase = Provider.of<UserUseCase>(context);
    return const Scaffold(
      appBar: NotLoginAppBar(pageName: "LURELINK"),
      body: Column(
        children: [
          RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "富津岬",
            user: '村岡正憲',
            startDate: "2023/09/10 18:00",
            departure: "東京都新宿区",
            budget: "3000円",
            memberCount: "0/2人",
          ),
          RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "潮風公園",
            user: 'RED鈴木',
            startDate: "2023/09/24 0:00",
            departure: "茨城県つくば市",
            budget: "10000円",
            memberCount: "1/2人",
          ),
          RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "豊洲ぐるり公園",
            user: '高橋優介',
            startDate: "2023/09/10 18:00",
            departure: "東京都江東区",
            budget: "1000円",
            memberCount: "0/1人",
          ),
          RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "武庫川一文字",
            user: '橋本翔大',
            startDate: "2023/09/10 18:00",
            departure: "東京都新宿区",
            budget: "3000円",
            memberCount: "0/2人",
          ),
        ],
      ),
    );
  }
}
