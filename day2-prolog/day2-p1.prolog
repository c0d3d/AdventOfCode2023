:- dynamic(game_possible/1).
:- use_module(library(yall)).

atomized_color(In, (Num, Color)) :-
    split_string(In, " ", "", [NumStr, ColorStr]),
    atom_string(Color, ColorStr),
    number_string(Num, NumStr).

most_of_color([], Acc, Acc).
most_of_color([ (Num, red) | Rest], (RAcc, GAcc, BAcc), Out) :-
    RNxt is max(Num, RAcc),
    most_of_color(Rest, (RNxt, GAcc, BAcc), Out).
most_of_color([ (Num, green) | Rest], (RAcc, GAcc, BAcc), Out) :-
    GNxt is max(Num, GAcc),
    most_of_color(Rest, (RAcc, GNxt, BAcc), Out).
most_of_color([ (Num, blue) | Rest], (RAcc, GAcc, BAcc), Out) :-
    BNxt is max(Num, BAcc),
    most_of_color(Rest, (RAcc, GAcc, BNxt), Out).

parse_runs([], RunSums, RunSums).
parse_runs([Nxt | Rest], (RAcc, GAcc, BAcc), RunSums) :-
    split_string(Nxt, ",", " ", Splits),
    maplist(atomized_color, Splits, AtomizedSplits),
    most_of_color(AtomizedSplits, (0,0,0), (RMost, GMost, BMost)),
    RNxt is max(RAcc, RMost),
    GNxt is max(GAcc, GMost),
    BNxt is max(BAcc, BMost),
    parse_runs(Rest, (RNxt, GNxt, BNxt), RunSums).

parse_game(Line, Id) :-
    split_string(Line, ":", "", [GameAndId, RestLine]),
    sub_string(GameAndId, 5, _, 0, IdStr),
    number_string(Id, IdStr),
    split_string(RestLine, ";", "", Runs),
    parse_runs(Runs, (0,0,0), RunTotals),
    (RMax, GMax, BMax) = RunTotals,
    format("Possible? ~w ~w ~w ~w~n", [Id, RMax, GMax, BMax]),
    RMax =< 12, GMax =< 13, BMax =< 14 -> assertz(game_possible(Id)).

parse_and_mark(Stream, Acc, Ids) :-
    read_line_to_string(Stream, NxtLine),
    NxtLine \== end_of_file
    -> ( parse_game(NxtLine, Id),
         parse_and_mark(Stream, [Id | Acc], Ids)
       ).

possible(Id, Out) :-
    game_possible(Id),
    Out is Id.

run_comp(Stream, Sum) :-
    parse_and_mark(Stream, [], IdList),
    convlist(possible, IdList, Filtered),
    format("Sum: ~p~n", [IdList]),
    foldl(plus, IdList, 0, Filtered),
    format("Sum: ~d ~p~n", [OutSum, Filtered]),
    Sum = OutSum, !.

run(Sum) :-
    setup_call_cleanup(
        open("input.txt", read, ReadStream, []),
        run_comp(ReadStream, Sum),
        close(ReadStream)).
