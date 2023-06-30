type CurOutcome<S extends State> = Moves<S> extends never
  ? IsCheck<S["pieces"], S["toMove"]> extends true
    ? `Checkmate - ${Other<S["toMove"]>} Wins!`
    : "Draw by stalemate."
  : "Ongoing.";
