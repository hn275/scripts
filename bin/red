#! /usr/bin/bash

# Toggling redshift (night light)

if pgrep -x redshift > /dev/null;then
	polybar-msg action "#redshift.hook.0"
	killall redshift
	exit 0 
else
	polybar-msg action "#redshift.hook.1"
	redshift &> /dev/null
fi
