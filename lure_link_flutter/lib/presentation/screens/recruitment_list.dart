import 'package:flutter/material.dart';
import 'package:lure_link_flutter/domains/repository/line_login/login.dart';
import '../components/recruitment_card.dart';

class RecruitmentList extends StatelessWidget {
  const RecruitmentList({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // todo:テスト用なのであとで削除します
        TextButton(
            onPressed: () async {
              try {
                final res = await LineLogin().login();
                print(res);
              } catch (_) {
                print("object");
              }
            },
            child: Text("LINE LOGIN")),
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
    );
  }
}
