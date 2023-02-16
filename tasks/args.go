package tasks

type LogOptions struct {
	Verbose bool `arg:"-v, --verbose" help:"Show verbose logging messages." default:"false"`
}

type BuildCmd struct {
	Clean bool `arg:"-c, --clean" help:"Whether to clean the build output." default:"false"`
}

type CleanCmd struct{}

type CheckCmd struct {
	Targets string `arg:"-t, --targets" help:"A comma-separated list of the targets to build. [all | kernel | symbols]" default:"all"`
}

type RunCmd struct {
	Clean bool `arg:"-c, --clean" help:"Whether to clean the build output." default:"false"`
}

type DebugCmd struct {
	GDB     bool `arg:"-g, --gdb" help:"Use GDB instead of OpenOCD." default:"true"`
	OpenOCD bool `arg:"-o, --openocd" help:"Use OpenOCD instead of GDB." default:"false"`
	Clean   bool `arg:"-c, --clean" help:"Whether to clean the build output." default:"false"`
}

type TestCmd struct {
	All         bool `arg:"-a, --all" help:"Run all the tests." default:"true"`
	Unit        bool `arg:"-u, --unit" help:"Run the unit tests." default:"true"`
	Boot        bool `arg:"-b, --boot" help:"Run the boot tests." default:"true"`
	Integration bool `arg:"-i, --integration" help:"Run the integration tests." default:"true"`
	Clean       bool `arg:"-c, --clean" help:"Whether to clean the build output." default:"false"`
}

type DocCmd struct{}

type Arguments struct {
	Version bool `arg:"-V, --version" help:"Print the version and exit." default:"false"`

	Build *BuildCmd `arg:"subcommand:build"`
	Clean *CleanCmd `arg:"subcommand:clean"`
	Check *CheckCmd `arg:"subcommand:check"`
	Run   *RunCmd   `arg:"subcommand:run"`
	Debug *DebugCmd `arg:"subcommand:debug"`
	Test  *TestCmd  `arg:"subcommand:test"`

	LogOptions
}

func (Arguments) Description() string {
	return "== A build utility for the DESK kernel. =="
}
