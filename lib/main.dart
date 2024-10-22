import 'package:opcua_ffi/src/rust/api/minimal.dart';
import 'package:opcua_ffi/src/rust/frb_generated.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  print('init');
  final client = WrapClientBuilder()
      .applicationName('Simple Client')
      .applicationUri('urn:SimpleClient')
      .productUri('urn:SimpleClient')
      .trustServerCerts(true)
      .createSampleKeypair(true)
      .sessionRetryLimit(3)
      .endpoint(endpointId: 'localhost', endpoint: WrapClientEndpoint(url: 'opc.tcp://0.0.0.0:4855'))
      .defaultEndpoint('localhost')
      .userToken('userToken', WrapClientUserToken.userPass(user: 'sample1', password: 'sample1pwd'))
      .client();
  print('client');
  final (session, eventLoop) = await client.connectToEndpointId(endpointId: 'localhost');
  print('session');
  await eventLoop.spawn();
  print('spawned');
  final connected = await session.waitForConnection();
  print('connected: $connected');
  // print('Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
