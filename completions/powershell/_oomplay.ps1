
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
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Start playground')
            [CompletionResult]::new('stop', 'stop', [CompletionResultType]::ParameterValue, 'Stop playground')
            [CompletionResult]::new('reset', 'reset', [CompletionResultType]::ParameterValue, 'Reset playground')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Start or reset playground')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Output shell completion code')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'oomplay;start' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;stop' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;reset' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;init' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'config file path')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'oomplay;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
