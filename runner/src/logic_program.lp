edge(0, 1).
edge(1, 2).
edge(2, 0).

% node(A) :- edge(A, _).
% node(A) :- edge(_, A).
connected(A, B) :- edge(A, B).
connected(A, C) :- connected(A, B), connected(B, C).

has_cycle :- connected(A, A).

#show has_cycle/0.
