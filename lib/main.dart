import 'package:opcua_ffi/src/rust/api/minimal.dart';
import 'package:opcua_ffi/src/rust/frb_generated.dart';
import 'package:opcua_ffi/src/rust/api/types/monitored_item_create_request.dart';
import 'package:opcua_ffi/src/rust/api/types/node_id.dart';
import 'package:opcua_ffi/src/rust/api/types/string.dart';
import 'package:opcua_ffi/src/rust/api/types/enums.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  print('init');
  final client = ClientBuilder()
      .applicationName('Simple Client')
      .applicationUri('urn:SimpleClient')
      .productUri('urn:SimpleClient')
      .trustServerCerts(true)
      .createSampleKeypair(true)
      .sessionRetryLimit(3)
      .endpoint(
          endpointId: 'foo',
          endpoint: ClientEndpoint(url: 'opc.tcp://0.0.0.0:4855'))
      .defaultEndpoint('foo')
      .client();
  print('client');
  final (session, eventLoop) = await client.connectToEndpointId();
  print('session');
  await eventLoop.spawn();
  print('spawned');
  final connected = await session.waitForConnection();
  print('connected: $connected');

  final subscription_id = await session.makeSubscription(
      publishingInterval: Duration(seconds: 1),
      lifetimeCount: 10,
      maxKeepAliveCount: 30,
      maxNotificationsPerPublish: 0,
      priority: 0,
      publishingEnabled: true);
  print('subscription_id: $subscription_id');

  final ns = 2;
  print('ns: $ns');

  List<String> identifiers = ["Slot/i64/spawned_bitch0", "Signal/string/hello"];
  print('identifiers: $identifiers');
  List<MonitoredItemCreateRequest> itemsToCreate = identifiers.map((id) {
    final nodeId =
        NodeId(namespace: ns, value: Identifier.string(UaString(id)));
    return nodeId.toMonitoredItemCreateRequest();
  }).toList();

  print('itemsToCreate: $itemsToCreate');
  final res = await session.createMonitoredItems(
      subscriptionId: subscription_id,
      timestampsToReturn: TimestampsToReturn.both,
      itemsToCreate: itemsToCreate);

  final stream = session.stream(subscription_id);

  stream.listen((dataChange) {
    print('dataChange: ${dataChange.dataValue.value}');
  });

  print('res: $res');
}
