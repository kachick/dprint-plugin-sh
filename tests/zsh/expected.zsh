#!/usr/bin/env zsh

typeset -a list=(apple banana cherry)

repeat 3 {
	print -l ${(U)list}
}
