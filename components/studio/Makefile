HAB_BLDR_CHANNEL ?= stable

export HAB_INTERNAL_BLDR_CHANNEL = ${HAB_BLDR_CHANNEL}
export CI_OVERRIDE_CHANNEL = ${HAB_BLDR_CHANNEL}
export HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL = ${HAB_BLDR_CHANNEL}
export HAB_STUDIO_SECRET_CI_OVERRIDE_CHANNEL = ${HAB_BLDR_CHANNEL}

test: studio-from-package studio-from-source

studio-from-package:
	test/studio-from-package/test.sh 

studio-from-source: 
	test/studio-from-source/test.sh

