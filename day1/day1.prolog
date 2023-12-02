:- use_module(library(apply)).

is_word_in_list(StrChars, Word, Out) :-
    string_chars(Str, StrChars),
    sub_string(Str, Idx, _, _, Word),
    Out = (Word, Idx).

as_num("one", '1').
as_num("two", '2').
as_num("three", '3').
as_num("four", '4').
as_num("five", '5').
as_num("six", '6').
as_num("seven", '7').
as_num("eight", '8').
as_num("nine", '9').

best(_, [], Ans, Ans) :- !.
best(least, [(W, I) | Rest], (BestW, BestIdx), Ans) :-
    ( I < BestIdx -> (best(least, Rest, (W, I), Ans)))
    ; best(least, Rest, (BestW, BestIdx), Ans).
best(most, [(W, I) | Rest], (BestW, BestIdx), Ans) :-
    ( I > BestIdx -> (best(most, Rest, (W, I), Ans)) )
    ; best(most, Rest, (BestW, BestIdx), Ans).

get_word_digit(StrChars, OutDigit, most) :-
    convlist(
        is_word_in_list(StrChars),
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"],
        WordAndIndex),
    length(WordAndIndex, L),
    L > 0,
    best(most, WordAndIndex, (_, -1), (DigitWord, LastIdx)),
    LastIdx =:= 0,
    as_num(DigitWord, OutDigit).

get_word_digit(StrChars, OutDigit, least) :-
    convlist(
        is_word_in_list(StrChars),
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"],
        WordAndIndex),
    length(WordAndIndex, L),
    L > 0,
    best(least, WordAndIndex, (_, 199999999), (DigitWord, LastIdx)),
    LastIdx =:= 0,
    as_num(DigitWord, OutDigit).

first_digit([NxtChar | Rest], OutChar) :-
    ( char_type(NxtChar, digit), OutChar = NxtChar, ! )
    ; ( get_word_digit([NxtChar | Rest], OutDigit, least) -> OutChar = OutDigit, ! )
    ; first_digit(Rest, OutChar).

last_digit([NxtChar | Rest], OutChar) :-
    ( last_digit(Rest, RecurFirst) -> OutChar = RecurFirst, ! )
    ; ( char_type(NxtChar, digit), OutChar = NxtChar, ! )
    ; ( get_word_digit([NxtChar | Rest], OutDigit, most) -> OutChar = OutDigit, ! ).

first_and_last_digit(Str, First, Last) :-
    string_chars(Str, StrChars),
    first_digit(StrChars, First),
    last_digit(StrChars, Last).

calibration_value(Line, Value) :-
    first_and_last_digit(Line, First, Last),
    string_chars(ValueS, [First, Last]),
    number_string(Value, ValueS).

calibration_sum(Lines, OutSum) :-
    maplist(calibration_value, Lines, Outs),
    foldl(plus, Outs, 0, OutSum).


final_sum(Stream, Acc, FinalSum) :-
    read_line_to_string(Stream, NxtLine),
    NxtLine \== end_of_file
    -> ( calibration_value(NxtLine, LineTotal),
         NewAcc is Acc + LineTotal,
         final_sum(Stream, NewAcc, FinalSum)
       )
    ; FinalSum is Acc, !.


run(Sum) :-
    setup_call_cleanup(
        open("input.txt", read, ReadStream, []),
        final_sum(ReadStream, 0, Sum),
        close(ReadStream)).
