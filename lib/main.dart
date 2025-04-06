import 'package:flutter/material.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';
import 'package:go_router/go_router.dart';
import 'package:portfolio/auth.dart';
import 'package:provider/provider.dart';

void main() {
  usePathUrlStrategy();

  runApp(const MyApp());
}

final _router = GoRouter(
  routes: [
    ShellRoute(
      builder: (context, state, child) => NavigationLayout(child: child),
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const MyHomePage(title: MyApp.title),
        ),
      ],
    ),
  ],
  errorBuilder: (context, state) => const MyHomePage(title: MyApp.title),
);

class MyApp extends StatelessWidget {
  static const String title = 'justin.directory';
  static const Color seedColor = Color.fromARGB(255, 50, 174, 181);
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return AuthenticationLayout(
      child: MaterialApp.router(
        title: title,
        darkTheme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.fromSeed(
            seedColor: seedColor,
            brightness: Brightness.dark,
          ),
        ),
        theme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.fromSeed(seedColor: seedColor),
        ),
        routerConfig: _router,
      ),
    );
  }
}

class NavigationLayout extends StatefulWidget {
  const NavigationLayout({super.key, required this.child});

  final Widget child;

  @override
  State<NavigationLayout> createState() => _NavigationLayoutState();
}

class _NavigationLayoutState extends State<NavigationLayout> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(
        title: const Text(MyApp.title),
        backgroundColor: theme.colorScheme.inversePrimary,
        actions: [Icon(Icons.account_circle)],
        actionsPadding: const EdgeInsets.all(8),
        actionsIconTheme: const IconThemeData(size: 32),
      ),
      body: Row(
        children: [
          NavigationRail(
            backgroundColor: theme.colorScheme.surfaceBright,
            selectedIconTheme: IconThemeData(color: theme.colorScheme.primary),
            selectedIndex: _selectedIndex,
            onDestinationSelected: (int index) {
              setState(() {
                _selectedIndex = index;
              });
            },
            labelType: NavigationRailLabelType.all,
            destinations: const [
              NavigationRailDestination(
                icon: Icon(Icons.home),
                label: Text('Home'),
              ),
              NavigationRailDestination(
                icon: Icon(Icons.settings),
                label: Text('Settings'),
              ),
            ],
          ),
          Expanded(child: widget.child),
        ],
      ),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          const Text('You have pushed the button this many times:'),
        ],
      ),
    );
  }
}
