
function Convert-Number {
  param ([Parameter(
      ValueFromPipeline = $true)]
    $value)
  return $value `
    -replace "one", 'o1e' `
    -replace "two", 't2o' `
    -replace "three", 't3e' `
    -replace "four", '4' `
    -replace "five", '5e' `
    -replace "six", '6' `
    -replace "seven", '7n' `
    -replace "eight", 'e8t' `
    -replace "nine", 'n9e' 
}
function Get-FirstLast() {
  param ([Parameter(
      ValueFromPipeline = $true)]
    $value)
  return $value.Substring(0, 1) + $value.Substring($value.Length - 1)
}

function Remove-Alpha {
  param ([Parameter(
      ValueFromPipeline = $true)]
    $value) 
  return $value -replace "[A-Z]", '' 
}

function Invoke-PartOne() {
  param ($content) 
  $total = 0
  foreach ($line in $content) {
    $total += [int](Remove-Alpha $line | Get-FirstLast)
  }

  Write-Host $total
}

function Invoke-PartTwo() {
  param ($content) 
  $total = 0
  foreach ($line in $content) {
    $total += [int](Convert-Number $line | Remove-Alpha | Get-FirstLast)
  }

  Write-Host $total
}

$content = Get-Content .\input.txt
Invoke-PartOne $content
Invoke-PartTwo $content