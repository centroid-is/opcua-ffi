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
      .endpoint(endpointId: 'foo', endpoint: WrapClientEndpoint(url: 'opc.tcp://0.0.0.0:4855'))
      .defaultEndpoint('foo')
      .client();
  print('client');
  final (session, eventLoop) = await client.connectToEndpointId();
  print('session');
  await eventLoop.spawn();
  print('spawned');
  final connected = await session.waitForConnection();
  print('connected: $connected');

  final subscription_id = await session.createSubscriptionDataChange(
      publishingInterval: Duration(seconds: 1),
      lifetimeCount: 10,
      maxKeepAliveCount: 30,
      maxNotificationsPerPublish: 0,
      priority: 0,
      publishingEnabled: true,
      callback: DataChangeCallback((dataValue, monitoredItem) => {}));
  print('subscription_id: $subscription_id');
}
