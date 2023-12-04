function Get-Number() {
  param ($line, $pos, $index)  

  $isNum = $true
  do {
    $pos-- 
    $isNum = $line[$pos] -match '[0-9]'
  } while ($isNum)

  $value = [regex]::Matches($line.Substring($pos + 1), '([0-9]+)').Groups[1].Value
  $obj = [pscustomobject]@{Line = $index; Range = ($pos)..($pos + $value.Length - 1); Value = $value }
  return $obj
}

function Find-Adjacent {
  param ($content, $index, $pos, $numbers)

  if ($pos -gt 1) {
    if ($content[$index][$pos + 1] -match '[0-9]') {
      $numbers += (Get-Number -line $content[$index] -pos ($pos + 1) -index $index)
    }
  }
  if ($pos -lt $content[1].Length) {
    if ($content[$index][$pos - 1] -match '[0-9]') {
      $numbers += (Get-Number -line $content[$index] -pos ($pos - 1) -index $index)
    }
  }
  
  if ($index -gt 0) {
    if ($content[$index - 1][$pos] -match '[0-9]') {
      $numbers += (Get-Number -line $content[$index - 1] -pos $pos -index ($index - 1))
    }
    if ($pos -lt $content[1].Length) {
      if ($content[$index - 1][$pos + 1] -match '[0-9]') {
        $numbers += (Get-Number -line $content[$index - 1] -pos ($pos + 1) -index ($index - 1))
      }
    }
    
    if ($pos -gt 1) {
      if ($content[$index - 1][$pos - 1] -match '[0-9]') {
        $numbers += (Get-Number -line $content[$index - 1] -pos ($pos - 1) -index ($index - 1))
      }
    }
  } 
  
  if ($index -lt $content.Length) {
    if ($content[$index + 1][$pos] -match '[0-9]') {
      $numbers += (Get-Number -line $content[$index + 1] -pos $pos -index ($index + 1))
    }
    if ($pos -lt $content[1].Length) {
      if ($content[$index + 1][$pos + 1] -match '[0-9]') {
        $numbers += (Get-Number -line $content[$index + 1] -pos ($pos + 1) -index ($index + 1))
      }
    }
    if ($pos -gt 1) {
      if ($content[$index + 1][$pos - 1] -match '[0-9]') {
        $numbers += (Get-Number -line $content[$index + 1] -pos ($pos - 1) -index ($index + 1))
      }
    }
  }
  return $numbers
}

function Find-Ratio() {
  param ($content, $index, $pos)

  $numbers = @()

  $numbers = Find-Adjacent -content $content -index $index -pos $pos -numbers $numbers

  $numbers = $numbers | Sort-Object -Property Line, Range -Unique

  $numberList = @()
  if ($numbers.Length -eq 2) {
    foreach ($number in $numbers) {
      $numberList += [int]$number.Value
    }
    $Command = $numberList -join "*";
    return Invoke-Expression -Command $Command;
  }
  return 0
  
  
}
function Invoke-PartOne() {
  param ($content)
  $total = 0
  $index = 0
  $numbers = @()
  foreach ($line in $content) {
    for ($pos = 0; $pos -lt $line.Length; $pos++) {
      if (($line[$pos] -match '[0-9]|\.')) {
        continue
      }     
      $numbers = Find-Adjacent -content $content -index $index -pos $pos -numbers $numbers
    }
    $index++
  }
  $numbers = $numbers | Sort-Object -Property Line, Range -Unique
  foreach ($number in $numbers) {
    $total += [int]$number.Value
  }

  Write-Host $total
}

function Invoke-PartTwo() {
  param ($content)

  $total = 0
  $index = 0
  foreach ($line in $content) {
    for ($pos = 0; $pos -lt $line.Length; $pos++) {
      if (-not ($line[$pos] -match '\*')) {
        continue
      }     
      $total += Find-Ratio -content $content -index $index -pos $pos 
    }
    $index++
  }

  Write-Host $total
}


$content = Get-Content .\input.txt
Invoke-PartOne $content
Invoke-PartTwo $content