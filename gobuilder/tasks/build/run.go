package build

import (
	"context"

	"github.com/RedstoneWizard08/kernel/builder/tasks"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/client"
)

func Setup() (tasks.Config, context.Context, *client.Client) {
	ctx := context.Background()

	cfg := tasks.DefaultConfig()
	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())

	if err != nil {
		panic(err)
	}

	defer cli.Close()

	return cfg, ctx, cli
}

func Build(args tasks.Arguments) error {
	cfg, ctx, cli := Setup()

	cli.ImagePull(ctx, cfg.Image, types.ImagePullOptions{})

	BuildKernel(cfg, args, ctx, cli)

	return nil
}
