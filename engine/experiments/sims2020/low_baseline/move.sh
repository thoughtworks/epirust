
for DIR in *
do
	[ ! -d ${DIR} ] && continue
	DIR_RUN=run_${DIR}
	mv $DIR/run_1  ${DIR_RUN}
done
