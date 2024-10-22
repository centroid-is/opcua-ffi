// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored (category: IgnoreBecauseExplicitAttribute): `from`, `from`, `from`, `new`
// These functions are ignored (category: IgnoreBecauseNotAllowedOwner): `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner< JoinHandle < String >>>
abstract class JoinHandleString implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapClient>>
abstract class WrapClient implements RustOpaqueInterface {
  /// Connects to a named endpoint that you have defined in the `ClientConfig`
  /// and creates a [`Session`] for that endpoint. Note that `GetEndpoints` is first
  /// called on the server and it is expected to support the endpoint you intend to connect to.
  ///
  /// # Returns
  ///
  /// * `Ok((Arc<AsyncSession>, SessionEventLoop))` - Session and event loop.
  /// * `Err(StatusCode)` - Request failed, [Status code](StatusCode) is the reason for failure.
  ///
  Future<(WrapSession, WrapSessionEventLoop)> connectToEndpointId(
      {String? endpointId});
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapClientBuilder>>
abstract class WrapClientBuilder implements RustOpaqueInterface {
  /// Sets the application name.
  WrapClientBuilder applicationName(String applicationName);

  /// Sets the application uri.
  WrapClientBuilder applicationUri(String applicationUri);

  /// Sets a custom client certificate path. The path is required to be provided as a partial
  /// path relative to the PKI directory. If set, this path will be used to read the client
  /// certificate from disk. The certificate can be in either the .der or .pem format.
  WrapClientBuilder certificatePath(String certificatePath);

  /// Yields a [`Client`] from the values set by the builder. If the builder is not in a valid state
  /// it will return `None`.
  ///
  /// [`Client`]: client/struct.Client.html
  WrapClient client();

  /// Sets whether the client should generate its own key pair if there is none found in the pki
  /// directory.
  WrapClientBuilder createSampleKeypair(bool createSampleKeypair);

  /// Sets the id of the default endpoint to connect to.
  WrapClientBuilder defaultEndpoint(String defaultEndpoint);

  /// Adds an endpoint to the list of endpoints the client knows of.
  WrapClientBuilder endpoint(
      {required String endpointId, required WrapClientEndpoint endpoint});

  /// Adds multiple endpoints to the list of endpoints the client knows of.
  WrapClientBuilder endpoints(List<(String, WrapClientEndpoint)> endpoints);

  /// Creates a `ClientBuilder` using a configuration file as the initial state.
  static WrapClientBuilder fromConfig({required String path}) =>
      RustLib.instance.api
          .crateApiMinimalWrapClientBuilderFromConfig(path: path);

  /// Sets whether the client should ignore clock skew so the client can make a successful
  /// connection to the server, even when the client and server clocks are out of sync.
  WrapClientBuilder ignoreClockSkew();

  bool isValid();

  /// Time between making simple Read requests to the server to check for liveness
  /// and avoid session timeouts.
  WrapClientBuilder keepAliveInterval(Duration keepAliveInterval);

  /// Maximum number of array elements. 0 actually means 0, i.e. no array permitted
  WrapClientBuilder maxArrayLength(BigInt maxArrayLength);

  /// Maximum length in bytes of a byte string. 0 actually means 0, i.e. no byte strings permitted.
  WrapClientBuilder maxByteStringLength(BigInt maxByteStringLength);

  /// Sets the maximum number of chunks in an outgoing message. 0 means no limit.
  WrapClientBuilder maxChunkCount(BigInt maxChunkCount);

  /// Maximum size of each individual outgoing message chunk.
  WrapClientBuilder maxChunkSize(BigInt maxChunkSize);

  /// Maximum size of each incoming chunk.
  WrapClientBuilder maxIncomingChunkSize(BigInt maxIncomingChunkSize);

  /// Maximum number of inflight messages.
  WrapClientBuilder maxInflightMessages(BigInt maxInflightMessages);

  /// Maximum number of pending publish requests.
  WrapClientBuilder maxInflightPublish(BigInt maxInflightPublish);

  /// Sets the maximum outgoing message size in bytes. 0 means no limit.
  WrapClientBuilder maxMessageSize(BigInt maxMessageSize);

  /// Maximum length in bytes of a string. 0 actually means 0, i.e. no string permitted.
  WrapClientBuilder maxStringLength(BigInt maxStringLength);

  /// Set the lowest allowed publishing interval by the client.
  /// The server may also enforce its own minimum.
  WrapClientBuilder minPublishInterval(Duration minPublishInterval);

  /// Creates a `ClientBuilder`
  factory WrapClientBuilder() =>
      RustLib.instance.api.crateApiMinimalWrapClientBuilderNew();

  /// Sets the pki directory where client's own key pair is stored and where `/trusted` and
  /// `/rejected` server certificates are stored.
  WrapClientBuilder pkiDir(String pkiDir);

  /// Sets the preferred locales of the client. These are passed to the server during session
  /// creation to ensure localized strings are in the preferred language.
  WrapClientBuilder preferredLocales(List<String> preferredLocales);

  /// Sets a custom private key path. The path is required to be provided as a partial path
  /// relative to the PKI directory. If set, this path will be used to read the private key
  /// from disk.
  WrapClientBuilder privateKeyPath(String privateKeyPath);

  /// Sets the product uri.
  WrapClientBuilder productUri(String productUri);

  /// Set the timeout on publish requests sent to the server.
  WrapClientBuilder publishTimeout(Duration publishTimeout);

  /// When a session is recreated on the server, the client will attempt to
  /// transfer monitored subscriptions from the old session to the new.
  /// This is the maximum number of monitored items to create per request.
  WrapClientBuilder recreateMonitoredItemsChunk(
      BigInt recreateMonitoredItemsChunk);

  /// Set the timeout on requests sent to the server.
  WrapClientBuilder requestTimeout(Duration requestTimeout);

  /// Session name - the default name to use for a new session
  WrapClientBuilder sessionName(String sessionName);

  /// Initial time between retries when backing off on session reconnects.
  WrapClientBuilder sessionRetryInitial(Duration sessionRetryInitial);

  /// Sets the session retry limit.
  ///
  /// # Panics
  ///
  /// Panics if `session_retry_limit` is less -1.
  WrapClientBuilder sessionRetryLimit(int sessionRetryLimit);

  /// Maximum time between retries when backing off on session reconnects.
  WrapClientBuilder sessionRetryMax(Duration sessionRetryMax);

  /// Sets the session timeout period, in milliseconds.
  WrapClientBuilder sessionTimeout(int sessionTimeout);

  /// Sets whether the client should automatically trust servers. If this is not set then
  /// the client will reject the server upon first connect and the server's certificate
  /// must be manually moved from pki's `/rejected` folder to the `/trusted` folder. If it is
  /// set, then the server cert will automatically be stored in the `/trusted` folder.
  WrapClientBuilder trustServerCerts(bool trustServerCerts);

  /// Adds a user token to the list supported by the client.
  WrapClientBuilder userToken(
      String userTokenId, WrapClientUserToken userToken);

  /// Sets whether the client should verify server certificates. Regardless of this setting,
  /// server certificates are always checked to see if they are trusted and have a valid key
  /// length. In addition (if `verify_server_certs` is unset or is set to `true`) it will
  /// verify the hostname, application uri and the not before / after values to ensure validity.
  WrapClientBuilder verifyServerCerts(bool verifyServerCerts);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapClientEndpoint>>
abstract class WrapClientEndpoint implements RustOpaqueInterface {
  factory WrapClientEndpoint({required String url}) =>
      RustLib.instance.api.crateApiMinimalWrapClientEndpointNew(url: url);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapClientUserToken>>
abstract class WrapClientUserToken implements RustOpaqueInterface {
  bool isValid();

  static WrapClientUserToken userPass(
          {required String user, required String password}) =>
      RustLib.instance.api.crateApiMinimalWrapClientUserTokenUserPass(
          user: user, password: password);

  static WrapClientUserToken x509(
          {required String user,
          required String certPath,
          required String privateKeyPath}) =>
      RustLib.instance.api.crateApiMinimalWrapClientUserTokenX509(
          user: user, certPath: certPath, privateKeyPath: privateKeyPath);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapSession>>
abstract class WrapSession implements RustOpaqueInterface {
  /// Disconnect from the server and wait until disconnected.
  Future<void> disconnect();

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
  /// The internal ID of the session, used to keep track of multiple sessions in the same program.
  Future<int> sessionId();

  /// Convenience method to wait for a connection to the server.
  ///
  /// You should also monitor the session event loop. If it ends, this method will never return.
  Future<bool> waitForConnection();
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<WrapSessionEventLoop>>
abstract class WrapSessionEventLoop implements RustOpaqueInterface {
  /// Convenience method for running the session event loop until completion,
  /// this method will return once the session is closed manually, or
  /// after it fails to reconnect.
  ///
  /// # Returns
  ///
  /// * `StatusCode` - [Status code](StatusCode) indicating how the session terminated.
  Future<String> run();

  /// Convenience method for running the session event loop until completion on a tokio task.
  /// This method will return a [`JoinHandle`](tokio::task::JoinHandle) that will terminate
  /// once the session is closed manually, or after it fails to reconnect.
  ///
  /// # Returns
  ///
  /// * `JoinHandle<StatusCode>` - Handle to a tokio task wrapping the event loop.
  Future<JoinHandleString> spawn();
}
