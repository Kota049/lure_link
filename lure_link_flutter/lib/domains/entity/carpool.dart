import 'dart:core';
import 'package:lure_link_flutter/domains/value_object/carpool_status.dart';

class Carpool {
  int id;
  int organizerId;
  String organizerNickName;
  DateTime startTime;
  DateTime endTime;
  DateTime aplDeadline;
  String departurePoint;
  String departurePrefecture;
  String departureMunicipality;
  String destinationPoint;
  String destinationPrefecture;
  String destinationMunicipality;
  int budget;
  int maxParticipant;
  int currentParticipant;
  CarPoolStatus status;
  String? description;

  Carpool(
      this.id,
      this.organizerId,
      this.organizerNickName,
      this.startTime,
      this.endTime,
      this.aplDeadline,
      this.departurePoint,
      this.departurePrefecture,
      this.departureMunicipality,
      this.destinationPoint,
      this.destinationPrefecture,
      this.destinationMunicipality,
      this.budget,
      this.maxParticipant,
      this.currentParticipant,
      this.status,
      this.description,
  );

  Carpool.fromJson(Map<String, dynamic> json)
      : this(
          json['id'] as int,
          json['organizerId'] as int,
          json['organizerNickName'] as String,
          json['startTime'] as DateTime,
          json['endTime'] as DateTime,
          json['aplDeadline'] as DateTime,
          json['departurePoint'] as String,
          json['departurePrefecture'] as String,
          json['departureMunicipality'] as String,
          json['destinationPoint'] as String,
          json['destinationPrefecture'] as String,
          json['destinationMunicipality'] as String,
          json['budget'] as int,
          json['maxParticipant'] as int,
          json['currentParticipant'] as int,
          CarPoolStatus.parse(json['status']),
          json['description'] as String?,
        );

  Map<String, Object?> toJson() {
    return {
      'id':id,
      'organizerId':organizerId,
      'organizerNickName':organizerNickName,
      'startTime':startTime,
      'endTime':endTime,
      'aplDeadline':aplDeadline,
      'departurePoint':departurePoint,
      'departurePrefecture':departurePrefecture,
      'departureMunicipality':departureMunicipality,
      'destinationPoint':destinationPoint,
      'destinationPrefecture':destinationPrefecture,
      'destinationMunicipality':destinationMunicipality,
      'budget':budget,
      'maxParticipant':maxParticipant,
      'currentParticipant':currentParticipant,
      'status':status.display,
    };
  }
}
