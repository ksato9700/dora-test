#!/bin/sh
dora up &&
    dora start /work/dataflow.yml --name conversation
