import game_rs

game = game_rs.PyGameState()
print("Initial state:", game)

# Make some moves
moves = ["RIGHT", "RIGHT", "UP", "LEFT"]
for move in moves:
    game.make_move(move)
    print(f"After {move}: {game}")

# Undo last move
game.undo_move()
print("After undo:", game)
