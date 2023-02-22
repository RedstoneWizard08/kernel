package build

import (
	"bytes"
	"context"
	"fmt"
	"strings"

	"github.com/RedstoneWizard08/kernel/builder/tasks"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/api/types/mount"
	"github.com/docker/docker/client"
)

func RunDocker(cmd []string, env []string, cfg tasks.Config, ctx context.Context, cli *client.Client, verbose bool) error {
	resp, err := cli.ContainerCreate(ctx, &container.Config{
		Image:        cfg.Image,
		Cmd:          append([]string{"bash", "-c"}, strings.Join(cmd, " ")),
		Env:          env,
		AttachStdout: verbose,
		AttachStderr: verbose,
		WorkingDir:   "/work",
		Tty:          true,
	}, &container.HostConfig{
		Mounts: []mount.Mount{
			{
				Type:   mount.TypeBind,
				Source: gwd(),
				Target: "/work",
			},
		},
	}, nil, nil, "")

	if err != nil {
		return err
	}

	err = cli.ContainerStart(ctx, resp.ID, types.ContainerStartOptions{})

	if err != nil {
		return err
	}

	cli.ContainerWait(ctx, resp.ID, container.WaitConditionNotRunning)

	out, err := cli.ContainerLogs(ctx, resp.ID, types.ContainerLogsOptions{ShowStdout: true, ShowStderr: true})

	if verbose {
		buf := new(bytes.Buffer)

		buf.ReadFrom(out)

		str := buf.String()

		fmt.Println(str)
	}

	return err
}
