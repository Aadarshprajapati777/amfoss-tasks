import 'package:flame/components.dart';
import 'package:flame/game.dart';
import 'package:flutter/material.dart';
import 'package:flame/flame.dart';

void main() {
  print("starting from main");
  runApp(GameWidget(game: MyGame()));
}

class MyGame extends FlameGame {
  late final SpriteComponent bunny;
  late final SpriteComponent background;
  @override
  Future<void> onLoad() async {
    final bgsprite = await Sprite.load('background.png');
    final bgsize = Vector2.all(600.0);
    final background = SpriteComponent(size: bgsize, sprite: bgsprite);

    // screen coordinates
    background.position = Vector2(0.0, 0.0);
    background.angle = 0;
    add(background);

    final bunnysprite = await Sprite.load('bunny.png');
    final bunnysize = Vector2.all(128.0);
    final bunny = SpriteComponent(size: bunnysize, sprite: bunnysprite);

    // screen coordinates
    bunny.position = Vector2(0.0, 200.0);
    bunny.angle = 0;
    add(bunny);
  }
}
