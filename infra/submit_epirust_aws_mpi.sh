#!/bin/bash
#
# EpiRust
# Copyright (c) 2024  ThoughtWorks, Inc.
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
#


# get shared dir
# IFS=',' _shared_dirs=(${PCLUSTER_SHARED_DIRS})
# # _shared_dir=${_shared_dirs[0]}
# export _shared_dir=/efs
# _job_dir="${_shared_dir}/${AWS_BATCH_JOB_ID%#*}-${AWS_BATCH_JOB_ATTEMPT}"
# _exit_code_file="${_job_dir}/batch-exit-code"


#SBATCH --nodes=15
#SBATCH --ntasks-per-node=20
#SBATCH --cpus-per-task=2

if [[ "${AWS_BATCH_JOB_NODE_INDEX}" -eq  "${AWS_BATCH_JOB_MAIN_NODE_INDEX}" ]]; then

    echo "Hello I'm the main node $HOSTNAME! I run the mpi job!"
    echo "Running..."
    module load ${MPI_MODULE}

    EPIRUST_CONFIG_NAME="1m_100"
    CURRENT_DATE=$(date -u | sed -r 's/ +/::/g')
    SHARED_DIR=/home/ubuntu
    EPIRUST_OUTPUT_DIR=${SHARED_DIR}/${EPIRUST_CONFIG_NAME}_${CURRENT_DATE}

    mkdir -p ${EPIRUST_OUTPUT_DIR}

    EPIRUST_CONFIG=${SHARED_DIR}/epirust/engine/config/${EPIRUST_CONFIG_NAME}.json
    echo "Running ${EPIRUST_CONFIG}"

    echo "Nodelist: $SLURM_JOB_NODELIST"
    echo "CoerPerTask: $SLURM_CPUS_PER_TASK"

    export OMP_NUM_THREADS=$SLURM_CPUS_PER_TASK
    mpirun --map-by core --mca btl_tcp_if_include ens5 --mca orte_base_help_aggregate 0 --allow-run-as-root -n ${SLURM_NTASKS}  -x RUST_BACKTRACE=full ${SHARED_DIR}/epirust/target/release/engine-app -m mpi -c ${EPIRUST_CONFIG} -o ${EPIRUST_OUTPUT_DIR} -t ${SLURM_CPUS_PER_TASK}

    # Write exit status code
    echo "0" > "${_exit_code_file}"
    # Waiting for compute nodes to terminate
    sleep 30
else
    echo "Hello I'm the compute node $HOSTNAME! I let the main node orchestrate the mpi processing!"
    sleep 5
    echo $(ls -la "${_job_dir}")
    # Since mpi orchestration happens on the main node, we need to make sure the containers representing the compute
    # nodes are not terminated. A simple trick is to wait for a file containing the status code to be created.
    # All compute nodes are terminated by AWS Batch if the main node exits abruptly.
    while [ ! -f "${_exit_code_file}" ]; do
        sleep 2
    done
    exit $(cat "${_exit_code_file}")
fi