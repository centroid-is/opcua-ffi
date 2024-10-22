import 'dart:async';

import 'package:opcua_ffi/src/rust/api/minimal.dart';
import 'package:opcua_ffi/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('Client Builder', () async {
    final client =
        WrapClientBuilder().applicationName('Simple Client').applicationUri('urn:SimpleClient').productUri('urn:SimpleClient').trustServerCerts(true).createSampleKeypair(true).sessionRetryLimit(3);
    expect(client.isValid(), true);
  });
  print('Action: Configure tests (end)');
}
