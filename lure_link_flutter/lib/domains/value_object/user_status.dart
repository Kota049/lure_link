enum UserStatus {
  trial,
  registration,
  undefined,
  ;

  static UserStatus parse(String v) {
    switch (v) {
      case 'TRAIL':
        return UserStatus.trial;
      case 'REGISTRATION':
        return UserStatus.registration;
      default:
        return UserStatus.undefined;
    }
  }
}

extension UserStatusExt on UserStatus {
  String get display {
    switch (this) {
      case UserStatus.trial:
        return '仮登録';
      case UserStatus.registration:
        return '本登録';
      default:
        return 'undefined';
    }
  }
}
