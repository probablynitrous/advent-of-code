package main

import (
	"bytes"
	"flag"
	"fmt"
	"log"
	"os/exec"
	"time"
)

func main() {
	langArg := flag.String("lang", "", "")
	dayArg := flag.String("day", "", "")
	testArg := flag.Bool("test", false, "")
	flag.Parse()
	fmt.Println("Language:", string(*langArg))
	fmt.Println("Day:", string(*dayArg))
	
	var day string = string(*dayArg)
	var lang string = string(*langArg)
	var test bool = bool(*testArg)

	var cmd *exec.Cmd
	switch lang {
		case "rust":
			if test {
				cmd = exec.Command("cargo", "run", "--release", "--", "test")
			} else {
				cmd = exec.Command("cargo", "run", "--release")
			}
			cmd.Dir = fmt.Sprintf("day-%s/rust/", day)
		case "go":
			if test {
				cmd = exec.Command("go","run","main.go","-test", "true")
			} else {
				cmd = exec.Command("go","run","main.go")
			}
			cmd.Dir = fmt.Sprintf("day-%s/go/", day)
		case "js","javascript":
			if test {
				cmd = exec.Command("node","main.js","test")
			} else {
				cmd = exec.Command("node","main.js")
			}
			cmd.Dir = fmt.Sprintf("day-%s/js/", day)
		case "py", "python":
			if test {
				cmd = exec.Command("py","main.py","test")
			} else {
				cmd = exec.Command("py","main.py")
			}
			cmd.Dir = fmt.Sprintf("day-%s/python/", day)
	}

	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	fmt.Println("Running:", cmd.Args)
	
	start := time.Now()
	out, err := cmd.Output()
	t := time.Now()
	elapsed := t.Sub(start)

	if err != nil {
		fmt.Println("error")
		fmt.Println(stderr.String())
		log.Fatal(err)
	} else {
		fmt.Println("Solution:")
		fmt.Println(string(out))
		
		fmt.Println("Task ran in", elapsed)
	}
}
