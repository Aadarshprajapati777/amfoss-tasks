import 'package:flame/components.dart';
import 'package:flame/game.dart';
import 'package:flutter/material.dart';
import 'package:flame/flame.dart';

void main() {
  print("starting from main");
  runApp(GameWidget(game: MyGame()));
}

class MyGame extends FlameGame {
  SpriteComponent bunny = SpriteComponent();
  SpriteComponent background = SpriteComponent();
  final double charactersize = 200.0;

  @override
  Future<void> onLoad() async {
    await super.onLoad();
    final screenWidth = size.x;
    final screenHeight = size.y;

    // loading background
    add(background
      ..sprite = await loadSprite('background.png')
      ..size = size);

    // loading bunny
    bunny
      ..sprite = await loadSprite('bunny.png')
      ..size = Vector2(charactersize, charactersize)
      ..y = 100;

    add(bunny);
  }

  @override
  void update(double dt) {
    super.update(dt);
    if (bunny.x < size.x - charactersize) {
      bunny.x += 100 * dt;
    } else {
      if (bunny.y < size.y - charactersize) {
        bunny.y += 100 * dt;
      } else {
        if (bunny.y < size.y - charactersize) {
          bunny.y -= 100 * dt;
        }
      }
    }
  }
}
