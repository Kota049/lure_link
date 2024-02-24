enum CarPoolStatus {
  applying,
  aplComplete,
  cancel,
  finished,
  undefined
  ;

  static CarPoolStatus parse(String v) {
    switch (v) {
      case '申し込み中':
        return CarPoolStatus.applying;
      case '受付終了':
        return CarPoolStatus.aplComplete;
      case '中止':
        return CarPoolStatus.cancel;
      case '終了':
        return CarPoolStatus.finished;
      default:
        return CarPoolStatus.undefined;
    }
  }
}

extension UserStatusExt on CarPoolStatus {
  String get display {
    switch (this) {
      case CarPoolStatus.applying:
        return 'APPLYING';
      case CarPoolStatus.aplComplete:
        return 'APL_COMPLETE';
      case CarPoolStatus.cancel:
        return 'CANCEL';
      case CarPoolStatus.finished:
        return 'FINISHED';
      default:
        return 'undefined';
    }
  }
}
