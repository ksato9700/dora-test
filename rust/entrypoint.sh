#!/bin/sh
dora up &&
    dora start /opt/dora/etc/dataflow.yml --name conversation
