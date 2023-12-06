function Get-Params() {
  param($line)
  return $line.Substring($line.IndexOf(':') + 1).Split(' ').Where({ '' -ne $_ })
}
function Get-Races {
  param (
    $content
  )
  
  $Times = Get-Params $content[0]
  $Distances = Get-Params $content[1]

  $i = 0
  $Races = $Times | ForEach-Object {
    $Race = @{
      Time=[int]$_; 
      Distance=[int]$Distances[$i]
    }
    $i++
    return $Race
  }

  return $Races
}

function Get-CombinedRace {
  param (
    $content
  )
  
  $Times = Get-Params $content[0]
  $Distances = Get-Params $content[1]

  $Time = $Times -join ''
  $Distance = $Distances -join ''
  
  $Race = @{
    Time=[long]$Time; 
    Distance=[long]$Distance
  }
  
  return $Race
}
function Get-Winners() {
  param($Race)
  $start = [Math]::Floor(($Race.Time - [Math]::Sqrt([Math]::Pow($Race.Time,2) - 4 * $Race.Distance + 1)) / 2 )
  $end = [Math]::Ceiling(($Race.Time + [Math]::Sqrt([Math]::Pow($Race.Time,2) - 4 * $Race.Distance + 1)) / 2) - 1

  return ($end - $start)
}

function Invoke-PartOne() {
  param($content)

  $Races = Get-Races $content

  $WinnersList = @()
  foreach ($Race in $Races) {
    $Winners = Get-Winners $Race
    if ($Winners -ne 0) {
      $WinnersList += $Winners
    }
  }
  $Command = $WinnersList -join "*";
  $Result = Invoke-Expression -Command $Command;
  Write-Host $Result
}
function Invoke-PartTwo() {
  param($content)

  $Race = Get-CombinedRace $content
  $Winners = Get-Winners $Race

  Write-Host $Winners
}

$content = Get-Content .\input.txt 
Invoke-PartOne $content
Invoke-PartTwo $content