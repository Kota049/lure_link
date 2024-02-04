enum UserStatus {
  trial,
  registration,
  undefined,
  ;

  UserStatus parse(String v) {
    switch (v) {
      case 'TRAIL':
        return UserStatus.trial;
      case 'REGISTRATION':
        return UserStatus.registration;
      default:
        return UserStatus.undefined;
    }
  }

  String display(UserStatus u) {
    switch (u) {
      case UserStatus.trial:
        return '仮登録';
      case UserStatus.registration:
        return '本登録';
      default:
        return 'Undefined';
    }
  }
}
