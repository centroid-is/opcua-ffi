import 'dart:async';

import 'package:opcua_ffi/src/rust/api/minimal.dart';
import 'package:opcua_ffi/src/rust/api/status_code.dart';
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
  test('Status Code', () async {
    final code = WrapStatusCode.fromStr('foo');
    expect(code == null, true);
    final code2 = WrapStatusCode.fromStr('BadCertificateTimeInvalid');
    expect(code2 != null, true);
    expect(code2!.name(), 'BadCertificateTimeInvalid');
  });
  print('Action: Configure tests (end)');
}
