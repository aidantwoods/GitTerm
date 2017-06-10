<?php

/*
 * Settings
 */
const PS1            = '\[\033[m\]\u:__folder __status\[\033[m\]\$ ';
const FOLDER_DEFAULT = '\w';
const FOLDER_COLOR   = '\[\033[36;1m\]';
const STATUS_COLOR   = '\[\033[33;1m\]';
# order: 'modified', 'added', 'deleted', 'untracked'
const STATUS_SYMBOLS = '*+-?';

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

# strlen('/.git') === 5
const GIT_FOLDER_LEN = 5;

$gitInfo = `git status --porcelain 2> /dev/null || echo .`;
$isGit   = (".\n" !== $gitInfo);
$status  = '';

if ($isGit)
{
    $workDir = getWorkingDir();
    $status  = getGitStatus($gitInfo);
}

echo str_replace(
    ['__folder', '__status'],
    [FOLDER_COLOR . ($workDir ?? FOLDER_DEFAULT), STATUS_COLOR . $status],
    PS1
);

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

function getWorkingDir() : string
{
    $gitDir = substr(
        realpath(substr(`git rev-parse --git-dir`, 0, -1)),
        0,
        -GIT_FOLDER_LEN
    );

    $parentDirToGit = substr($gitDir, 0, strrpos($gitDir, '/')).'/';

    $workDir = realpath('');

    if (strpos($workDir, $parentDirToGit) === 0)
    {
        $workDir = substr_replace($workDir, '', 0, strlen($parentDirToGit));
    }

    return $workDir;
}

function getGitStatus(?string $gitInfo) : string
{
    if ( ! isset($gitInfo))
    {
        return '';
    }

    $gitInfo = explode("\n", $gitInfo);

    $gitInfo = array_map(
        function ($item)
        {
            return ($item[0] ?? '') . ($item[1] ?? '');
        },
        $gitInfo
    );

    $gitInfo          = implode('', $gitInfo);
    $n                = strlen($gitInfo);

    $states           = ['modified', 'added', 'deleted', 'untracked'];
    $gitCodes         = array_combine($states, ['MCRU', 'A', 'D', '?']);
    $statusIndicators = array_combine($states, str_split(STATUS_SYMBOLS));

    $state = array_map(
        function ($c) use ($gitInfo, $n)
        {
            return strcspn($gitInfo, $c) < $n;
        },
        $gitCodes
    );

    $state = array_keys(array_filter($state));

    $status = array_reduce(
        $state,
        function ($carry, $item) use ($statusIndicators)
        {
            return $carry.$statusIndicators[$item];
        },
        ''
    );

    return $status;
}
