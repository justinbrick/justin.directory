import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import 'package:oidc/oidc.dart';
import 'package:oidc_web_core/oidc_web_core.dart';
import 'package:provider/provider.dart';
import 'package:async/async.dart';

final authLogger = Logger('directory.justin.auth');

const clientId = '11ec9395-8c5d-4ac7-9bc2-f4505e7053cf';
final redirectLink = Uri.parse(
  kDebugMode
      ? 'http://localhost:8080/redirect.html'
      : 'https://justin.directory/auth',
);

final userManager = OidcUserManager.lazy(
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

final managerInit = AsyncMemoizer<void>();
Future<void> initializeManager() {
  return managerInit.runOnce(() async {
    userManager.userChanges().listen((event) {
      authLogger.info('User changed: ${event?.claims.toJson()}');
    });
    await userManager.init();
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
  }

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
  @override
  Widget build(BuildContext context) {
    final authState = context.watch<AuthenticationState>();
    if (authState.user == null) {
      return const Icon(Icons.account_circle_outlined);
    }
    return Consumer<AuthenticationState>(
      builder: (context, value, child) {
        return const Icon(Icons.account_circle);
      },
    );
  }
}
