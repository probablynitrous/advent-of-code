package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var prefixes = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

type passport struct {
	byr int
	iyr int
	eyr int
	hgt string
	hcl string
	ecl string
	pid string
}

func validPassport(passport passport) bool {

	// byr
	if len(strconv.Itoa(passport.byr)) != 4 {
		return false
	}

	if (passport.byr < 1920) || (2002 < passport.byr) {
		return false
	}

	// iyr
	if len(strconv.Itoa(passport.iyr)) != 4 {
		return false
	}

	if (passport.iyr < 2010) || (2020 < passport.iyr) {
		return false
	}

	// eyr
	if len(strconv.Itoa(passport.iyr)) != 4 {
		return false
	}

	if (passport.eyr < 2020) || (2030 < passport.eyr) {
		return false
	}

	// hgt
	if !validHeight(passport.hgt) {
		return false
	}

	// hcl
	if !regexp.MustCompile("^#[a-f0-9]{6}$").MatchString(passport.hcl) {
		return false
	}

	// ecl
	if !regexp.MustCompile("^(amb|blu|brn|gry|grn|hzl|oth)$").MatchString(passport.ecl) {
		return false
	}

	// pid
	if !regexp.MustCompile("^[0-9]{9}$").MatchString(passport.pid) {
		return false
	}

	return true
}

func validHeight(heightString string) bool {
	if strings.Contains(heightString, "cm") {
		heightString = strings.TrimSuffix(heightString, "cm")
		height, _ := strconv.Atoi(heightString)
		if (150 <= height) && (height <= 193) {
			return true
		}
	} else if strings.Contains(heightString, "in") {
		heightString = strings.TrimSuffix(heightString, "in")
		height, _ := strconv.Atoi(heightString)
		if (59 <= height) && (height <= 76) {
			return true
		}
	}
	return false
}

func newPassportFromString(s string) (passport, error) {
	fields := make(map[string]string)
	for _, prefix := range prefixes {
		r := regexp.MustCompile(fmt.Sprintf("%v:(\\S+) ", prefix))
		if r.MatchString(s) {
			fields[prefix] = r.FindStringSubmatch(s)[1]
		} else {
			return passport{}, fmt.Errorf("Passport %v missing required field %v", s, prefix)
		}
	}

	byr, _ := strconv.Atoi(fields["byr"])
	iyr, _ := strconv.Atoi(fields["iyr"])
	eyr, _ := strconv.Atoi(fields["eyr"])
	hgt, _ := fields["hgt"]
	hcl, _ := fields["hcl"]
	ecl, _ := fields["ecl"]
	pid, _ := fields["pid"]

	return passport{
		byr: byr,
		iyr: iyr,
		eyr: eyr,
		hgt: hgt,
		hcl: hcl,
		ecl: ecl,
		pid: pid,
	}, nil
}

func main() {
	// Assume we can always open the file
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	passportString := ""

	var passports = []passport{}

	// For each line
	for scanner.Scan() {
		line := scanner.Text()
		passportString = fmt.Sprintf("%v %v", passportString, line)
		if line == "" {
			passport, err := newPassportFromString(passportString)
			if err == nil {
				if validPassport(passport) {
					passports = append(passports, passport)
				}
			}
			passportString = ""
		}
	}

	println("There are", len(passports), "valid passports")
}
