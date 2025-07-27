#!/usr/bin/env bash

trim() {
    local var="$*"
    var="${var##+([[:space:]])}"
    var="${var%%+([[:space:]])}"
    echo "$var"
}

camel_to_snake() {
  camel_case_string="$1"
  snake_case_string=""
  for (( i=0; i<${#camel_case_string}; i++ )); do
    char="${camel_case_string:$i:1}"
    if [[ "$char" =~ [A-Z] ]]; then
      snake_case_string+="_${char,,}"
    else
      snake_case_string+="$char"
    fi
  done

  echo "${snake_case_string:1}"

}

gen_card() {
  for s in {S,H,D,C}; do
    for r in {2,3,4,5,6,7,8,9,T,J,Q,K,A}; do
      echo "const C_${r}${s}: Self = Self(1 << OFFSET_${r} << OFFSET_${s});"
    done
  done
}

gen_straight() {
  suffix=("TJQKA" "9TJQK" "89TJQ" "789TJ" "6789T" "56789" "45678" "34567" "23456")

  for c in ${suffix[@]}; do
    echo "if STRAIGHT_$c & ranks == STRAIGHT_$c { mk_ranking(STRAIGHT, mk_flags_rank(0, STRAIGHT_$c)) }"
  done
}

gen_comb() {
  for i in $(seq 0 12 | tac); do
    echo "else if nc2(${i}) <= idx { [u8_to_rank(idx - nc2(${i})), u8_to_rank(${i})]}"
  done
}

gen_data() {
  types=(BoardRange Boolean Card CardCount Double Equity FlopHandCategory Fraction HandType HiRating Integer Long LoRating Numeric Player PlayerCount Range Rank RankSet Street String)

  for t in ${types[@]}; do
content=$(cat <<EOF
      #[derive(Debug, Clone)]
      pub struct PQL$t {}
EOF
)

    filename="pql_$(camel_to_snake $t).rs"

    # echo $content > $filename

    echo "mod pql_$(camel_to_snake $t);"
    echo "pub use pql_$(camel_to_snake $t)::PQL$t;"
  done
}


gen_functions() {

functions_def="
┌────────────────────────────┬───────────────────────────────────────┬───────────────────┐
│       Function Name        │            Argument Types             │    Return Type    │
├────────────────────────────┼───────────────────────────────────────┼───────────────────┤
│ bestHiRating               │ TPlayer, TStreet                      │ TBoolean          │
│ bestLoRating               │ TPlayer, TStreet                      │ TBoolean          │
│ boardAllowsMadeLo          │ TStreet                               │ TBoolean          │
│ boardHasOneDistinctLoCard  │ TStreet                               │ TBoolean          │
│ boardHasTwoDistinctLoCards │ TStreet                               │ TBoolean          │
│ boardInRange               │ TBoardRange                           │ TBoolean          │
│ boardLoCardCount           │ TStreet                               │ TCardCount        │
│ boardRanks                 │ TStreet                               │ TRankSet          │
│ boardSuitCount             │ TStreet                               │ TCardCount        │
│ cardsPlay                  │ TPlayer, TStreet                      │ TCardCount        │
│ duplicatedBoardRanks       │ TStreet                               │ TRankSet          │
│ duplicatedHandRanks        │ TPlayer, TStreet                      │ TRankSet          │
│ equity                     │ TPlayer, TStreet                      │ TEquity           │
│ exactFlopHandCategory      │ TPlayer, TFlopHandCategory            │ TBoolean          │
│ exactHandType              │ TPlayer, TStreet, THandType           │ TBoolean          │
│ fiveCardHiHandNumber       │ TPlayer, TStreet                      │ TInteger          │
│ flopHandCategory           │ TPlayer                               │ TFlopHandCategory │
│ flushingBoard              │ TStreet                               │ TBoolean          │
│ fourFlush                  │ TPlayer, TStreet                      │ TBoolean          │
│ fractionalRiverEquity      │ TPlayer                               │ TFraction         │
│ handBoardIntersections     │ TPlayer, TStreet                      │ TCardCount        │
│ handRanking                │ TPlayer                               │ THandRanking      │
│ handRankingFor             │ TPlayer, TString                      │ THandRanking      │
│ handRanks                  │ TPlayer, TStreet                      │ TRankSet          │
│ handType                   │ TPlayer, TStreet                      │ THandType         │
│ hasSecondBoardRank         │ TPlayer, TStreet                      │ TBoolean          │
│ hasTopBoardRank            │ TPlayer, TStreet                      │ TBoolean          │
│ hiRating                   │ TPlayer, TStreet                      │ THiRating         │
│ HvHEquity                  │ TPlayer, TStreet                      │ TEquity           │
│ HvPerceivedRangeEquity     │ TPlayer, TStreet, TRange              │ TEquity           │
│ HvREquity                  │ TPlayer, TStreet                      │ TEquity           │
│ inRange                    │ TPlayer, TRange                       │ TBoolean          │
│ intersectingHandRanks      │ TPlayer, TStreet                      │ TRankSet          │
│ loRating                   │ TPlayer, TStreet                      │ TLoRating         │
│ madeLo                     │ TPlayer, TStreet                      │ TBoolean          │
│ maxRank                    │ TRankSet                              │ TRank             │
│ minEquity                  │ TPlayer, TStreet, TDouble             │ TBoolean          │
│ minFlopHandCategory        │ TPlayer, TFlopHandCategory            │ TBoolean          │
│ minHandType                │ TPlayer, TStreet, THandType           │ TBoolean          │
│ minHiRating                │ TPlayer, TStreet, THiRating           │ TBoolean          │
│ minHvHEquity               │ TPlayer, TStreet, TDouble             │ TBoolean          │
│ minHvPerceivedRangeEquity  │ TPlayer, TStreet, TRange, TDouble     │ TBoolean          │
│ minHvREquity               │ TPlayer, TStreet, TDouble             │ TBoolean          │
│ minLoRating                │ TPlayer, TStreet, TLoRating           │ TBoolean          │
│ minOutsToHandType          │ TPlayer, TStreet, THandType, TInteger │ TBoolean          │
│ minRank                    │ TRankSet                              │ TRank             │
│ monotoneBoard              │ TStreet                               │ TBoolean          │
│ nonIntersectingHandRanks   │ TPlayer, TStreet                      │ TRankSet          │
│ nthRank                    │ TInteger, TRankSet                    │ TRank             │
│ nutHi                      │ TPlayer, TStreet                      │ TBoolean          │
│ nutHiForHandType           │ TPlayer, TStreet                      │ TBoolean          │
│ nutHiOuts                  │ TPlayer, TStreet                      │ TCardCount        │
│ nutLo                      │ TPlayer, TStreet                      │ TBoolean          │
│ nutLoOuts                  │ TPlayer, TStreet                      │ TCardCount        │
│ outsToHandType             │ TPlayer, TStreet, THandType           │ TCardCount        │
│ overpair                   │ TPlayer, TStreet                      │ TBoolean          │
│ pairedBoard                │ TStreet                               │ TBoolean          │
│ pocketPair                 │ TPlayer                               │ TBoolean          │
│ rainbowBoard               │ TStreet                               │ TBoolean          │
│ rankCount                  │ TRankSet                              │ TCardCount        │
│ rateHiHand                 │ TString                               │ THiRating         │
│ rateLoHand                 │ TString                               │ TLoRating         │
│ riverCard                  │                                       │ TCard             │
│ riverEquity                │ TPlayer                               │ TEquity           │
│ scoops                     │ TPlayer                               │ TBoolean          │
│ straightBoard              │ TStreet                               │ TBoolean          │
│ threeFlush                 │ TPlayer, TStreet                      │ TBoolean          │
│ tiesHi                     │ TPlayer                               │ TBoolean          │
│ tiesLo                     │ TPlayer                               │ TBoolean          │
│ toCard                     │ TString                               │ TCard             │
│ toRank                     │ TString                               │ TRank             │
│ toString                   │ Type                                  │ TString           │
│ turnCard                   │                                       │ TCard             │
│ twoToneBoard               │ TStreet                               │ TBoolean          │
│ upCard                     │ TPlayer, TStreet                      │ TCard             │
│ winningHandType            │                                       │ THandType         │
│ winsHi                     │ TPlayer                               │ TBoolean          │
│ winsLo                     │ TPlayer                               │ TBoolean          │
└────────────────────────────┴───────────────────────────────────────┴───────────────────┘
"
  lines=$(echo "$functions_def" | sed '1,4d' | head -n -2 | sed 's/│/|/g' | sed 's/^| //' | sed 's/ |$//')
  lines=$(echo "$lines" | sed 's/^./\U&/')

  res_arg=()
  res_out=()
  res_from=()

  while IFS='|' read -r name args output; do
    name=$(trim $name)
    args=$(trim $args)
    output=$(trim $output)
    output=${output:1}
    readarray -t args < <(echo "$args" | sed 's/,[[:space:]]*/\n/g')

    mapped_args=()

    if [[ "${args[0]}" != "" ]]; then
      for el in "${args[@]}"; do
        mapped_args+=("PQLType::${el:1}")
      done
    fi

    res_out+=("Self::$name => PQLType::$output,")

    name_lower=$(echo $name | tr "[:upper:]" "[:lower:]")

    arm="\"${name_lower}\" => Ok(Self::${name}),"

    res_from+=("$arm")

    arg=""

    for a in "${mapped_args[@]}"; do
      arg+="${a},"
    done

    res_arg+=("Self::$name => smallvec![$arg],")

    echo "name: $name"
    echo "nargs: ${#mapped_args[@]}"
    echo "mapped_args: ${mapped_args[@]}"
    echo "output: $output"
    echo "------------------------------------"
  done < <(printf "$lines\n")

  echo "ARGS:"
  printf "%s\n" "${res_arg[@]}"

  #echo "OUTPUT TYPE:"
  #printf "%s\n" "${res_out[@]}"

  #echo "TRYFROM:"
  #printf "%s\n" "${res_from[@]}"
}

gen_functions

sleep 1
echo '-----'
