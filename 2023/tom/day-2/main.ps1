function Test-Possible {
  param (
    $line
  )

  $redMatches = [regex]::Matches($line, '([0-9]+) red')
  foreach ($match in $redMatches) {
    $total = [int]$match.Groups[1].Value
    if ($total -gt 12) {
      return $false
    }
  } 
  $greenMatches = [regex]::Matches($line, '([0-9]+) green')
  foreach ($match in $greenMatches) {
    $total = [int]$match.Groups[1].Value
    if ($total -gt 13) {
      return $false
    }
  } 
  $blueMatches = [regex]::Matches($line, '([0-9]+) blue')
  foreach ($match in $blueMatches) {
    $total = [int]$match.Groups[1].Value
    if ($total -gt 14) {
      return $false
    }
  } 

  return $true
}

function Get-Power {
  param (
    $line
  )

  $redMatches = [regex]::Matches($line, '([0-9]+) red')
  $maxRed = 0
  foreach ($match in $redMatches) {
    $total = [int]$match.Groups[1].Value
    $total = [int]$match.Groups[1].Value
    if ($total -gt $maxRed) {
      $maxRed = $total
    }
  } 
  $greenMatches = [regex]::Matches($line, '([0-9]+) green')
  $maxGreen = 0
  foreach ($match in $greenMatches) {
    $total = [int]$match.Groups[1].Value
    if ($total -gt $maxGreen) {
      $maxGreen = $total
    }
  } 
  $blueMatches = [regex]::Matches($line, '([0-9]+) blue')
  $maxBlue = 0
  foreach ($match in $blueMatches) {
    $total = [int]$match.Groups[1].Value
    if ($total -gt $maxBlue) {
      $maxBlue = $total
    }
  } 
  return $maxRed * $maxGreen * $maxBlue
}

function Invoke-PartOne() {
  param($content)
  $counter

  foreach ($line in $content) {
    if (Test-Possible $line) {
      $counter += [int][regex]::Matches($line,'Game ([0-9]+)').Groups[1].Value
    }
  }

  Write-Host $counter
}

function Invoke-PartTwo() {
  param($content)
  $counter

  foreach ($line in $content) {
    $counter += Get-Power $line
  }

  Write-Host $counter
}


$content = Get-Content .\input.txt
Invoke-PartOne $content
Invoke-PartTwo $content