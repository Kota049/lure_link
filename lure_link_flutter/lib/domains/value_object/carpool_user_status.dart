enum CarPoolUserStatus {
  applying,
  canApl,
  cannotApl,
  owner,
  undefined
  ;

  static CarPoolUserStatus parse(String v) {
    switch (v) {
      case '申し込み中':
        return CarPoolUserStatus.applying;
      case '申し込み可能':
        return CarPoolUserStatus.canApl;
      case '申し込み不可':
        return CarPoolUserStatus.cannotApl;
      case '主催者':
        return CarPoolUserStatus.owner;
      default:
        return CarPoolUserStatus.undefined;
    }
  }
}

extension ApplyBtnStatus on CarPoolUserStatus{
  String get btnText{
    switch (this) {
      case CarPoolUserStatus.applying:
        return '申し込み内容をを確認する';
      case CarPoolUserStatus.canApl:
        return '申し込む';
      case CarPoolUserStatus.cannotApl:
        return '申し込みは締め切りました';
      case CarPoolUserStatus.owner:
        return '募集詳細へ';
      default:
        return '申し込みできません';
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
      default:
        return 'undefined';
    }
  }
}