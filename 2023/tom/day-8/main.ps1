function Get-Nodes() {
  param (
    $content
  )
  $maplines = $content[2..($content.Length -1)]
  $nodes = [ordered]@{}
  foreach ($line in $maplines) {
    $nodes.Add(([string]$line[0..2] -replace ' ',''),@(([string]$line[7..9] -replace ' ',''),([string]$line[12..14] -replace' ','')))
  }
  return $nodes
}
function Get-LCM() {
  param (
    $a,
    $b
  )

  if ($a -gt $b) {
    $low, $high = $b,$a
  } else {
    $low, $high = $a,$b
  }

  while ($true) {
    $remainder = $high % $low
    if ($remainder -eq 0) {
      return ($a*$b) / $low
    }

    $high = $low
    $low = $remainder
  }
}
function Invoke-PartOne() {
  param (
    $content
  )
  $directions = ($content[0] -replace "R", 1 -replace "L",0 -split '').Where({ '' -ne $_ })
  $nodes = Get-Nodes $content
  $currNode = 'AAA'
  $steps = 0

  while ($currNode -ne 'ZZZ') {
    $dir = $directions[$steps%$directions.Count]
    $currNode = $nodes.$currNode[$dir]
    $steps++
  }

  Write-Host $steps
}

function Invoke-PartTwo() {
  param (
    $content
  )
  $directions = ($content[0] -replace "R", 1 -replace "L",0 -split '').Where({ '' -ne $_ })
  $nodes = Get-Nodes $content

  $lcm = 1
  foreach ($key in $nodes.keys.Where({$_[2] -eq 'A'})) {
    $currNode = $key
    $steps = 0
    while ($currNode[2] -ne 'Z') {   
      $dir = $directions[$steps%$directions.Count]
      $currNode = $nodes.$currNode[$dir]
      $steps++
    }
    $lcm = Get-LCM -a $steps -b $lcm
  }

  Write-Host $lcm
}

$timer = [Diagnostics.Stopwatch]::StartNew()
$content = Get-Content .\input.txt 
Invoke-PartOne $content
Invoke-PartTwo $content
Write-Host $timer.elapsed.totalseconds