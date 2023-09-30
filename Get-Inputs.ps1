# Take the session token as a mandatory parameter, and take the year and day as optional parameters.
# If the year and day are not specified, then get all inputs for all years and days.
# If the year is specified, but the day is not, then get all inputs for the specified year.

param (
    [Parameter(Mandatory = $true)]
    [string]$SessionToken,

    [Parameter(Mandatory = $false)]
    [int]$Year,

    [Parameter(Mandatory = $false)]
    [int]$Day
)

$ScriptPath = Split-Path -Parent $MyInvocation.MyCommand.Definition

$BaseUrl = "https://adventofcode.com"
$Headers = @{
    "Cookie" = "session=$SessionToken"
}

$StartYear = if ($Year) { $Year } else { 2015 }
$EndYear = if ($Year) { $Year } else { 2022 }

$StartDay = if ($Day) { $Day } else { 1 }
$EndDay = if ($Day) { $Day } else { 25 }

for ($i = $StartYear; $i -le $EndYear; $i++) {
    $YearInputsDirectoryPath = "$ScriptPath\inputs\$i"
    New-Item -ItemType Directory -Force -Path $YearInputsDirectoryPath

    for ($j = $StartDay; $j -le $EndDay; $j++) {
        $Url = "$BaseUrl/$i/day/$j/input"
        $OutputFile = "$YearInputsDirectoryPath\$j.txt"
        Invoke-WebRequest -Uri $Url -Headers $Headers -OutFile $OutputFile
    }
}
