function Get-Winners() {
  param($line)

  $Start = $line.IndexOf(':')
  $CardValues = $line.Substring($Start + 1).Split('|');

  $Winners = $CardValues[0].Split(' ').Where({ '' -ne $_ })
  $Game = $CardValues[1].Split(' ').Where({ '' -ne $_ })

  $counter = 0
  foreach($Winner in $Winners) {
    if ($Game.Contains($Winner)) {
      $counter += 1
    }
  }

  return $counter
}

function Invoke-PartOne() {
  param($content)

  $total = 0;

  foreach ($line in $content) {
    $counter = Get-Winners $line
    if ($counter -eq 0) {
      $total += 0
    } else {
      $total +=[Math]::Pow(2,($counter - 1))
    }
     
  }

  Write-Host $total
}
function Invoke-PartTwo() {
  param($content)
  $Cards = New-Object int[] $content.Length
  $i = 0
  foreach ($line in $content) {
    $Cards[$i] += 1
    $Winners = Get-Winners $line
    for($j = $i + 1; $j -lt ($i + $Winners) + 1; $j++) {
      $Cards[$j] += $Cards[$i]
    }

    $i++
  }

  Write-Host ($Cards | measure-object -sum).Sum
}

$content = Get-Content .\input.txt
Invoke-PartOne $content
Invoke-PartTwo $content