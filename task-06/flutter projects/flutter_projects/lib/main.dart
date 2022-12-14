import 'package:flame/game.dart';
import 'package:flutter/material.dart';
import 'rabbit_game.dart';
import 'helpers/navigation_keys.dart';

void main() {
  final game = RabbitGame();
  runApp(
    MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        body: Stack(
          children: [
            GameWidget(
              game: game,
            ),
            Align(
              alignment: Alignment.bottomLeft,
              child: NavigationKeys(
                onDirectionChanged: game.onArrowKeyChanged,
              ),
            ),
          ],
        ),
      ),
    ),
  );
}
