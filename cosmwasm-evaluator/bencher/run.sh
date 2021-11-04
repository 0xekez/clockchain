function die() {
    echo "shutting down.."

    pkill python3

    exit
}

set -e
set -o pipefail

python3 -m http.server 80 &

trap 'die' ERR SIGINT SIGTERM

while :
do
    ./measure.sh ekez >> all.data
    cat all.data | tail -1440 > day.data
    cat all.data | tail -60 > hour.data

    gnuplot -e "filename='all.data'" timeseries.plot > all.svg
    gnuplot -e "filename='day.data'" timeseries.plot > day.svg
    gnuplot -e "filename='hour.data'" timeseries.plot > hour.svg
done
