class User {
  final int id;
  final String applicationToken;
  final String refreshToken;
  final String? nickName;
  final String? firstName;
  final String? lastName;
  final String? firstNameJp;
  final String? lastNameJp;
  final String? userStatus;

  User(
      this.nickName,
      this.firstName,
      this.lastName,
      this.id,
      this.applicationToken,
      this.refreshToken,
      this.firstNameJp,
      this.lastNameJp,
      this.userStatus);
}
