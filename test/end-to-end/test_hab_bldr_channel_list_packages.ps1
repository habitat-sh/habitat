Describe "hab bldr channel list-packages" {
    It "prints fully qualified identifiers and equivalent JSON for a public channel" {
        $plain = @(hab bldr channel list-packages core stable --limit 2)
        $LASTEXITCODE | Should -Be 0
        $plain.Count | Should -Be 2
        foreach ($ident in $plain) {
            $ident | Should -Match '^core/[^/]+/[^/]+/[^/]+$'
        }

        $json = hab bldr channel list-packages core stable --limit 2 --json
        $LASTEXITCODE | Should -Be 0
        $parsed = @($json | ConvertFrom-Json)
        $parsed.Count | Should -Be 2
        ($parsed -join "`n") | Should -Be ($plain -join "`n")
    }
}
