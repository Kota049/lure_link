import 'package:flutter/material.dart';

import 'dart:convert';
import 'package:http/http.dart' as http;

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a blue toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: Scaffold(
        appBar: AppBar(
            backgroundColor: Colors.lightBlue, title: const Text("LURELINK")),
        body: const RecruitmentList(title: '富津岬', user: "村岡正憲"),
      ),
    );
  }
}

class RecruitmentList extends StatelessWidget {
  final String title;
  final String user;

  const RecruitmentList({super.key, required this.title, required this.user});

  @override
  Widget build(BuildContext context) {
    return Card(
        child: Row(
      children: <Widget>[
        // ClipRRect(
        //   borderRadius: BorderRadius.circular(10),
        //   child: Image.asset(
        //     'images/saba.png',
        //     width: 100,
        //     height: 100,
        //     alignment: Alignment.center,
        //   ),
        // ),
        ClipRRect(
          borderRadius: BorderRadius.circular(20), // Image border
          child: SizedBox.fromSize(
            size: const Size.fromRadius(48), // Image radius
            child: Image.asset('images/saba.png', fit: BoxFit.cover),
          ),
        ),
        Flexible(
          child: ListTile(
            subtitle: Text(user),
            title: Text(title),
          ),
        )
      ],
    ));
  }
}
