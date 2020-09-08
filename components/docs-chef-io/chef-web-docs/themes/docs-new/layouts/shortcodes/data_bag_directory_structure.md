All data bags are stored in the `data_bags` directory of the chef-repo.
This directory structure is understood by knife so that the full path
does not need to be entered when working with data bags from the command
line. An example of the `data_bags` directory structure:

    - data_bags
        -  admins
            -  charlie.json
            -  bob.json
            -  tom.json
        -  db_users
            -  charlie.json
            -  bob.json
            -  sarah.json
        -  db_config
            -  small.json
            -  medium.json
            -  large.json

where `admins`, `db_users`, and `db_config` are the names of individual
data bags and all of the files that end with `.json` are the individual
data bag items.