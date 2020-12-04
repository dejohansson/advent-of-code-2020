re='^([1-9]|1[0-9]|2[0-5])$'
if ! [[ $1 =~ $re ]] ; then
   echo "error: $1 is not a day in December :)" >&2; exit 1
fi

if ! [[ -d "./day-$1" ]] ; 
then
    cargo new "day-$1";
    cp -a "./day-template/." "./day-$1/";
else
    echo "error: './day-$1' already exists!" >&2; exit 1
fi