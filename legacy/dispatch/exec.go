package dispatch

import (
	"fmt"
	"io"
	"os"
	"os/exec"
	"strings"
)

// NotImplemented throws a "Not Implemented" error.
func NotImplemented() (err error) {
	return fmt.Errorf("Feature not implemented")
}

// PrintCommand prints the command to be executed.
func PrintCommand(cmd []string, dry bool) {
	var prompt string
	if dry {
		prompt = "#>"
	} else {
		prompt = ">>"
	}
	fmt.Printf("%s %s\n", prompt, strings.Join(cmd, " "))
}

// RunCommand and get the error.
func RunCommand(cmd []string) (err error) {
	var errBuf strings.Builder
	PrintCommand(cmd, false)
	p := exec.Command(cmd[0], cmd[1:]...)
	p.Stdin = os.Stdin
	p.Stdout = os.Stdout
	p.Stderr = io.MultiWriter(os.Stderr, &errBuf)

	if runErr := p.Run(); runErr != nil {
		return fmt.Errorf("%s", errBuf.String())
	}
	return nil
}