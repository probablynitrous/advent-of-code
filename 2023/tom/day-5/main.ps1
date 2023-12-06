function Get-Conversions() {
  param ($lines)

  $conversions = [ordered]@{}
  foreach ($line in $lines) {
    $line = $line.Split(' ').Where({ '' -ne $_ })
    $conversions.Add(@{
      lower = [long]$line[1]; 
      upper = ([long]$line[1] + [long]$line[2]) - 1 
    }, 
    [long]$line[0] - [long]$line[1]
    )
  }

  return $conversions
}

function Get-Seeds {
  param (
    $numbers
  )
  return $numbers.Substring($numbers.IndexOf(':') + 1).Split(' ').Where({ '' -ne $_ })
}
function Invoke-PartOne() {
  param($content)

  $maps = $content -split '(?:\r?\n){2,}'
  $numbers , $maps = $maps
  $seeds = Get-Seeds $numbers
  foreach ($map in $maps) {
    $lines = $map -split '\r\n'
    $header , $lines = $lines
    $conversions = Get-Conversions $lines
    $NewNumbers = @()
    foreach ($number in $seeds) {
      $set = $false
      foreach ($key in $conversions.keys) {
        if (($key.lower -le $number) -and ($number -le $key.upper)) {
          $NewNumbers += [long]$number + [long]$conversions.$key
          $set = $true
        }
      }
      if (-not $set) {
        $NewNumbers += [long]$number
      }

    }
    $seeds = $NewNumbers
  }

  $minValue = [int]::MaxValue
  $seeds | ForEach-Object { if ($minValue -gt $_) { $minValue = $_ } }
  Write-Host $minValue
}
function Invoke-PartTwo() {
  param($content)

  $maps = $content -split '(?:\r?\n){2,}'
  $numbers , $maps = $maps
  $seeds = Get-Seeds $numbers 
  
  $seedRanges = @()
  for ($i = 0; $i -lt $seeds.count) {
    $seedRanges += @{
      lower = [long]$seeds[$i];
      upper = ([long]$seeds[$i] + [long]$seeds[$i + 1])
    }

    $i += 2
  }  

  foreach ($map in $maps) {
    $lines = $map -split '\r\n'
    $header , $lines = $lines    
    $conversions = Get-Conversions $lines
    $NewNumbers = @()

    foreach ($seedRange in $seedRanges) {
      $set = $false
      foreach ($key in $conversions.keys) {
        if (($seedRange.lower -ge $key.lower ) -and ($seedRange.upper -le $key.upper)) {
          $NewNumbers += @{
            lower = ([long]$seedRange.lower + [long]$conversions.$key); 
            upper = ([long]$seedRange.upper + [long]$conversions.$key)
          } 
          $set = $true
        }
        if (($seedRange.lower -gt $key.lower) -and ($seedRange.upper -gt $key.upper) -and ($seedRange.lower -le $key.upper) ){
          $NewNumbers += @{
            lower = ([long]$seedRange.lower + [long]$conversions.$key); 
            upper = ([long]$key.upper + [long]$conversions.$key)
          } 
          $set = $true
        }

        if (($seedRange.lower -lt $key.lower) -and ($seedRange.upper -lt $key.upper) -and ($seedRange.upper -ge $key.lower)) {
          $NewNumbers += @{
            lower = ([long]$key.lower + [long]$conversions.$key); 
            upper = ([long]$seedRange.upper + [long]$conversions.$key)
          }
          $set = $true
        }  
      }
      if (-not $set) {
        $NewNumbers += $seedRange
      }
      
    }
    $seedRanges = $NewNumbers
   
  }

  $minValue = [int]::MaxValue
  $seedRanges | ForEach-Object { 
    if ($minValue -gt $_.lower) 
    { 
      $minValue = $_.lower 
    } 
  }
  Write-Host $minValue
}

$content = Get-Content .\input.txt -raw 
Invoke-PartOne $content
Invoke-PartTwo $content