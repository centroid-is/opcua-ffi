import 'dart:async';

import 'package:opcua_ffi/src/rust/api/minimal.dart';
import 'package:opcua_ffi/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  test('Client User Token', () async {
    final token = await ClientUserToken.userPass(user: 'foo', password: 'bar');
    expect(await token.isValid(), true);
    expect(token.user, 'foo');
    expect(token.password, 'bar');
  });
  print('Action: Configure tests (end)');
}
