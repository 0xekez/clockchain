set terminal svg

stats filename

set label gprintf("Mean = %g", STATS_mean) at graph 0.1, graph 0.1 front
set label gprintf("Stddev = %g", STATS_stddev) at graph 0.1, graph 0.15 front

set style fill transparent solid 0.2 noborder

plot filename with lines title "execution time"
