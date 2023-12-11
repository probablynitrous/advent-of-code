function Get-Next {
  param (
    $history
  )

  $history = $history.split(' ').Where({ '' -ne $_ })

  $diffArr = Invoke-loop $history

  $diffArr = $diffArr , $history
 
  $currValue = 0
  $diffArr | ForEach-Object {
    Write-Host $_
  }
  for ($i = $diffArr.Count; $i -gt 0; $i--) {
    Write-Host 'Len' $diffArr[$i].count 
    $currValue = $currValue + $diffArr[$i - 1][$diffArr[$i - 1].count - 1]
  }

  Write-Host $currValue
}

function Invoke-loop {
  param (
    $line,
    $diffArr
  )
  
  $diff = @()

  for ($i = 1; $i -lt $line.Count; $i ++) {
    $diff += ($line[$i] - $line[$i - 1])
  }

  $diffArr = $diffArr, $diff
  Write-Host ($diffArr -join "-")
  if ($diff -match '[1-9]') {
    $diffArr = $diffArr, (Invoke-loop $diff -diffArr $diffArr)
  } 

  return $diffArr
}
function Invoke-PartOne {
  param (
    $content
  )
  Get-Next $content[0]
}

function Invoke-PartTwo {
  param (
    $content
  )
  
}


$content = Get-Content .\test.txt 
Invoke-PartOne $content
Invoke-PartTwo $content