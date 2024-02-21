import 'package:lure_link_flutter/domains/value_object/user_status.dart';

class Carpool {
  // todo implement field
  final int id;
  final String applicationToken;
  final String refreshToken;
  final String? nickName;
  final String? firstName;
  final String? lastName;
  final String? firstNameJp;
  final String? lastNameJp;
  final UserStatus? userStatus;

  Carpool(
      this.nickName,
      this.firstName,
      this.lastName,
      this.id,
      this.applicationToken,
      this.refreshToken,
      this.firstNameJp,
      this.lastNameJp,
      this.userStatus);

  Carpool.fromJson(Map<String, dynamic> json)
      : this(
    json['nick_name'] as String?,
    json['first_name'] as String?,
    json['last_name'] as String?,
    json['id'] as int,
    json['application_token'] as String,
    json['refresh_token'] as String,
    json['first_name_jp'] as String?,
    json['last_name_jp'] as String?,
    UserStatus.parse(json['status']),
  );

  Map<String, Object?> toJson() {
    return {
      'nick_name': nickName,
      'first_name': firstName,
      'fist_name_jp': firstNameJp,
      'id': id,
      'last_name': lastName,
      'last_name_jp': lastNameJp,
      'application_token': applicationToken,
      'refresh_token': refreshToken,
      'status': userStatus?.display,
    };
  }
}
