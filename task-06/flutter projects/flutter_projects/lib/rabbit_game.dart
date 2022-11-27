import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/game.dart';
import 'package:flame/input.dart';
import 'package:flutter/services.dart';
import 'rabbit_player.dart';
import 'rabbit_world.dart';
import 'package:flutter/material.dart';

import 'helpers/directions.dart';

class RabbitGame extends FlameGame with KeyboardEvents {
  RabbitPlayer _rabbitPlayer = RabbitPlayer();
  RabbitWorld _rabbitWorld = RabbitWorld();

  @override
  Future<void> onLoad() async {
    super.onLoad();
    await add(_rabbitWorld);
    await add(_rabbitPlayer);
    _rabbitPlayer.position = _rabbitWorld.size / 1.5;
    camera.followComponent(_rabbitPlayer,
        worldBounds:
            Rect.fromLTRB(0, 0, _rabbitWorld.size.x, _rabbitWorld.size.y));
  }

  onArrowKeyChanged(Direction direction) {
    _rabbitPlayer.direction = direction;
  }
}
