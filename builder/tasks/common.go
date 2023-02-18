package tasks

import (
	"fmt"
	"os"
	"path"

	"github.com/fatih/color"
)

type Logger struct {
	Prefix      string
	PrefixColor *color.Color
	TextColor   *color.Color
}

func NewLogger(prefix string, prefixColor color.Color, textColor color.Color) Logger {
	return Logger{
		Prefix:      prefix,
		PrefixColor: &prefixColor,
		TextColor:   &textColor,
	}
}

func (logger *Logger) Log(format string, args ...any) {
	msg := fmt.Sprintf(format, args...)

	fmt.Printf("%s : %s\n", logger.PrefixColor.Sprintf("%s", logger.Prefix), msg)
}

func ResolveRoot(target string) string {
	root, _ := os.Getwd()
	return path.Join(root, target)
}
