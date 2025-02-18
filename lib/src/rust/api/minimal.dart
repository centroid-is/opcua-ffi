// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.8.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'types/byte_string.dart';
import 'types/data_value.dart';
import 'types/date_time.dart';
import 'types/enums.dart';
import 'types/guid.dart';
import 'types/monitored_item.dart';
import 'types/monitored_item_create_request.dart';
import 'types/monitored_item_create_result.dart';
import 'types/status_code.dart';
import 'types/string.dart';
import 'types/variant.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `on_data_value`
// These functions are ignored (category: IgnoreBecauseExplicitAttribute): `from`, `from`, `from`, `from`, `new`

Future<void> datachangecallback({required DataChangeCallback a}) =>
    RustLib.instance.api.crateApiMinimalDatachangecallback(a: a);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Client>>
abstract class Client implements RustOpaqueInterface {
  /// Connects to a named endpoint that you have defined in the `ClientConfig`
  /// and creates a [`Session`] for that endpoint. Note that `GetEndpoints` is first
  /// called on the server and it is expected to support the endpoint you intend to connect to.
  ///
  /// # Returns
  ///
  /// * `Ok((Arc<AsyncSession>, SessionEventLoop))` - Session and event loop.
  /// * `Err(StatusCode)` - Request failed, [Status code](StatusCode) is the reason for failure.
  ///
  Future<(Session, SessionEventLoop)> connectToEndpointId({String? endpointId});
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ClientBuilder>>
abstract class ClientBuilder implements RustOpaqueInterface {
  /// Sets the application name.
  ClientBuilder applicationName(String applicationName);

  /// Sets the application uri.
  ClientBuilder applicationUri(String applicationUri);

  /// Sets a custom client certificate path. The path is required to be provided as a partial
  /// path relative to the PKI directory. If set, this path will be used to read the client
  /// certificate from disk. The certificate can be in either the .der or .pem format.
  ClientBuilder certificatePath(String certificatePath);

  /// Yields a [`Client`] from the values set by the builder. If the builder is not in a valid state
  /// it will return `None`.
  ///
  /// [`Client`]: client/struct.Client.html
  Client client();

  /// Sets whether the client should generate its own key pair if there is none found in the pki
  /// directory.
  ClientBuilder createSampleKeypair(bool createSampleKeypair);

  /// Sets the id of the default endpoint to connect to.
  ClientBuilder defaultEndpoint(String defaultEndpoint);

  /// Adds an endpoint to the list of endpoints the client knows of.
  ClientBuilder endpoint(
      {required String endpointId, required ClientEndpoint endpoint});

  /// Adds multiple endpoints to the list of endpoints the client knows of.
  ClientBuilder endpoints(List<(String, ClientEndpoint)> endpoints);

  /// Creates a `ClientBuilder` using a configuration file as the initial state.
  static ClientBuilder fromConfig({required String path}) =>
      RustLib.instance.api.crateApiMinimalClientBuilderFromConfig(path: path);

  /// Sets whether the client should ignore clock skew so the client can make a successful
  /// connection to the server, even when the client and server clocks are out of sync.
  ClientBuilder ignoreClockSkew();

  bool isValid();

  /// Time between making simple Read requests to the server to check for liveness
  /// and avoid session timeouts.
  ClientBuilder keepAliveInterval(Duration keepAliveInterval);

  /// Maximum number of array elements. 0 actually means 0, i.e. no array permitted
  ClientBuilder maxArrayLength(BigInt maxArrayLength);

  /// Maximum length in bytes of a byte string. 0 actually means 0, i.e. no byte strings permitted.
  ClientBuilder maxByteStringLength(BigInt maxByteStringLength);

  /// Sets the maximum number of chunks in an outgoing message. 0 means no limit.
  ClientBuilder maxChunkCount(BigInt maxChunkCount);

  /// Maximum size of each individual outgoing message chunk.
  ClientBuilder maxChunkSize(BigInt maxChunkSize);

  /// Maximum size of each incoming chunk.
  ClientBuilder maxIncomingChunkSize(BigInt maxIncomingChunkSize);

  /// Maximum number of inflight messages.
  ClientBuilder maxInflightMessages(BigInt maxInflightMessages);

  /// Maximum number of pending publish requests.
  ClientBuilder maxInflightPublish(BigInt maxInflightPublish);

  /// Sets the maximum outgoing message size in bytes. 0 means no limit.
  ClientBuilder maxMessageSize(BigInt maxMessageSize);

  /// Maximum length in bytes of a string. 0 actually means 0, i.e. no string permitted.
  ClientBuilder maxStringLength(BigInt maxStringLength);

  /// Set the lowest allowed publishing interval by the client.
  /// The server may also enforce its own minimum.
  ClientBuilder minPublishInterval(Duration minPublishInterval);

  /// Creates a `ClientBuilder`
  factory ClientBuilder() =>
      RustLib.instance.api.crateApiMinimalClientBuilderNew();

  /// Sets the pki directory where client's own key pair is stored and where `/trusted` and
  /// `/rejected` server certificates are stored.
  ClientBuilder pkiDir(String pkiDir);

  /// Sets the preferred locales of the client. These are passed to the server during session
  /// creation to ensure localized strings are in the preferred language.
  ClientBuilder preferredLocales(List<String> preferredLocales);

  /// Sets a custom private key path. The path is required to be provided as a partial path
  /// relative to the PKI directory. If set, this path will be used to read the private key
  /// from disk.
  ClientBuilder privateKeyPath(String privateKeyPath);

  /// Sets the product uri.
  ClientBuilder productUri(String productUri);

  /// Set the timeout on publish requests sent to the server.
  ClientBuilder publishTimeout(Duration publishTimeout);

  /// When a session is recreated on the server, the client will attempt to
  /// transfer monitored subscriptions from the old session to the new.
  /// This is the maximum number of monitored items to create per request.
  ClientBuilder recreateMonitoredItemsChunk(BigInt recreateMonitoredItemsChunk);

  /// Set the timeout on requests sent to the server.
  ClientBuilder requestTimeout(Duration requestTimeout);

  /// Session name - the default name to use for a new session
  ClientBuilder sessionName(String sessionName);

  /// Initial time between retries when backing off on session reconnects.
  ClientBuilder sessionRetryInitial(Duration sessionRetryInitial);

  /// Sets the session retry limit.
  ///
  /// # Panics
  ///
  /// Panics if `session_retry_limit` is less -1.
  ClientBuilder sessionRetryLimit(int sessionRetryLimit);

  /// Maximum time between retries when backing off on session reconnects.
  ClientBuilder sessionRetryMax(Duration sessionRetryMax);

  /// Sets the session timeout period, in milliseconds.
  ClientBuilder sessionTimeout(int sessionTimeout);

  /// Sets whether the client should automatically trust servers. If this is not set then
  /// the client will reject the server upon first connect and the server's certificate
  /// must be manually moved from pki's `/rejected` folder to the `/trusted` folder. If it is
  /// set, then the server cert will automatically be stored in the `/trusted` folder.
  ClientBuilder trustServerCerts(bool trustServerCerts);

  /// Adds a user token to the list supported by the client.
  ClientBuilder userToken(String userTokenId, ClientUserToken userToken);

  /// Sets whether the client should verify server certificates. Regardless of this setting,
  /// server certificates are always checked to see if they are trusted and have a valid key
  /// length. In addition (if `verify_server_certs` is unset or is set to `true`) it will
  /// verify the hostname, application uri and the not before / after values to ensure validity.
  ClientBuilder verifyServerCerts(bool verifyServerCerts);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ClientEndpoint>>
abstract class ClientEndpoint implements RustOpaqueInterface {
  factory ClientEndpoint({required String url}) =>
      RustLib.instance.api.crateApiMinimalClientEndpointNew(url: url);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ClientUserToken>>
abstract class ClientUserToken implements RustOpaqueInterface {
  bool isValid();

  static ClientUserToken userPass(
          {required String user, required String password}) =>
      RustLib.instance.api.crateApiMinimalClientUserTokenUserPass(
          user: user, password: password);

  static ClientUserToken x509(
          {required String user,
          required String certPath,
          required String privateKeyPath}) =>
      RustLib.instance.api.crateApiMinimalClientUserTokenX509(
          user: user, certPath: certPath, privateKeyPath: privateKeyPath);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DataChangeCallback>>
abstract class DataChangeCallback implements RustOpaqueInterface {
  /// Create a new data change callback wrapper.
  ///
  /// # Arguments
  ///
  /// * `data_value` - Called for each received data value.
  factory DataChangeCallback(
          FutureOr<void> Function(DataValue, MonitoredItem) dataValue) =>
      RustLib.instance.api
          .crateApiMinimalDataChangeCallbackNew(dataValue: dataValue);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner< JoinHandle < StatusCode >>>
abstract class JoinHandleStatusCode implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Session>>
abstract class Session implements RustOpaqueInterface {
  /// Creates monitored items on a subscription by sending a [`CreateMonitoredItemsRequest`] to the server.
  ///
  /// See OPC UA Part 4 - Services 5.12.2 for complete description of the service and error responses.
  ///
  /// # Arguments
  ///
  /// * `subscription_id` - The Server-assigned identifier for the Subscription that will report Notifications for this MonitoredItem
  /// * `timestamps_to_return` - An enumeration that specifies the timestamp Attributes to be transmitted for each MonitoredItem.
  /// * `items_to_create` - A list of [`MonitoredItemCreateRequest`] to be created and assigned to the specified Subscription.
  ///
  /// # Returns
  ///
  /// * `Ok(Vec<MonitoredItemCreateResult>)` - A list of [`MonitoredItemCreateResult`] corresponding to the items to create.
  ///    The size and order of the list matches the size and order of the `items_to_create` request parameter.
  /// * `Err(StatusCode)` - Request failed, [Status code](StatusCode) is the reason for failure.
  ///
  Future<List<MonitoredItemCreateResult>> createMonitoredItems(
      {required int subscriptionId,
      required TimestampsToReturn timestampsToReturn,
      required List<MonitoredItemCreateRequest> itemsToCreate});

  /// Send a message and wait for response, using the default configured timeout.
  ///
  /// In order to set a different timeout, call `send` on the inner channel instead.
  /// Create a subscription by sending a [`CreateSubscriptionRequest`] to the server.
  ///
  /// See OPC UA Part 4 - Services 5.13.2 for complete description of the service and error responses.
  ///
  /// # Arguments
  ///
  /// * `publishing_interval` - The requested publishing interval defines the cyclic rate that
  ///   the Subscription is being requested to return Notifications to the Client. This interval
  ///   is expressed in milliseconds. This interval is represented by the publishing timer in the
  ///   Subscription state table. The negotiated value for this parameter returned in the
  ///   response is used as the default sampling interval for MonitoredItems assigned to this
  ///   Subscription. If the requested value is 0 or negative, the server shall revise with the
  ///   fastest supported publishing interval in milliseconds.
  /// * `lifetime_count` - Requested lifetime count. The lifetime count shall be a minimum of
  ///   three times the keep keep-alive count. When the publishing timer has expired this
  ///   number of times without a Publish request being available to send a NotificationMessage,
  ///   then the Subscription shall be deleted by the Server.
  /// * `max_keep_alive_count` - Requested maximum keep-alive count. When the publishing timer has
  ///   expired this number of times without requiring any NotificationMessage to be sent, the
  ///   Subscription sends a keep-alive Message to the Client. The negotiated value for this
  ///   parameter is returned in the response. If the requested value is 0, the server shall
  ///   revise with the smallest supported keep-alive count.
  /// * `max_notifications_per_publish` - The maximum number of notifications that the Client
  ///   wishes to receive in a single Publish response. A value of zero indicates that there is
  ///   no limit. The number of notifications per Publish is the sum of monitoredItems in
  ///   the DataChangeNotification and events in the EventNotificationList.
  /// * `priority` - Indicates the relative priority of the Subscription. When more than one
  ///   Subscription needs to send Notifications, the Server should de-queue a Publish request
  ///   to the Subscription with the highest priority number. For Subscriptions with equal
  ///   priority the Server should de-queue Publish requests in a round-robin fashion.
  ///   A Client that does not require special priority settings should set this value to zero.
  /// * `publishing_enabled` - A boolean parameter with the following values - `true` publishing
  ///   is enabled for the Subscription, `false`, publishing is disabled for the Subscription.
  ///   The value of this parameter does not affect the value of the monitoring mode Attribute of
  ///   MonitoredItems.
  ///
  /// # Returns
  ///
  /// * `Ok(u32)` - identifier for new subscription
  /// * `Err(StatusCode)` - Request failed, [Status code](StatusCode) is the reason for failure.
  ///
  Future<int> createSubscriptionDataChange(
      {required Duration publishingInterval,
      required int lifetimeCount,
      required int maxKeepAliveCount,
      required int maxNotificationsPerPublish,
      required int priority,
      required bool publishingEnabled,
      required DataChangeCallback callback});

  /// Disconnect from the server and wait until disconnected.
  Future<void> disconnect();

  /// The internal ID of the session, used to keep track of multiple sessions in the same program.
  int sessionId();

  /// Convenience method to wait for a connection to the server.
  ///
  /// You should also monitor the session event loop. If it ends, this method will never return.
  Future<bool> waitForConnection();
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SessionEventLoop>>
abstract class SessionEventLoop implements RustOpaqueInterface {
  /// Convenience method for running the session event loop until completion,
  /// this method will return once the session is closed manually, or
  /// after it fails to reconnect.
  ///
  /// # Returns
  ///
  /// * `StatusCode` - [Status code](StatusCode) indicating how the session terminated.
  Future<StatusCode> run();

  /// Convenience method for running the session event loop until completion on a tokio task.
  /// This method will return a [`JoinHandle`](tokio::task::JoinHandle) that will terminate
  /// once the session is closed manually, or after it fails to reconnect.
  ///
  /// # Returns
  ///
  /// * `JoinHandle<StatusCode>` - Handle to a tokio task wrapping the event loop.
  Future<JoinHandleStatusCode> spawn();
}
