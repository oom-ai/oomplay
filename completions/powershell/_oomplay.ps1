
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'oomplay' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'oomplay'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'oomplay' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize playgrounds')
            [CompletionResult]::new('clear', 'clear', [CompletionResultType]::ParameterValue, 'Clean up playgrounds')
            [CompletionResult]::new('stop', 'stop', [CompletionResultType]::ParameterValue, 'Stop playgrounds')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Output shell completion code')
            break
        }
        'oomplay;init' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('redis', 'redis', [CompletionResultType]::ParameterValue, 'Redis store')
            [CompletionResult]::new('postgres', 'postgres', [CompletionResultType]::ParameterValue, 'Postgres store')
            [CompletionResult]::new('mysql', 'mysql', [CompletionResultType]::ParameterValue, 'Mysql store')
            break
        }
        'oomplay;init;redis' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;init;postgres' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;init;mysql' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;clear' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('redis', 'redis', [CompletionResultType]::ParameterValue, 'Redis store')
            [CompletionResult]::new('postgres', 'postgres', [CompletionResultType]::ParameterValue, 'Postgres store')
            [CompletionResult]::new('mysql', 'mysql', [CompletionResultType]::ParameterValue, 'Mysql store')
            break
        }
        'oomplay;clear;redis' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;clear;postgres' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;clear;mysql' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;stop' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('redis', 'redis', [CompletionResultType]::ParameterValue, 'Redis store')
            [CompletionResult]::new('postgres', 'postgres', [CompletionResultType]::ParameterValue, 'Postgres store')
            [CompletionResult]::new('mysql', 'mysql', [CompletionResultType]::ParameterValue, 'Mysql store')
            break
        }
        'oomplay;stop;redis' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;stop;postgres' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;stop;mysql' {
            [CompletionResult]::new('-P', 'P', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'port')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--password', 'password', [CompletionResultType]::ParameterName, 'password')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--database', 'database', [CompletionResultType]::ParameterName, 'database')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file path containing backends')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
