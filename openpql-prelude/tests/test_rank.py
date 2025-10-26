import pytest

from opql_prelude import Rank


def test_variants():
    assert hasattr(Rank, "R2")
    assert hasattr(Rank, "R3")
    assert hasattr(Rank, "R4")
    assert hasattr(Rank, "R5")
    assert hasattr(Rank, "R6")
    assert hasattr(Rank, "R7")
    assert hasattr(Rank, "R8")
    assert hasattr(Rank, "R9")
    assert hasattr(Rank, "RT")
    assert hasattr(Rank, "RJ")
    assert hasattr(Rank, "RQ")
    assert hasattr(Rank, "RK")
    assert hasattr(Rank, "RA")


def test_eq():
    assert Rank.R2 == Rank.R2
    assert Rank.R2 != Rank.R3


def test_ord():
    arr = [
        Rank.R2,
        Rank.R3,
        Rank.R4,
        Rank.R5,
        Rank.R6,
        Rank.R7,
        Rank.R8,
        Rank.R9,
        Rank.RT,
        Rank.RJ,
        Rank.RQ,
        Rank.RK,
        Rank.RA,
    ]

    assert sorted(arr) == arr


def test_hash():
    assert hash(Rank.RA) == hash(Rank.RA)
    assert hash(Rank.R2) == hash(Rank.R2)


def test_repr_str():
    assert repr(Rank.R2) == "Rank.R2"
    assert str(Rank.RK) == "K"


def test_from_str():
    assert Rank.from_str("2") == Rank.R2
    assert Rank.from_str("3") == Rank.R3
    assert Rank.from_str("4") == Rank.R4
    assert Rank.from_str("5") == Rank.R5
    assert Rank.from_str("6") == Rank.R6
    assert Rank.from_str("7") == Rank.R7
    assert Rank.from_str("8") == Rank.R8
    assert Rank.from_str("9") == Rank.R9
    assert Rank.from_str("T") == Rank.RT
    assert Rank.from_str("J") == Rank.RJ
    assert Rank.from_str("Q") == Rank.RQ
    assert Rank.from_str("K") == Rank.RK
    assert Rank.from_str("A") == Rank.RA

    with pytest.raises(ValueError):
        Rank.from_str("AA")
