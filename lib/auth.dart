import 'package:flutter/foundation.dart';
import 'package:oidc/oidc.dart';
import 'package:oidc_web_core/oidc_web_core.dart';

const clientId = '11ec9395-8c5d-4ac7-9bc2-f4505e7053cf';
final redirectLink = Uri.parse(
  kDebugMode
      ? 'http://localhost:8080/redirect.html'
      : 'https://justin.directory/auth',
);

final manager = OidcUserManager.lazy(
  discoveryDocumentUri: Uri.parse(
    'https://login.microsoftonline.com/common/.well-known/openid-configuration',
  ),
  clientCredentials: OidcClientAuthentication.none(clientId: clientId),
  settings: OidcUserManagerSettings(
    redirectUri: redirectLink,
    prompt: ['select_account'],
    scope: ['openid', 'profile', 'email'],
    options: OidcPlatformSpecificOptions(
      web: OidcPlatformSpecificOptions_Web(
        navigationMode: OidcPlatformSpecificOptions_Web_NavigationMode.samePage,
      ),
    ),
  ),
  store: OidcWebStore(),
);

class AuthenticationState extends ChangeNotifier {}
