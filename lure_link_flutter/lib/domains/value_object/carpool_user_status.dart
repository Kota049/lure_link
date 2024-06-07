enum CarPoolUserStatus {
  applying,
  canApl,
  cannotApl,
  owner,
  undefined,
  notLogin;

  static CarPoolUserStatus parse(String v) {
    switch (v) {
      case 'APPLYING':
        return CarPoolUserStatus.applying;
      case 'CAN_APL':
        return CarPoolUserStatus.canApl;
      case 'CANNOT_APL':
        return CarPoolUserStatus.cannotApl;
      case 'OWNER':
        return CarPoolUserStatus.owner;
      case 'NOT_LOGIN':
        return CarPoolUserStatus.notLogin;
      default:
        return CarPoolUserStatus.undefined;
    }
  }
}
extension ApplyButtonText on CarPoolUserStatus {
  String get btnText {
    switch (this) {
      case CarPoolUserStatus.applying:
        return '申し込みを確認する';
      case CarPoolUserStatus.canApl:
        return '申し込む';
      case CarPoolUserStatus.cannotApl:
        return '現在申し込みはできません';
      case CarPoolUserStatus.owner:
        return '募集詳細を確認する';
      case CarPoolUserStatus.notLogin:
        return 'ログインして申し込む';
      default:
        return '現在申し込みできません';
    }
  }
}

extension UserStatusExt on CarPoolUserStatus {
  String get display {
    switch (this) {
      case CarPoolUserStatus.applying:
        return 'APPLYING';
      case CarPoolUserStatus.canApl:
        return 'CAN_APL';
      case CarPoolUserStatus.cannotApl:
        return 'CANNOT_APL';
      case CarPoolUserStatus.owner:
        return 'OWNER';
      case CarPoolUserStatus.notLogin:
        return 'NOT_LOGIN';
      default:
        return 'undefined';
    }
  }
}
