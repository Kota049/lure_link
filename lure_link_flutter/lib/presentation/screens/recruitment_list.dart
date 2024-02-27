import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../domains/use_case/login_manager.dart';
import '../widgets/common_app_bar.dart';
import '../widgets/recruitment_card.dart';

class RecruitmentList extends StatelessWidget {
  const RecruitmentList({super.key});

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
          const RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "富津岬",
            user: '村岡正憲',
            startDate: "2023/09/10 18:00",
            departure: "東京都新宿区",
            budget: "3000円",
            memberCount: "0/2人",
          ),
          const RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "潮風公園",
            user: 'RED鈴木',
            startDate: "2023/09/24 0:00",
            departure: "茨城県つくば市",
            budget: "10000円",
            memberCount: "1/2人",
          ),
          const RecruitmentCard(
            destinationPrefecture: "",
            destinationCity: "",
            destination: "豊洲ぐるり公園",
            user: '高橋優介',
            startDate: "2023/09/10 18:00",
            departure: "東京都江東区",
            budget: "1000円",
            memberCount: "0/1人",
          ),
          const RecruitmentCard(
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
