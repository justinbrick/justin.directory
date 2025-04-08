import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import 'package:oidc/oidc.dart';
import 'package:oidc_web_core/oidc_web_core.dart';
import 'package:provider/provider.dart';
import 'package:async/async.dart';

final authLogger = Logger('directory.justin.auth');

const clientId = '11ec9395-8c5d-4ac7-9bc2-f4505e7053cf';
final rootDirectory = Uri.parse(
  kDebugMode ? 'http://localhost:8080/' : 'https://justin.directory/',
);
final redirectLink = Uri.parse(
  kDebugMode ? '${rootDirectory}redirect.html' : '${rootDirectory}auth',
);

final userManager = OidcUserManager.lazy(
  discoveryDocumentUri: Uri.parse(
    'https://login.microsoftonline.com/organizations/v2.0/.well-known/openid-configuration',
  ),
  clientCredentials: OidcClientAuthentication.none(clientId: clientId),
  store: OidcWebStore(
    webSessionManagementLocation:
        OidcWebStoreSessionManagementLocation.localStorage,
  ),
  settings: OidcUserManagerSettings(
    redirectUri: redirectLink,
    strictJwtVerification: true,
    scope: [
      OidcConstants_Scopes.openid,
      OidcConstants_Scopes.profile,
      OidcConstants_Scopes.email,
      OidcConstants_Scopes.openid,
    ],
    options: OidcPlatformSpecificOptions(
      web: OidcPlatformSpecificOptions_Web(
        navigationMode: OidcPlatformSpecificOptions_Web_NavigationMode.samePage,
      ),
    ),
  ),
);

final managerInit = AsyncMemoizer<void>();
Future<void> initializeManager() {
  return managerInit.runOnce(() async {
    await userManager.init();

    // we are using entra, and users sign in with custom tenant issuers, so we need to remove issuer so it does not validate.
    userManager.discoveryDocument = userManager.discoveryDocument.copyWith(
      issuer: null,
      codeChallengeMethodsSupported: ['S256'],
    );

    authLogger.info('Manager initialized');
  });
}

class AuthenticationLayout extends StatelessWidget {
  const AuthenticationLayout({super.key, required this.child});
  final Widget child;
  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: initializeManager(),
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return const Center(child: CircularProgressIndicator());
        }

        if (snapshot.hasError) {
          return Center(child: Text('Error: ${snapshot.error}'));
        }

        return ChangeNotifierProvider(
          create: (context) => AuthenticationState(userManager: userManager),
          child: child,
        );
      },
    );
  }
}

class AuthenticationState extends ChangeNotifier {
  AuthenticationState({required this.userManager}) {
    _user = userManager.currentUser;
    userManager.userChanges().listen((event) {
      _user = event;
      notifyListeners();
    });
    notifyListeners();
  }

  void signIn() async {}

  final OidcUserManager userManager;
  OidcUser? _user;
  OidcUser? get user => _user;
}

class UserIconWidget extends StatefulWidget {
  const UserIconWidget({super.key});
  @override
  State<StatefulWidget> createState() => _UserIconWidgetState();
}

class _UserIconWidgetState extends State<UserIconWidget> {
  AuthenticationState? authState;

  void _loginUser() async {
    await authState?.userManager.loginAuthorizationCodeFlow(
      originalUri: Uri.parse('/'),
    );
  }

  void _logoutUser() async {
    await authState?.userManager.logout();
  }

  @override
  Widget build(BuildContext context) {
    authState = context.watch<AuthenticationState>();
    if (authState?.user == null) {
      return Row(
        children: [
          TextButton(onPressed: _loginUser, child: const Text('Login')),
          const Icon(Icons.account_circle_outlined),
        ],
      );
    }

    // todo: add user pfp as the icon
    return Row(
      children: [
        TextButton(onPressed: _logoutUser, child: const Text('Logout')),
        const Icon(Icons.account_circle_sharp),
      ],
    );
  }
}
