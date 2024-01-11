#!/usr/bin/env bash

day=$1

tac .vscode/launch.json | sed '1,2d' | tac > .vscode/launch.head
cat <<EOF >> .vscode/launch.head
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable '${day}'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=${day}",
                    "--package=advent2023"
                ],
                "filter": {
                    "name": "${day}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "\${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable '${day}'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=${day}",
                    "--package=advent2023"
                ],
                "filter": {
                    "name": "${day}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "\${workspaceFolder}"
        },
    ]
}
EOF
mv .vscode/launch.head .vscode/launch.json 
sed -e"s/DAY/$day/" < src/bin/template.rs > src/bin/${day}.rs
git add src/bin/${day}.rs input/${day}.txt