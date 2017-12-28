use position;
pub type Score = u32;

fn perft(depth: u32, position: &mut position::Position, us: position::player::Player, enemy: position::player::Player) -> u32
{
    if depth <= 0
    {
        return 1;
    }

    let mut nodes = 0;
    let movelist = position.generate_move_list(us, enemy);
    for i in 0..movelist.len
    {
        /*let mut position_n = position.clone();
        position_n.make_move(&movelist[i], us, enemy);*/
        let backup_en_passant_castling = position.make_move(&movelist[i], us, enemy);
        if !position.is_check_unkown_kings_index(us, enemy)
        {
            nodes += perft(depth - 1, position, enemy, us);
        }
        position.undo_move(&movelist[i], backup_en_passant_castling, us, enemy);
    }
    nodes
}

pub fn start_perft(mut position: position::Position, depth: u32) -> u32
{
    let enemy = position::player::switch_player(position.whose_move);
    let us = position.whose_move;
    perft(depth, &mut position, us, enemy)
}
/*
fn nega_max(us: position::player::Player, enemy: position::player::Player, orig_position, int depth, Score alpha, Score beta) -> Score
{
  if(depth<=0)
  {
    return evaluation(us, enemy, origPosition);
  }
  nodes++;
  Score currentScore;
  Bitboard currentZobristKey;
  int numberLegalMoves = 0;
  PositionList newPositions;
  generateAllMoves(us, enemy, origPosition, newPositions);
  sortMoves(us, enemy, origPosition, newPositions, killerMove[depth]);
  for(int i = 0; i < newPositions.size; i++)
  {
    if(isKingInCheck(us, enemy, newPositions[i]))
    {
      continue;
    }
    numberLegalMoves++;
    currentZobristKey = getZobristKey(newPositions[i]);

    if((tpTable[currentZobristKey%TP_TABLE_SIZE].zobristKey == currentZobristKey) && (tpTable[currentZobristKey%TP_TABLE_SIZE].searchedDepth >= depth))
    {
      currentScore = tpTable[currentZobristKey%TP_TABLE_SIZE].searchedScore;
    }
    else
    {
      currentScore = -negaMax(enemy, us, newPositions[i], depth - 1, -beta, -alpha);
      tpTable[currentZobristKey%TP_TABLE_SIZE].zobristKey = currentZobristKey;
      tpTable[currentZobristKey%TP_TABLE_SIZE].searchedDepth = depth;
      tpTable[currentZobristKey%TP_TABLE_SIZE].searchedScore = currentScore;
    }


    if(alpha < currentScore)
    {
      alpha = currentScore;
      if(beta <= currentScore)
      {
        killerMove[depth][1] = killerMove[depth][0];
        killerMove[depth][0] = newPositions[i].lastMove;
        break;
      }
    }
  }
  //check for MATE or STALEMATE
  if(numberLegalMoves == 0)
  {
    if(isKingInCheck(us, enemy, origPosition))
    {
      alpha = -SCORE_MATE-depth;
    }
    else
    {
      alpha = 0;
    }
  }
  return alpha;
}*/
