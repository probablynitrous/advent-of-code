function Get-Strength {
  param (
    $value,
    $isJoker
  )
  $testStr = @()
  if ($isJoker) {
    [char[]]$strength = @("A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J")
  } else {
    [char[]]$strength = @("A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2")
  }
  foreach ($character in [char[]]$value) {
    [string]$charStrength = ($strength.Count - $strength.IndexOf($character))
    $charStrength = $charStrength.PadLeft(2, "0")
    $testStr += $charStrength
  }

  return $testStr,$strength
}

function Get-MaxValue() {
  param($value)
  $uniqueChars = @()
  $currLength = $value.Length
  $maxLength = 0
  foreach ($character in [char[]]$value) {
    $value = $value -replace $character, ""
    $newLength = $value.Length
    $length = $currLength - $newLength
    if (($length -gt $maxLength) -and ($length -ne 1)) {
      $maxLength = $length
    }
    if (-not $uniqueChars.Contains($character)) {
      $uniqueChars += $character
    }
    $currLength = $newLength
  }
  if (($maxLength -eq 2) -and ($uniqueChars.Length -eq 4)) {
    $maxLength = 1
  }
  if ($maxLength -gt 3) {
    $maxLength += 1
  }
  if (($maxLength -eq 3) -and ($uniqueChars.Length -eq 2)) {
    $maxLength = 4
  }
  return $maxLength
}

function Get-Characters {
  param (
    $value,
    $isJoker
  )

  $maxLength = 0
  
  if ($isJoker) {
    $testStr,$strength = Get-Strength $value -isJoker $isJoker
    if ($value -match "J") {
      foreach ($level in $strength[0..($strength.Length - 2)]) {
        [string]$newValue = ([char[]]$value -replace [char]"J", $level) -join ''
        $length = Get-MaxValue $newValue
        if ($length -gt $maxLength) {
          $maxLength = $length
        }
      }
    } else {
      $maxLength = Get-MaxValue $value
    }
  } else {
    $testStr,$strength = Get-Strength $value -isJoker $false
    $maxLength = Get-MaxValue $value
  }

  $characters = @{
    Type    = $maxLength
    TestStr = $testStr
  }

  return $characters
}
function Invoke-PartOne() {
  param($content)
  $hands = @()
  foreach ($line in $content) {
    $handValues = $line.Split(' ')
    $value = $handValues[0]
    $characters = Get-Characters $value -IsJoker $false
    $hands += @{
      Value         = $value;
      Bid           = $handValues[1];
      Type          = $characters.Type
      CharactersStr = $characters.TestStr
    }
  }

  $hands = $hands | ForEach-Object { 
    New-Object object | Add-Member -NotePropertyMembers $_ -PassThru 
  } | Sort-Object -Property @{Expression = { $_.Type }; Descending = $false }, @{Expression = { $_.CharactersStr } ; Descending = $false }
  
  $i = 0
  $total = 0
  foreach ($hand in $hands) {
    $i++
    $total += [int]$hand.bid * $i
  }
  Write-Host $total
}
function Invoke-PartTwo() {
  param($content)

  $hands = @()
  foreach ($line in $content) {
    $handValues = $line.Split(' ')
    $value = $handValues[0]
    $characters = Get-Characters $value -IsJoker $true
    $hands += @{
      Value         = $value;
      Bid           = $handValues[1];
      Type          = $characters.Type
      CharactersStr = $characters.TestStr
    }
  }

  $hands = $hands | ForEach-Object { 
    New-Object object | Add-Member -NotePropertyMembers $_ -PassThru 
  } | Sort-Object -Property @{Expression = { $_.Type }; Descending = $false }, @{Expression = { $_.CharactersStr } ; Descending = $false }

  $i = 0
  $total = 0
  foreach ($hand in $hands) {
    $i++
    $total += [int]$hand.bid * $i
  }

  Write-Host $total
}

$content = Get-Content .\input.txt 
Invoke-PartOne $content
Invoke-PartTwo $content