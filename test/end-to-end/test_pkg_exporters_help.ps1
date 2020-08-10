
Describe "`hab pkg export` executes the correct external command" {
    It "container exporter help" {
        $out = hab pkg export container --help
        $LastExitCode | Should -Be 0
        "Creates a container image from a set of Habitat packages (and optionally pushes to a remote repository)" | Should -BeIn $out

        $out = hab pkg export docker --help
        $LastExitCode | Should -Be 0
        "Creates a container image from a set of Habitat packages (and optionally pushes to a remote repository)" | Should -BeIn $out
    }

    It "cf exporter help" {
        $out = hab pkg export cf --help
        $LastExitCode | Should -Be 0
        "Habitat Package CFize - Create a Cloud Foundry ready Docker image from a given package." | Should -BeIn $out
    }

    It "mesos exporter help" {
        # The mesos exporter is only available on linux
        if ($IsLinux) {
            $out = hab pkg export mesos --help
            $LastExitCode | Should -Be 0
            "Habitat Package Mesosize - Create a Mesos application from a set of Habitat packages" | Should -BeIn $out
        } else {
            hab pkg export mesos --help
            $LastExitCode | Should -Be 1
        }
    }

    It "tar exporter help" {
        $out = hab pkg export tar --help
        $LastExitCode | Should -Be 0
        "Creates a tar package from a Habitat package" | Should -BeIn $out
    }

    It "`hab pkg export` with bad exporter" {
        hab pkg export a_bad_exporter --help
        $LastExitCode | Should -Be 1
    }
}

